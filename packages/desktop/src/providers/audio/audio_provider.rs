use std::sync::OnceLock;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{self, Sender};
use windows::Win32::{
  Media::Audio::{
    eMultimedia, eRender,
    Endpoints::{
      IAudioEndpointVolume, IAudioEndpointVolumeCallback,
      IAudioEndpointVolumeCallback_Impl,
    },
    IAudioSessionControl, IAudioSessionNotification,
    IAudioSessionNotification_Impl, IMMDeviceEnumerator,
    MMDeviceEnumerator,
  },
  System::Com::{
    CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED,
  },
};

use crate::providers::{Provider, ProviderOutput, ProviderResult};

static PROVIDER_TX: OnceLock<mpsc::Sender<ProviderResult>> =
  OnceLock::new();

static AUDIO_STATUS: OnceLock<AudioOutput> = OnceLock::new();

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AudioProviderConfig {}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioOutput {
  pub device: String,
  pub volume: f32,
}

pub struct AudioProvider {
  _config: AudioProviderConfig,
}

impl AudioProvider {
  pub fn new(config: AudioProviderConfig) -> Self {
    Self { _config: config }
  }

  fn emit_volume() {
    let tx = PROVIDER_TX.get().expect("Error getting provider tx");
    let output = AUDIO_STATUS.get().expect("Error getting audio status");
    tx.try_send(Ok(ProviderOutput::Audio(output.clone())).into())
      .expect("Error sending audio status");
  }

  fn create_audio_manager(
    &self,
    emit_result_tx: Sender<ProviderResult>,
  ) -> anyhow::Result<()> {
    PROVIDER_TX
      .set(emit_result_tx.clone())
      .expect("Error setting provider tx in focused window provider");

    // TODO is this the best way to initialize this
    let _ = AUDIO_STATUS.set(AudioOutput {
      device: "n/a".to_string(),
      volume: 0.0,
    });

    unsafe {
      // Initialize COM library
      let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

      // Get the audio endpoint volume interface
      let enumerator: IMMDeviceEnumerator =
        CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
      let default_device =
        enumerator.GetDefaultAudioEndpoint(eRender, eMultimedia)?;
      let endpoint_volume: IAudioEndpointVolume =
        default_device.Activate(CLSCTX_ALL, None)?;

      // Register the volume change callback
      let device_volume_callback =
        IAudioEndpointVolumeCallback::from(MediaDeviceEventHandler {});
      endpoint_volume
        .RegisterControlChangeNotify(&device_volume_callback)?;

      loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
      }
    }
  }
}

#[async_trait]
impl Provider for AudioProvider {
  async fn run(&self, emit_result_tx: Sender<ProviderResult>) {
    if let Err(err) = self.create_audio_manager(emit_result_tx.clone()) {
      emit_result_tx
        .send(Err(err).into())
        .await
        .expect("Erroring emitting media provider err.");
    }
  }
}

#[windows::core::implement(
  IAudioEndpointVolumeCallback,
  IAudioSessionNotification
)]
struct MediaDeviceEventHandler {}

impl IAudioEndpointVolumeCallback_Impl for MediaDeviceEventHandler_Impl {
  fn OnNotify(
    &self,
    data: *mut windows::Win32::Media::Audio::AUDIO_VOLUME_NOTIFICATION_DATA,
  ) -> windows_core::Result<()> {
    println!("Volume notification");
    if let Some(data) = unsafe { data.as_ref() } {
      // TODO: surely theres a better way to do this without the clone
      AUDIO_STATUS
        .set(AudioOutput {
          device: AUDIO_STATUS.get().expect("msg").device.clone(),
          volume: data.fMasterVolume,
        })
        .expect("Error setting audio status");
      println!("Volume update: {}", data.fMasterVolume,);
      AudioProvider::emit_volume();
    }
    Ok(())
  }
}

impl IAudioSessionNotification_Impl for MediaDeviceEventHandler_Impl {
  fn OnSessionCreated(
    &self,
    _new_session: Option<&IAudioSessionControl>,
  ) -> windows::core::Result<()> {
    let name = unsafe {
      _new_session
        .unwrap()
        .GetDisplayName()
        .unwrap()
        .to_string()
        .unwrap()
    };
    AUDIO_STATUS
      .set(AudioOutput {
        device: name.clone(),
        volume: AUDIO_STATUS.get().expect("msg").volume,
      })
      .expect("Error setting audio status");
    println!("New session created: {}", name);
    AudioProvider::emit_volume();
    Ok(())
  }
}
