use std::{
  collections::{HashMap, HashSet},
  ops::Mul,
  sync::{Arc, Mutex, OnceLock},
  time::Duration,
};

use anyhow::Context;
use crossbeam::channel;
use serde::{Deserialize, Serialize};
use tokio::{
  sync::mpsc::{self},
  task,
};
use tracing::debug;
use windows::Win32::{
  Devices::FunctionDiscovery::PKEY_Device_FriendlyName,
  Media::Audio::{
    eCapture, eMultimedia, eRender, EDataFlow, ERole,
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

use crate::providers::{
  CommonProviderState, Provider, ProviderEmitter, RuntimeType,
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AudioProviderConfig {}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioOutput {
  pub playback_devices: Vec<AudioDevice>,
  pub recording_devices: Vec<AudioDevice>,
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

// TODO: Should there be handling for devices that can be both playback and
// recording?
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DeviceType {
  Playback,
  Recording,
}

impl From<EDataFlow> for DeviceType {
  fn from(flow: EDataFlow) -> Self {
    match flow {
      e if e == eRender => Self::Playback,
      e if e == eCapture => Self::Recording,
      _ => Self::Playback,
    }
  }
}

/// Events that can be emitted from audio state changes.
#[derive(Debug)]
enum AudioEvent {
  DeviceAdded(String),
  DeviceRemoved(String),
  DeviceStateChanged(String, DEVICE_STATE),
  DefaultDeviceChanged(String),
  VolumeChanged(String, f32),
}

/// Holds the state of an audio device.
#[derive(Clone)]
struct DeviceState {
  device: AudioDevice,
  volume_callback: IAudioEndpointVolume,
}

pub struct AudioProvider {
  common: CommonProviderState,
  enumerator: Option<IMMDeviceEnumerator>,
  default_playback_id: Option<String>,
  default_recording_id: Option<String>,
  devices: HashMap<String, DeviceState>,
  event_sender: channel::Sender<AudioEvent>,
  event_receiver: channel::Receiver<AudioEvent>,
}

impl AudioProvider {
  pub fn new(
    _config: AudioProviderConfig,
    common: CommonProviderState,
  ) -> Self {
    let (event_sender, event_receiver) = channel::unbounded();

    Self {
      common,
      enumerator: None,
      default_playback_id: None,
      default_recording_id: None,
      devices: HashMap::new(),
      event_sender,
      event_receiver,
    }
  }

  fn create_audio_manager(&mut self) -> anyhow::Result<()> {
    unsafe {
      let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

      let enumerator: IMMDeviceEnumerator =
        CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;

      // Register device callback
      let device_callback = DeviceCallback {
        event_sender: self.event_sender.clone(),
      };
      enumerator.RegisterEndpointNotificationCallback(
        &IMMNotificationClient::from(device_callback),
      )?;

      self.enumerator = Some(enumerator);

      // Initial state update
      self.update_device_state()?;

      // Event loop
      while let Ok(event) = self.event_receiver.recv() {
        if let Err(e) = self.handle_event(event) {
          debug!("Error handling audio event: {}", e);
        }
      }

      Ok(())
    }
  }

  fn get_device_properties(
    &self,
    device: &IMMDevice,
  ) -> anyhow::Result<(String, String)> {
    unsafe {
      let device_id = device.GetId()?.to_string()?;
      let store: IPropertyStore = device.OpenPropertyStore(STGM_READ)?;
      let friendly_name =
        store.GetValue(&PKEY_Device_FriendlyName)?.to_string();
      Ok((device_id, friendly_name))
    }
  }

  fn register_volume_callback(
    &self,
    device: &IMMDevice,
    device_id: String,
  ) -> anyhow::Result<IAudioEndpointVolume> {
    unsafe {
      let endpoint_volume: IAudioEndpointVolume =
        device.Activate(CLSCTX_ALL, None)?;

      let callback = VolumeCallback {
        device_id,
        event_sender: self.event_sender.clone(),
      };

      endpoint_volume.RegisterControlChangeNotify(
        &IAudioEndpointVolumeCallback::from(callback),
      )?;

      Ok(endpoint_volume)
    }
  }

  fn build_output(&self) -> AudioOutput {
    let mut playback_devices = Vec::new();
    let mut recording_devices = Vec::new();
    let mut default_playback_device = None;
    let mut default_recording_device = None;

    for (id, state) in &self.devices {
      match &state.device.device_type {
        DeviceType::Playback => {
          if Some(id) == self.default_playback_id.as_ref() {
            default_playback_device = Some(state.device.clone());
          }
          playback_devices.push(state.device.clone());
        }
        DeviceType::Recording => {
          if Some(id) == self.default_recording_id.as_ref() {
            default_recording_device = Some(state.device.clone());
          }
          recording_devices.push(state.device.clone());
        }
      }
    }

    // Sort devices by name for consistent ordering.
    playback_devices.sort_by(|a, b| a.name.cmp(&b.name));
    recording_devices.sort_by(|a, b| a.name.cmp(&b.name));

    AudioOutput {
      playback_devices,
      recording_devices,
      default_playback_device,
      default_recording_device,
    }
  }

  fn update_device_state(&mut self) -> anyhow::Result<()> {
    let mut active_devices = HashSet::new();

    // Process both playback and recording devices
    for flow in [eRender, eCapture] {
      let devices = self.enumerate_devices(flow)?;
      let default_device = self.get_default_device(flow).ok();
      let default_id = default_device
        .as_ref()
        .and_then(|d| unsafe { d.GetId().ok() })
        .and_then(|id| unsafe { id.to_string().ok() });

      // Update default device IDs
      match flow {
        e if e == eRender => self.default_playback_id = default_id.clone(),
        e if e == eCapture => self.default_recording_id = default_id,
        _ => {}
      }

      for device in devices {
        let (device_id, _) = self.get_device_info(&device, flow)?;
        active_devices.insert(device_id.clone());

        let endpoint_volume =
          if let Some(state) = self.devices.get(&device_id) {
            state.volume_callback.clone()
          } else {
            self.register_volume_callback(&device, device_id.clone())?
          };

        let is_default = match flow {
          e if e == eRender => {
            Some(&device_id) == self.default_playback_id.as_ref()
          }
          e if e == eCapture => {
            Some(&device_id) == self.default_recording_id.as_ref()
          }
          _ => false,
        };

        let device_info = self.create_audio_device(
          &device,
          flow,
          is_default,
          &endpoint_volume,
        )?;

        self.devices.insert(
          device_id,
          DeviceState {
            device: device_info,
            volume_callback: endpoint_volume,
          },
        );
      }
    }

    // Remove devices that are no longer active
    self.devices.retain(|id, _| active_devices.contains(id));

    // Emit updated state
    self.common.emitter.emit_output(Ok(self.build_output()));
    Ok(())
  }

  fn handle_event(&mut self, event: AudioEvent) -> anyhow::Result<()> {
    match event {
      AudioEvent::DeviceAdded(_, _)
      | AudioEvent::DeviceRemoved(_, _)
      | AudioEvent::DeviceStateChanged(_, _, _)
      | AudioEvent::DefaultDeviceChanged(_, _) => {
        self.update_device_state()?;
      }
      AudioEvent::VolumeChanged(device_id, new_volume) => {
        if let Some(state) = self.devices.get_mut(&device_id) {
          state.device.volume = (new_volume * 100.0).round() as u32;
          self.common.emitter.emit_output(Ok(self.build_output()));
        }
      }
    }
    Ok(())
  }
}

impl Drop for AudioProvider {
  fn drop(&mut self) {
    // Deregister volume callbacks.
    for state in self.devices.values() {
      unsafe {
        let _ = state.volume_callback.UnregisterControlChangeNotify(
          &IAudioEndpointVolumeCallback::from(&state.volume_callback),
        );
      }
    }

    // Deregister device notification callback.
    if let Some(enumerator) = &self.enumerator {
      unsafe {
        let _ = enumerator.UnregisterEndpointNotificationCallback(
          &IMMNotificationClient::null(),
        );
      }
    }
  }
}

impl Provider for AudioProvider {
  fn runtime_type(&self) -> RuntimeType {
    RuntimeType::Sync
  }

  fn start_sync(&mut self) {
    if let Err(err) = self.create_audio_manager() {
      self.common.emitter.emit_output::<AudioOutput>(Err(err));
    }
  }
}

/// Callback handler for volume notifications.
///
/// Each device has a volume callback that is used to notify when the
/// volume changes.
#[windows::core::implement(IAudioEndpointVolumeCallback)]
struct VolumeCallback {
  device_id: String,
  event_sender: channel::Sender<AudioEvent>,
}

impl IAudioEndpointVolumeCallback_Impl for VolumeCallback_Impl {
  fn OnNotify(
    &self,
    data: *mut windows::Win32::Media::Audio::AUDIO_VOLUME_NOTIFICATION_DATA,
  ) -> windows::core::Result<()> {
    if let Some(data) = unsafe { data.as_ref() } {
      let _ = self.event_sender.send(AudioEvent::VolumeChanged(
        self.device_id.clone(),
        data.fMasterVolume,
      ));
    }
    Ok(())
  }
}

/// Callback handler for device notifications.
///
/// This is used to detect when new devices are added or removed, and when
/// the default device changes.
#[windows::core::implement(IMMNotificationClient)]
struct DeviceCallback {
  event_sender: channel::Sender<AudioEvent>,
}

impl IMMNotificationClient_Impl for DeviceCallback_Impl {
  fn OnDeviceAdded(
    &self,
    device_id: &windows::core::PCWSTR,
  ) -> windows::core::Result<()> {
    if let Ok(id) = unsafe { device_id.to_string() } {
      let _ = self.event_sender.send(AudioEvent::DeviceAdded(id));
    }
    Ok(())
  }

  fn OnDeviceRemoved(
    &self,
    device_id: &windows::core::PCWSTR,
  ) -> windows::core::Result<()> {
    if let Ok(id) = unsafe { device_id.to_string() } {
      let _ = self.event_sender.send(AudioEvent::DeviceRemoved(id));
    }
    Ok(())
  }

  fn OnDeviceStateChanged(
    &self,
    device_id: &windows::core::PCWSTR,
    new_state: DEVICE_STATE,
  ) -> windows::core::Result<()> {
    if let Ok(id) = unsafe { device_id.to_string() } {
      let _ = self
        .event_sender
        .send(AudioEvent::DeviceStateChanged(id, new_state));
    }
    Ok(())
  }

  fn OnDefaultDeviceChanged(
    &self,
    flow: EDataFlow,
    _role: ERole,
    default_device_id: &windows::core::PCWSTR,
  ) -> windows::core::Result<()> {
    if flow == eRender {
      if let Ok(id) = unsafe { default_device_id.to_string() } {
        let _ =
          self.event_sender.send(AudioEvent::DefaultDeviceChanged(id));
      }
    }
    Ok(())
  }

  fn OnPropertyValueChanged(
    &self,
    _device_id: &windows::core::PCWSTR,
    _key: &windows::Win32::UI::Shell::PropertiesSystem::PROPERTYKEY,
  ) -> windows::core::Result<()> {
    Ok(())
  }
}
