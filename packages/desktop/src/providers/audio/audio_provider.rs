use std::{
  collections::HashMap,
  ops::Mul,
  sync::{Arc, Mutex, OnceLock},
  time::Duration,
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::{
  sync::mpsc::{self, Sender},
  task,
  time::sleep,
};
use tracing::debug;
use windows::Win32::{
  Devices::FunctionDiscovery::PKEY_Device_FriendlyName,
  Media::Audio::{
    eMultimedia, eRender, EDataFlow, ERole,
    Endpoints::{
      IAudioEndpointVolume, IAudioEndpointVolumeCallback,
      IAudioEndpointVolumeCallback_Impl,
    },
    IMMDevice, IMMDeviceEnumerator, IMMNotificationClient,
    IMMNotificationClient_Impl, MMDeviceEnumerator, DEVICE_STATE,
    DEVICE_STATE_ACTIVE,
  },
  System::Com::{
    CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED,
    STGM_READ,
  },
  UI::Shell::PropertiesSystem::{IPropertyStore, PROPERTYKEY},
};
use windows_core::PCWSTR;

use crate::providers::{Provider, ProviderOutput, ProviderResult};

static PROVIDER_TX: OnceLock<mpsc::Sender<ProviderResult>> =
  OnceLock::new();

static AUDIO_STATE: OnceLock<Arc<Mutex<AudioOutput>>> = OnceLock::new();

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AudioProviderConfig {}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioDevice {
  pub name: String,
  pub device_id: String,
  pub volume: u32,
  pub is_default: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioOutput {
  pub devices: Vec<AudioDevice>,
  pub default_device: Option<AudioDevice>,
}

impl AudioOutput {
  fn new() -> Self {
    Self {
      devices: Vec::new(),
      default_device: None,
    }
  }
}

#[derive(Clone)]
struct DeviceInfo {
  name: String,
  endpoint_volume: IAudioEndpointVolume,
}

#[derive(Clone)]
#[windows::core::implement(
  IMMNotificationClient,
  IAudioEndpointVolumeCallback
)]
struct MediaDeviceEventHandler {
  enumerator: IMMDeviceEnumerator,
  device_state: Arc<Mutex<HashMap<String, DeviceInfo>>>,
  current_device: String,
  update_tx: mpsc::Sender<(String, u32)>,
}

impl MediaDeviceEventHandler {
  fn new(
    enumerator: IMMDeviceEnumerator,
    update_tx: mpsc::Sender<(String, u32)>,
  ) -> Self {
    Self {
      enumerator,
      device_state: Arc::new(Mutex::new(HashMap::new())),
      current_device: String::new(),
      update_tx,
    }
  }

  fn get_device_name(device: &IMMDevice) -> windows::core::Result<String> {
    unsafe {
      let store: IPropertyStore = device.OpenPropertyStore(STGM_READ)?;
      let value = store.GetValue(&PKEY_Device_FriendlyName)?;
      Ok(value.to_string())
    }
  }

  fn get_device_info(
    &self,
    device: &IMMDevice,
  ) -> windows::core::Result<AudioDevice> {
    unsafe {
      let device_id = device.GetId()?.to_string()?;
      let mut device_state = self.device_state.lock().unwrap();

      if !device_state.contains_key(&device_id) {
        let new_device = self.register_new_device(device)?;
        device_state.insert(device_id.clone(), new_device);
      }

      let device_info = device_state.get(&device_id).unwrap();
      let is_default = self.is_default_device(&device_id)?;
      let volume = device_info
        .endpoint_volume
        .GetMasterVolumeLevelScalar()
        .unwrap_or(0.0)
        .mul(100.0)
        .round() as u32;

      Ok(AudioDevice {
        device_id,
        name: device_info.name.clone(),
        volume,
        is_default,
      })
    }
  }

  fn register_new_device(
    &self,
    device: &IMMDevice,
  ) -> windows::core::Result<DeviceInfo> {
    unsafe {
      let device_name = Self::get_device_name(device)?;
      let endpoint_volume: IAudioEndpointVolume =
        device.Activate(CLSCTX_ALL, None)?;

      let mut handler = self.clone();
      handler.current_device = device.GetId()?.to_string()?;
      endpoint_volume.RegisterControlChangeNotify(
        &IAudioEndpointVolumeCallback::from(handler),
      )?;

      Ok(DeviceInfo {
        name: device_name,
        endpoint_volume,
      })
    }
  }

  fn is_default_device(
    &self,
    device_id: &str,
  ) -> windows::core::Result<bool> {
    unsafe {
      let default = self
        .enumerator
        .GetDefaultAudioEndpoint(eRender, eMultimedia)?;
      let default_id = default.GetId()?.to_string()?;
      Ok(default_id == device_id)
    }
  }

  fn enumerate_devices(&self) -> windows::core::Result<()> {
    unsafe {
      let collection = self
        .enumerator
        .EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)?;

      let mut devices = Vec::new();
      let mut default_device = None;

      // Get info for all active devices.
      for i in 0..collection.GetCount()? {
        if let Ok(device) = collection.Item(i) {
          let info = self.get_device_info(&device)?;
          if info.is_default {
            default_device = Some(info.clone());
          }
          devices.push(info);
        }
      }

      if let Some(state) = AUDIO_STATE.get() {
        let mut audio_state = state.lock().unwrap();
        audio_state.devices = devices;
        audio_state.default_device = default_device;
      }

      AudioProvider::emit_volume();
      Ok(())
    }
  }
}

