use std::{
  collections::HashMap,
  sync::{Arc, Mutex, OnceLock},
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::{
  sync::mpsc::{self, Sender},
  task,
};
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
pub struct AudioDeviceInfo {
  pub name: String,
  pub volume: f32,
  pub is_default: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioOutput {
  pub devices: HashMap<String, AudioDeviceInfo>,
  pub default_device: Option<String>,
}

impl AudioOutput {
  fn new() -> Self {
    Self {
      devices: HashMap::new(),
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
}

impl MediaDeviceEventHandler {
  fn new(enumerator: IMMDeviceEnumerator) -> Self {
    Self {
      enumerator,
      device_state: Arc::new(Mutex::new(HashMap::new())),
      current_device: String::new(),
    }
  }

  fn get_device_name(device: &IMMDevice) -> windows::core::Result<String> {
    unsafe {
      let store: IPropertyStore = device.OpenPropertyStore(STGM_READ)?;
      let value = store.GetValue(&PKEY_Device_FriendlyName)?;
      Ok(value.to_string())
    }
  }

  fn enumerate_devices(&self) -> windows::core::Result<()> {
    unsafe {
      let collection = self
        .enumerator
        .EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)?;
      let mut audio_state = AUDIO_STATE.get().unwrap().lock().unwrap();
      let mut devices = HashMap::new();
      let mut device_state = self.device_state.lock().unwrap();

      for i in 0..collection.GetCount()? {
        if let Ok(device) = collection.Item(i) {
          let device_id = device.GetId()?.to_string()?;

          // Check if the device is already being monitored
          if !device_state.contains_key(&device_id) {
            let device_name = Self::get_device_name(&device)?;
            let endpoint_volume: IAudioEndpointVolume =
              device.Activate(CLSCTX_ALL, None)?;
            let mut handler = self.clone();
            handler.current_device = device_id.clone();
            let callback = IAudioEndpointVolumeCallback::from(handler);
            endpoint_volume.RegisterControlChangeNotify(&callback)?;
            device_state.insert(
              device_id.clone(),
              DeviceInfo {
                name: device_name.clone(),
                endpoint_volume,
              },
            );
          }

          let device_info = device_state.get(&device_id).unwrap();
          let volume = device_info
            .endpoint_volume
            .GetMasterVolumeLevelScalar()
            .unwrap_or(0.0);
          let is_default = self
            .enumerator
            .GetDefaultAudioEndpoint(eRender, eMultimedia)
            .ok()
            .and_then(|d| d.GetId().ok())
            .and_then(|id| id.to_string().ok())
            .as_ref()
            .map(|id| id == &device_id)
            .unwrap_or(false);

          devices.insert(
            device_id,
            AudioDeviceInfo {
              name: device_info.name.clone(),
              volume,
              is_default,
            },
          );
        }
      }

      audio_state.devices = devices;
      audio_state.default_device = self
        .enumerator
        .GetDefaultAudioEndpoint(eRender, eMultimedia)
        .ok()
        .and_then(|d| d.GetId().ok())
        .and_then(|id| id.to_string().ok());
    }

    AudioProvider::emit_volume();
    Ok(())
  }
}

impl Drop for MediaDeviceEventHandler {
  fn drop(&mut self) {
    unsafe {
      let mut device_state = self.device_state.lock().unwrap();
      for (device_id, device_info) in device_state.iter() {
        device_info
          .endpoint_volume
          .UnregisterControlChangeNotify(
            &IAudioEndpointVolumeCallback::from(self.clone()),
          )
          .expect("Failed to unregister volume callback");
      }
    }
  }
}

impl IAudioEndpointVolumeCallback_Impl for MediaDeviceEventHandler_Impl {
  fn OnNotify(
    &self,
    data: *mut windows::Win32::Media::Audio::AUDIO_VOLUME_NOTIFICATION_DATA,
  ) -> windows::core::Result<()> {
    unsafe {
      if let Some(data) = data.as_ref() {
        let device_id = &*self.current_device;
        println!("Got notification for device: {}", device_id);

        if let Some(state) = AUDIO_STATE.get() {
          let mut output = state.lock().unwrap();
          if let Some(device) = output.devices.get_mut(device_id) {
            device.volume = data.fMasterVolume;
            println!(
              "Volume update for {} (ID: {}): {}",
              device.name, device_id, data.fMasterVolume
            );
            AudioProvider::emit_volume();
          }
        }
      }
      Ok(())
    }
  }
}

impl IMMNotificationClient_Impl for MediaDeviceEventHandler_Impl {
  fn OnDefaultDeviceChanged(
    &self,
    flow: EDataFlow,
    role: ERole,
    _pwstrDefaultDeviceId: &PCWSTR,
  ) -> windows::core::Result<()> {
    if flow == eRender && role == eMultimedia {
      self.enumerate_devices()?;
    }
    Ok(())
  }

  fn OnDeviceStateChanged(
    &self,
    _pwstrDeviceId: &PCWSTR,
    _dwNewState: DEVICE_STATE,
  ) -> windows::core::Result<()> {
    self.enumerate_devices()
  }

  fn OnDeviceAdded(
    &self,
    _pwstrDeviceId: &PCWSTR,
  ) -> windows::core::Result<()> {
    self.enumerate_devices()
  }

  fn OnDeviceRemoved(
    &self,
    _pwstrDeviceId: &PCWSTR,
  ) -> windows::core::Result<()> {
    self.enumerate_devices()
  }

  fn OnPropertyValueChanged(
    &self,
    _pwstrDeviceId: &PCWSTR,
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
      println!("Emitting audio output: {:#?}", output);
      let _ = tx.try_send(Ok(ProviderOutput::Audio(output)).into());
    }
  }

  fn create_audio_manager() -> anyhow::Result<()> {
    unsafe {
      let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
      let enumerator: IMMDeviceEnumerator =
        CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
      let handler = MediaDeviceEventHandler::new(enumerator.clone());

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

    task::spawn_blocking(move || {
      if let Err(err) = Self::create_audio_manager() {
        emit_result_tx
          .blocking_send(Err(err).into())
          .expect("Error with media provider");
      }
    });
  }
}
