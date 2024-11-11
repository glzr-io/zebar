use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AudioProviderConfig {}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioOutput {
  pub device: String,
  pub volume: u8,
}

pub struct AudioProvider {
  _config: AudioProviderConfig,
}

impl AudioProvider {
  pub fn new(config: AudioProviderConfig) -> Self {
    Self { _config: config }
  }

  fn create_audio_manager(
    &self,
    emit_result_tx: Sender<ProviderResult>,
  ) -> anyhow::Result<()> {
    unsafe {
      // Initialize COM library
      let _ =
        CoInitializeEx(Some(std::ptr::null_mut()), COINIT_MULTITHREADED);

      // Get the audio endpoint volume interface
      let enumerator: IMMDeviceEnumerator =
        CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
      let default_device =
        enumerator.GetDefaultAudioEndpoint(eRender, eMultimedia)?;
      let endpoint_volume: IAudioEndpointVolume =
        default_device.Activate(CLSCTX_ALL, None)?;

      let device_id = default_device.GetId()?.to_string()?;
      println!("Default audio render device: {}", device_id.clone());

      // Register the volume change callback
      let device_volume_callback =
        IAudioEndpointVolumeCallback::from(MediaDeviceEventHandler {});

      endpoint_volume
        .RegisterControlChangeNotify(&device_volume_callback)?;
    }

    loop {
      // tx randomized data to test
      let output = AudioOutput {
        device: "default".to_string(),
        volume: 50,
      };
      emit_result_tx.try_send(Ok(ProviderOutput::Audio(output)).into())?;
      std::thread::sleep(std::time::Duration::from_secs(1));
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
      println!("Volume notification: {}", data.fMasterVolume,);
    }
    Ok(())
  }
}

impl IAudioSessionNotification_Impl for MediaDeviceEventHandler_Impl {
  fn OnSessionCreated(
    &self,
    _new_session: Option<&IAudioSessionControl>,
  ) -> windows::core::Result<()> {
    println!("SESSION CREATED!");
    let name = unsafe {
      _new_session
        .unwrap()
        .GetDisplayName()
        .unwrap()
        .to_string()
        .unwrap()
    };
    println!("New session created: {}", name);
    Ok(())
  }
}

// this works for current device changes
// but typedeventhandlers cant be used for volume updates
// -----------------------------------------------------------
// let handler = TypedEventHandler::<IInspectable,
// DefaultAudioRenderDeviceChangedEventArgs>::new(     move |_:
// &Option<IInspectable>, args:
// &Option<DefaultAudioRenderDeviceChangedEventArgs>| {         println!("
// Default audio render device changed");         let device_id =
// args.as_ref().unwrap().Id().unwrap();         let device_info =
// DeviceInformation::CreateFromIdAsync(&device_id)             .unwrap()
//             .get()
//             .unwrap();
//         let device_name = device_info.Name().unwrap();
//         println!("New default audio render device: {}", device_name);
//         Ok(())
//     },
// );
// MediaDevice::DefaultAudioRenderDeviceChanged(&handler)?;