impl Drop for MediaDeviceEventHandler {
  fn drop(&mut self) {
    unsafe {
      let device_state = self.device_state.lock().unwrap();
      for (_, device_info) in device_state.iter() {
        let _ = device_info.endpoint_volume.UnregisterControlChangeNotify(
          &IAudioEndpointVolumeCallback::from(self.clone()),
        );
      }
    }
  }
}

impl IAudioEndpointVolumeCallback_Impl for MediaDeviceEventHandler_Impl {
  fn OnNotify(
    &self,
    data: *mut windows::Win32::Media::Audio::AUDIO_VOLUME_NOTIFICATION_DATA,
  ) -> windows::core::Result<()> {
    if let Some(data) = unsafe { data.as_ref() } {
      let device_id = self.current_device.clone();
      let volume = data.fMasterVolume.mul(100.0).round() as u32;

      let _ = self.update_tx.blocking_send((device_id, volume));
    }
    Ok(())
  }
}

impl IMMNotificationClient_Impl for MediaDeviceEventHandler_Impl {
  fn OnDefaultDeviceChanged(
    &self,
    data_flow: EDataFlow,
    role: ERole,
    _default_device_id: &PCWSTR,
  ) -> windows::core::Result<()> {
    if data_flow == eRender && role == eMultimedia {
      self.enumerate_devices()?;
    }
    Ok(())
  }

  fn OnDeviceStateChanged(
    &self,
    _device_id: &PCWSTR,
    _new_state: DEVICE_STATE,
  ) -> windows::core::Result<()> {
    self.enumerate_devices()
  }

  fn OnDeviceAdded(
    &self,
    _device_id: &PCWSTR,
  ) -> windows::core::Result<()> {
    self.enumerate_devices()
  }

  fn OnDeviceRemoved(
    &self,
    _device_id: &PCWSTR,
  ) -> windows::core::Result<()> {
    self.enumerate_devices()
  }

  fn OnPropertyValueChanged(
    &self,
    _device_id: &PCWSTR,
    _key: &PROPERTYKEY,
  ) -> windows::core::Result<()> {
    Ok(())
  }
}

pub struct AudioProvider {
  _config: AudioProviderConfig,
}

impl AudioProvider {
  pub fn new(config: AudioProviderConfig) -> Self {
    Self { _config: config }
  }

  fn emit_volume() {
    if let Some(tx) = PROVIDER_TX.get() {
      let output = AUDIO_STATE.get().unwrap().lock().unwrap().clone();
      let _ = tx.try_send(Ok(ProviderOutput::Audio(output)).into());
    }
  }

  async fn handle_volume_updates(mut rx: mpsc::Receiver<(String, u32)>) {
    const PROCESS_DELAY: Duration = Duration::from_millis(50);
    let mut latest_updates = HashMap::new();

    while let Some((device_id, volume)) = rx.recv().await {
      latest_updates.insert(device_id, volume);

      // Collect any additional pending updates without waiting.
      while let Ok((device_id, volume)) = rx.try_recv() {
        latest_updates.insert(device_id, volume);
      }

      // Brief delay to collect more potential updates.
      sleep(PROCESS_DELAY).await;

      // Process all collected updates.
      if let Some(state) = AUDIO_STATE.get() {
        {
          let mut output = state.lock().unwrap();
          for (device_id, volume) in latest_updates.drain() {
            debug!(
              "Updating volume to {} for device: {}",
              volume, device_id
            );

            // Update device in the devices list.
            if let Some(device) =
              output.devices.iter_mut().find(|d| d.device_id == device_id)
            {
              device.volume = volume;
            }

            // Update default device if it matches.
            if let Some(default_device) = &mut output.default_device {
              if default_device.device_id == device_id {
                default_device.volume = volume;
              }
            }
          }
        }

        Self::emit_volume();
      }
    }
  }

  fn create_audio_manager(
    update_tx: mpsc::Sender<(String, u32)>,
  ) -> anyhow::Result<()> {
    unsafe {
      let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
      let enumerator: IMMDeviceEnumerator =
        CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
      let handler =
        MediaDeviceEventHandler::new(enumerator.clone(), update_tx);

      handler.enumerate_devices()?;

      let device_notification_callback =
        IMMNotificationClient::from(handler.clone());
      enumerator.RegisterEndpointNotificationCallback(
        &device_notification_callback,
      )?;

      loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
      }
    }
  }
}

#[async_trait]
impl Provider for AudioProvider {
  async fn run(&self, emit_result_tx: Sender<ProviderResult>) {
    PROVIDER_TX
      .set(emit_result_tx.clone())
      .expect("Error setting provider tx in audio provider");

    AUDIO_STATE
      .set(Arc::new(Mutex::new(AudioOutput::new())))
      .expect("Error setting initial audio state");

    // Create a channel for volume updates.
    let (update_tx, update_rx) = mpsc::channel(100);

    // Spawn both tasks.
    let update_handler =
      task::spawn(Self::handle_volume_updates(update_rx));

    let manager = task::spawn_blocking(move || {
      if let Err(err) = Self::create_audio_manager(update_tx) {
        emit_result_tx
          .blocking_send(Err(err).into())
          .expect("Error with media provider");
      }
    });

    // Wait for either task to complete (though they should run forever).
    tokio::select! {
      _ = manager => debug!("Audio manager stopped unexpectedly"),
      _ = update_handler => debug!("Update handler stopped unexpectedly"),
    }
  }
}
