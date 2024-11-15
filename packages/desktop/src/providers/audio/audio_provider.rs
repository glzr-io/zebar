use std::sync::{Arc, Mutex, OnceLock};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{self, Sender};
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
  },
  System::Com::{
    CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED,
    STGM_READ,
  },
  UI::Shell::PropertiesSystem::IPropertyStore,
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
pub struct AudioOutput {
  pub current_device: String,
  pub volume: f32,
}

pub struct AudioProvider {
  _config: AudioProviderConfig,
}

#[async_trait]
impl Provider for AudioProvider {
  async fn run(&self, emit_result_tx: Sender<ProviderResult>) {
    if let Err(err) = self.create_audio_manager(emit_result_tx.clone()) {
      emit_result_tx
        .send(Err(err).into())
        .await
        .expect("Error emitting media provider err.");
    }
  }
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

  fn create_audio_manager(
    &self,
    emit_result_tx: Sender<ProviderResult>,
  ) -> anyhow::Result<()> {
    PROVIDER_TX
      .set(emit_result_tx.clone())
      .expect("Error setting provider tx in focused window provider");

    // todo do this at initialization
    AUDIO_STATE
      .set(Arc::new(Mutex::new(AudioOutput {
        current_device: "n/a".to_string(),
        volume: 0.0,
      })))
      .expect("Error setting initial state");

    unsafe {
      let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

      let enumerator: IMMDeviceEnumerator =
        CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;

      let handler = MediaDeviceEventHandler::new(enumerator.clone());
      let device_notification_callback =
        IMMNotificationClient::from(handler.clone());

      let default_device =
        enumerator.GetDefaultAudioEndpoint(eRender, eMultimedia)?;
      if let Ok(name) =
        MediaDeviceEventHandler::get_device_name(&default_device)
      {
        println!("Default audio render device: {}", name);
      }

      // Set up initial volume monitoring
      handler.setup_volume_monitoring(&default_device)?;
      enumerator.RegisterEndpointNotificationCallback(
        &device_notification_callback,
      )?;

      loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
      }
    }
  }
}

struct DeviceState {
  volume_callbacks:
    Arc<Mutex<Vec<(IAudioEndpointVolumeCallback, IAudioEndpointVolume)>>>,
}

#[derive(Clone)]
#[windows::core::implement(
  IMMNotificationClient,
  IAudioEndpointVolumeCallback
)]
struct MediaDeviceEventHandler {
  enumerator: IMMDeviceEnumerator,
  device_state: Arc<DeviceState>,
}

impl MediaDeviceEventHandler {
  fn new(enumerator: IMMDeviceEnumerator) -> Self {
    Self {
      enumerator,
      device_state: Arc::new(DeviceState {
        volume_callbacks: Arc::new(Mutex::new(Vec::new())),
      }),
    }
  }

  fn get_device_name(device: &IMMDevice) -> windows_core::Result<String> {
    unsafe {
      let store: IPropertyStore = device.OpenPropertyStore(STGM_READ)?;
      let value = store.GetValue(&PKEY_Device_FriendlyName)?;
      Ok(value.to_string())
    }
  }

  fn setup_volume_monitoring(
    &self,
    device: &IMMDevice,
  ) -> windows_core::Result<()> {
    unsafe {
      let endpoint_volume: IAudioEndpointVolume =
        device.Activate(CLSCTX_ALL, None)?;
      let handler = MediaDeviceEventHandler::new(self.enumerator.clone());
      let volume_callback = IAudioEndpointVolumeCallback::from(handler);
      endpoint_volume.RegisterControlChangeNotify(&volume_callback)?;
      self
        .device_state
        .volume_callbacks
        .lock()
        .unwrap()
        .push((volume_callback, endpoint_volume));
    }
    Ok(())
  }
}

impl IAudioEndpointVolumeCallback_Impl for MediaDeviceEventHandler_Impl {
  fn OnNotify(
    &self,
    data: *mut windows::Win32::Media::Audio::AUDIO_VOLUME_NOTIFICATION_DATA,
  ) -> windows_core::Result<()> {
    if let Some(data) = unsafe { data.as_ref() } {
      if let Some(state) = AUDIO_STATE.get() {
        if let Ok(mut output) = state.lock() {
          output.volume = data.fMasterVolume;
          println!("Volume update: {}", data.fMasterVolume);
        }
        AudioProvider::emit_volume();
      }
    }
    Ok(())
  }
}

impl IMMNotificationClient_Impl for MediaDeviceEventHandler_Impl {
  fn OnDeviceStateChanged(
    &self,
    pwstrDeviceId: &PCWSTR,
    dwNewState: DEVICE_STATE,
  ) -> windows_core::Result<()> {
    let device_id = unsafe { pwstrDeviceId.to_string()? };
    println!(
      "Device state changed: {} - State: {:?}",
      device_id, dwNewState
    );
    Ok(())
  }

  fn OnDeviceAdded(
    &self,
    pwstrDeviceId: &PCWSTR,
  ) -> windows_core::Result<()> {
    let device_id = unsafe { pwstrDeviceId.to_string()? };
    println!("Device added: {}", device_id);
    Ok(())
  }

  fn OnDeviceRemoved(
    &self,
    pwstrDeviceId: &PCWSTR,
  ) -> windows_core::Result<()> {
    let device_id = unsafe { pwstrDeviceId.to_string()? };
    println!("Device removed: {}", device_id);
    Ok(())
  }

  fn OnDefaultDeviceChanged(
    &self,
    flow: EDataFlow,
    role: ERole,
    pwstrDefaultDeviceId: &PCWSTR,
  ) -> windows_core::Result<()> {
    unsafe {
      if flow == eRender && role == eMultimedia {
        let device = self.enumerator.GetDevice(*pwstrDefaultDeviceId)?;
        if let Ok(name) = MediaDeviceEventHandler::get_device_name(&device)
        {
          println!("Default device changed to: {}", name);
          self.setup_volume_monitoring(&device)?;
          if let Ok(mut output) = AUDIO_STATE.get().unwrap().lock() {
            output.current_device = name;
          }
          AudioProvider::emit_volume();
        }
      }
    }
    Ok(())
  }

  fn OnPropertyValueChanged(
    &self,
    pwstrDeviceId: &PCWSTR,
    key: &windows::Win32::UI::Shell::PropertiesSystem::PROPERTYKEY,
  ) -> windows_core::Result<()> {
    let device_id = unsafe { pwstrDeviceId.to_string()? };
    println!("Property changed: {} - Key: {:?}", device_id, key);
    Ok(())
  }
}
