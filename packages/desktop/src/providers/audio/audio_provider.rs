use std::collections::{HashMap, HashSet};

use anyhow::Context;
use crossbeam::channel;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};
use windows::Win32::{
  Devices::FunctionDiscovery::{
    PKEY_Device_DeviceDesc, PKEY_Device_FriendlyName,
  },
  Media::Audio::{
    eAll, eCapture, eMultimedia, eRender, EDataFlow, ERole,
    Endpoints::{
      IAudioEndpointVolume, IAudioEndpointVolumeCallback,
      IAudioEndpointVolumeCallback_Impl,
    },
    IMMDevice, IMMDeviceEnumerator, IMMEndpoint, IMMNotificationClient,
    IMMNotificationClient_Impl, MMDeviceEnumerator,
    AUDIO_VOLUME_NOTIFICATION_DATA, DEVICE_STATE, DEVICE_STATE_ACTIVE,
  },
  System::Com::{
    CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED,
    STGM_READ,
  },
  UI::Shell::PropertiesSystem::{IPropertyStore, PROPERTYKEY},
};
use windows_core::{Interface, HSTRING, PCWSTR};

use crate::{
  common::windows::{ComInit, COM_INIT},
  providers::{CommonProviderState, Provider, RuntimeType},
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AudioProviderConfig {}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioOutput {
  pub playback_devices: Vec<AudioDevice>,
  pub recording_devices: Vec<AudioDevice>,
  pub all_devices: Vec<AudioDevice>,
  pub default_playback_device: Option<AudioDevice>,
  pub default_recording_device: Option<AudioDevice>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioDevice {
  pub name: String,
  pub device_id: String,
  pub device_type: DeviceType,
  pub volume: u32,
  pub is_default: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DeviceType {
  Playback,
  Recording,
}

impl From<EDataFlow> for DeviceType {
  fn from(flow: EDataFlow) -> Self {
    match flow {
      flow if flow == eCapture => Self::Recording,
      _ => Self::Playback,
    }
  }
}

impl From<DeviceType> for EDataFlow {
  fn from(device_type: DeviceType) -> Self {
    match device_type {
      DeviceType::Playback => eRender,
      DeviceType::Recording => eCapture,
    }
  }
}

/// Events that can be emitted from audio state changes.
#[derive(Debug)]
enum AudioEvent {
  DeviceAdded(String),
  DeviceRemoved(String),
  DefaultDeviceChanged(String, DeviceType),
  VolumeChanged(String, f32),
}

/// Holds the state of an audio device.
#[derive(Clone)]
struct DeviceState {
  com_device: IMMDevice,
  com_volume: IAudioEndpointVolume,
  com_volume_callback: IAudioEndpointVolumeCallback,
  output: AudioDevice,
}

pub struct AudioProvider {
  common: CommonProviderState,
  com_enumerator: Option<IMMDeviceEnumerator>,
  default_playback_id: Option<String>,
  default_recording_id: Option<String>,
  device_states: HashMap<String, DeviceState>,
  event_tx: channel::Sender<AudioEvent>,
  event_rx: channel::Receiver<AudioEvent>,
}

impl AudioProvider {
  pub fn new(
    _config: AudioProviderConfig,
    common: CommonProviderState,
  ) -> Self {
    let (event_tx, event_rx) = channel::unbounded();

    Self {
      common,
      com_enumerator: None,
      default_playback_id: None,
      default_recording_id: None,
      device_states: HashMap::new(),
      event_tx,
      event_rx,
    }
  }

  /// Main entry point.
  fn start(&mut self) -> anyhow::Result<()> {
    COM_INIT.with(|_| {
      let com_enumerator: IMMDeviceEnumerator = unsafe {
        CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)
      }?;

      // Note that this would sporadically segfault if we didn't keep a
      // separate variable for `IMMNotificationClient` when registering the
      // callback. Something funky with lifetimes and the COM API's.
      let com_device_callback: IMMNotificationClient = DeviceCallback {
        event_tx: self.event_tx.clone(),
      }
      .into();

      // Register device add/remove callback.
      unsafe {
        com_enumerator
          .RegisterEndpointNotificationCallback(&com_device_callback)
      }?;

      self.com_enumerator = Some(com_enumerator);

      // Update device list and default device IDs.
      for com_device in self.active_devices()? {
        self.add_device(com_device)?;
      }

      self.default_playback_id =
        self.default_device_id(&DeviceType::Playback)?;
      self.default_recording_id =
        self.default_device_id(&DeviceType::Recording)?;

      // Emit initial output.
      self.emit_output();

      // Listen to audio-related events.
      while let Ok(event) = self.event_rx.recv() {
        if let Err(err) = self.handle_event(event) {
          info!("Error handling audio event: {}", err);
        }
      }

      Ok(())
    })
  }

  /// Enumerates active devices of all device types.
  fn active_devices(&self) -> anyhow::Result<Vec<IMMDevice>> {
    let collection = unsafe {
      self
        .com_enumerator
        .as_ref()
        .context("Device enumerator not initialized.")?
        .EnumAudioEndpoints(eAll, DEVICE_STATE_ACTIVE)
    }?;

    let count = unsafe { collection.GetCount() }?;
    let devices = (0..count)
      .filter_map(|i| unsafe { collection.Item(i).ok() })
      .collect::<Vec<_>>();

    Ok(devices)
  }

  /// Gets the friendly name of a device.
  ///
  /// Returns a string (e.g. `Headphones (WH-1000XM3 Stereo)`).
  fn device_name(&self, com_device: &IMMDevice) -> anyhow::Result<String> {
    let store: IPropertyStore =
      unsafe { com_device.OpenPropertyStore(STGM_READ) }?;

    let friendly_name =
      unsafe { store.GetValue(&PKEY_Device_FriendlyName)?.to_string() };

    Ok(friendly_name)
  }

  /// Registers volume callbacks for a device.
  fn register_volume_callback(
    &self,
    com_device: &IMMDevice,
    device_id: String,
  ) -> anyhow::Result<(IAudioEndpointVolume, IAudioEndpointVolumeCallback)>
  {
    let com_volume = unsafe {
      com_device.Activate::<IAudioEndpointVolume>(CLSCTX_ALL, None)
    }?;

    let com_volume_callback: IAudioEndpointVolumeCallback =
      VolumeCallback {
        device_id,
        event_tx: self.event_tx.clone(),
      }
      .into();

    unsafe {
      com_volume.RegisterControlChangeNotify(&com_volume_callback)
    }?;

    Ok((com_volume, com_volume_callback))
  }

  /// Emits an `AudioOutput` update through the provider's emitter.
  fn emit_output(&mut self) {
    let mut output = AudioOutput {
      playback_devices: Vec::new(),
      recording_devices: Vec::new(),
      all_devices: Vec::new(),
      default_playback_device: None,
      default_recording_device: None,
    };

    for (id, state) in &self.device_states {
      let device = &state.output;
      output.all_devices.push(device.clone());

      match device.device_type {
        DeviceType::Playback => {
          output.playback_devices.push(device.clone());
        }
        DeviceType::Recording => {
          output.recording_devices.push(device.clone());
        }
      }

      if self.default_playback_id.as_ref() == Some(id) {
        output.default_playback_device = Some(device.clone());
      }

      if self.default_recording_id.as_ref() == Some(id) {
        output.default_recording_device = Some(device.clone());
      }
    }

    self.common.emitter.emit_output(Ok(output));
  }

  /// Gets the default device ID for the given device type.
  fn default_device_id(
    &self,
    device_type: &DeviceType,
  ) -> anyhow::Result<Option<String>> {
    let default_device = unsafe {
      self
        .com_enumerator
        .as_ref()
        .context("Device enumerator not initialized.")?
        .GetDefaultAudioEndpoint(
          EDataFlow::from(device_type.clone()),
          eMultimedia,
        )
    }
    .ok();

    let device_id = default_device
      .and_then(|device| unsafe { device.GetId().ok() })
      .and_then(|id| unsafe { id.to_string().ok() });

    Ok(device_id)
  }

  /// Whether a device is the default for the given type.
  fn is_default_device(
    &self,
    device_id: &str,
    device_type: &DeviceType,
  ) -> bool {
    match device_type {
      DeviceType::Playback => {
        self.default_playback_id == Some(device_id.to_string())
      }
      DeviceType::Recording => {
        self.default_recording_id == Some(device_id.to_string())
      }
    }
  }

  /// Adds a device by its ID.
  fn add_device_by_id(&mut self, device_id: &str) -> anyhow::Result<()> {
    let com_device = unsafe {
      self
        .com_enumerator
        .as_ref()
        .context("Device enumerator not initialized.")?
        .GetDevice(&HSTRING::from(device_id))
    }?;

    self.add_device(com_device)
  }

  /// Adds a device by its COM object.
  fn add_device(&mut self, com_device: IMMDevice) -> anyhow::Result<()> {
    let device_id = unsafe { com_device.GetId()?.to_string() }?;
    info!("Adding new audio device: {}", device_id);

    let device_type = DeviceType::from(unsafe {
      com_device.cast::<IMMEndpoint>()?.GetDataFlow()
    }?);

    let is_default = self.is_default_device(&device_id, &device_type);

    let (com_volume, com_volume_callback) =
      self.register_volume_callback(&com_device, device_id.clone())?;

    let volume = unsafe { com_volume.GetMasterVolumeLevelScalar() }?;

    let output = AudioDevice {
      name: self.device_name(&com_device)?,
      device_id: device_id.clone(),
      device_type: device_type.clone(),
      volume: (volume * 100.0).round() as u32,
      is_default,
    };

    self.device_states.insert(
      device_id,
      DeviceState {
        com_device,
        output,
        com_volume,
        com_volume_callback,
      },
    );

    Ok(())
  }

  /// Removes a device that is no longer active.
  ///
  /// Deregisters volume callback and removes device from state.
  fn remove_device(&mut self, device_id: &str) -> anyhow::Result<()> {
    if let Some(state) = self.device_states.remove(device_id) {
      info!("Audio device removed: {}", device_id);

      unsafe {
        state.com_volume.UnregisterControlChangeNotify(
          &IAudioEndpointVolumeCallback::from(state.com_volume_callback),
        )
      }?;
    }

    Ok(())
  }

  /// Handles an audio event.
  fn handle_event(&mut self, event: AudioEvent) -> anyhow::Result<()> {
    match event {
      AudioEvent::DeviceAdded(device_id) => {
        self.add_device_by_id(&device_id)?;
      }
      AudioEvent::DeviceRemoved(device_id) => {
        self.remove_device(&device_id)?;
      }
      AudioEvent::DefaultDeviceChanged(device_id, device_type) => {
        match device_type {
          DeviceType::Playback => {
            self.default_playback_id = Some(device_id);
          }
          DeviceType::Recording => {
            self.default_recording_id = Some(device_id);
          }
        }
      }
      AudioEvent::VolumeChanged(device_id, new_volume) => {
        if let Some(state) = self.device_states.get_mut(&device_id) {
          state.output.volume = (new_volume * 100.0).round() as u32;
        }
      }
    }

    // Emit new output after handling the event.
    self.emit_output();

    Ok(())
  }
}

impl Drop for AudioProvider {
  fn drop(&mut self) {
    // // Deregister volume callbacks.
    // for state in self.devices.values() {
    //   unsafe {
    //     let _ = state
    //       .volume_callback
    //       .UnregisterControlChangeNotify(&state.volume_callback);
    //   }
    // }

    // // Deregister device notification callback.
    // if let Some(enumerator) = &self.device_enumerator {
    //   unsafe {
    //     let _ = enumerator.UnregisterEndpointNotificationCallback(
    //       &IMMNotificationClient::null(),
    //     );
    //   }
    // }
  }
}

impl Provider for AudioProvider {
  fn runtime_type(&self) -> RuntimeType {
    RuntimeType::Sync
  }

  fn start_sync(&mut self) {
    if let Err(err) = self.start() {
      tracing::error!("Error starting audio provider: {}", err);
      self.common.emitter.emit_output::<AudioOutput>(Err(err));
    }
  }
}

/// Callback handler for volume notifications.
///
/// Each device has a volume callback that is used to notify when the
/// volume changes.
#[derive(Clone)]
#[windows::core::implement(IAudioEndpointVolumeCallback)]
struct VolumeCallback {
  device_id: String,
  event_tx: channel::Sender<AudioEvent>,
}

impl IAudioEndpointVolumeCallback_Impl for VolumeCallback_Impl {
  fn OnNotify(
    &self,
    data: *mut AUDIO_VOLUME_NOTIFICATION_DATA,
  ) -> windows::core::Result<()> {
    if let Some(data) = unsafe { data.as_ref() } {
      tracing::info!("Volume changed: {:?}", data.fMasterVolume);
      let _ = self.event_tx.send(AudioEvent::VolumeChanged(
        self.device_id.clone(),
        data.fMasterVolume,
      ));
      tracing::info!(
        "Sent volume changed event: {:?}",
        data.fMasterVolume
      );
    }

    Ok(())
  }
}

/// Callback handler for device change notifications.
///
/// This is used to detect when new devices are added or removed, and when
/// the default device changes.
#[windows::core::implement(IMMNotificationClient)]
struct DeviceCallback {
  event_tx: channel::Sender<AudioEvent>,
}

impl IMMNotificationClient_Impl for DeviceCallback_Impl {
  fn OnDeviceAdded(
    &self,
    device_id: &PCWSTR,
  ) -> windows::core::Result<()> {
    if let Ok(id) = unsafe { device_id.to_string() } {
      tracing::info!("Device added: {:?}", id);
      let _ = self.event_tx.send(AudioEvent::DeviceAdded(id.clone()));
      tracing::info!("Sent device added event: {:?}", id);
    }

    Ok(())
  }

  fn OnDeviceRemoved(
    &self,
    device_id: &PCWSTR,
  ) -> windows::core::Result<()> {
    if let Ok(id) = unsafe { device_id.to_string() } {
      tracing::info!("Device removed: {:?}", id);
      let _ = self.event_tx.send(AudioEvent::DeviceRemoved(id.clone()));
      tracing::info!("Sent device removed event: {:?}", id);
    }

    Ok(())
  }

  fn OnDeviceStateChanged(
    &self,
    device_id: &PCWSTR,
    new_state: DEVICE_STATE,
  ) -> windows::core::Result<()> {
    if let Ok(id) = unsafe { device_id.to_string() } {
      tracing::info!("Device state changed: {:?}", new_state);
      let event = match new_state {
        DEVICE_STATE_ACTIVE => AudioEvent::DeviceAdded(id.clone()),
        _ => AudioEvent::DeviceRemoved(id),
      };

      let _ = self.event_tx.send(event);
      tracing::info!("Sent device state changed event");
    }

    Ok(())
  }

  fn OnDefaultDeviceChanged(
    &self,
    flow: EDataFlow,
    role: ERole,
    default_device_id: &PCWSTR,
  ) -> windows::core::Result<()> {
    tracing::info!("Default device changed: {:?}", default_device_id);
    if role == eMultimedia {
      if let Ok(id) = unsafe { default_device_id.to_string() } {
        let _ = self.event_tx.send(AudioEvent::DefaultDeviceChanged(
          id.clone(),
          DeviceType::from(flow),
        ));
        tracing::info!("Sent default device changed event: {:?}", id);
      }
    }

    Ok(())
  }

  fn OnPropertyValueChanged(
    &self,
    _device_id: &PCWSTR,
    _key: &PROPERTYKEY,
  ) -> windows::core::Result<()> {
    Ok(())
  }
}
