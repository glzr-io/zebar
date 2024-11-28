use std::{
  collections::HashMap,
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

// Main provider implementation
pub struct AudioProvider {
  common: CommonProviderState,
  enumerator: Option<IMMDeviceEnumerator>,
  device_volumes: HashMap<String, IAudioEndpointVolume>,
  current_state: AudioOutput,
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
      device_volumes: HashMap::new(),
      current_state: AudioOutput {
        playback_devices: Vec::new(),
        recording_devices: Vec::new(),
        default_playback_device: None,
        default_recording_device: None,
      },
      event_sender,
      event_receiver,
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

  fn update_device_state(&mut self) -> anyhow::Result<()> {
    let enumerator = self
      .enumerator
      .as_ref()
      .context("Enumerator not initialized")?;

    unsafe {
      let collection =
        enumerator.EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)?;
      let default_device = enumerator
        .GetDefaultAudioEndpoint(eRender, eMultimedia)
        .ok();
      let default_id = default_device
        .as_ref()
        .and_then(|d| d.GetId().ok())
        .and_then(|id| id.to_string().ok());

      let mut new_devices = Vec::new();
      let mut new_volumes = HashMap::new();

      for i in 0..collection.GetCount()? {
        if let Ok(device) = collection.Item(i) {
          let (device_id, name) = self.get_device_properties(&device)?;

          // Register/get volume interface
          let endpoint_volume =
            if let Some(existing) = self.device_volumes.get(&device_id) {
              existing.clone()
            } else {
              self.register_volume_callback(&device, device_id.clone())?
            };

          let volume =
            endpoint_volume.GetMasterVolumeLevelScalar()? * 100.0;
          let is_default =
            default_id.as_ref().map_or(false, |id| *id == device_id);

          new_volumes.insert(device_id.clone(), endpoint_volume);

          let device_info = AudioDevice {
            name,
            device_id,
            volume: volume.round() as u32,
            is_default,
          };

          if is_default {
            self.current_state.default_playback_device =
              Some(device_info.clone());
          }
          new_devices.push(device_info);
        }
      }

      self.current_state.playback_devices = new_devices;
      self.device_volumes = new_volumes;

      self
        .common
        .emitter
        .emit_output(Ok(self.current_state.clone()));
    }

    Ok(())
  }

  fn handle_event(&mut self, event: AudioEvent) -> anyhow::Result<()> {
    match event {
      AudioEvent::DeviceAdded(_)
      | AudioEvent::DeviceRemoved(_)
      | AudioEvent::DeviceStateChanged(_, _)
      | AudioEvent::DefaultDeviceChanged(_) => {
        self.update_device_state()?;
      }
      AudioEvent::VolumeChanged(device_id, new_volume) => {
        let volume = (new_volume * 100.0).round() as u32;

        // Update volume in current state
        for device in &mut self.current_state.playback_devices {
          if device.device_id == device_id {
            device.volume = volume;
            if let Some(default_device) =
              &mut self.current_state.default_playback_device
            {
              if default_device.device_id == device_id {
                default_device.volume = volume;
              }
            }
            break;
          }
        }

        self
          .common
          .emitter
          .emit_output(Ok(self.current_state.clone()));
      }
    }

    Ok(())
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
}

impl Drop for AudioProvider {
  fn drop(&mut self) {
    // Clean up volume callbacks
    for volume in self.device_volumes.values() {
      unsafe {
        let _ = volume.UnregisterControlChangeNotify(
          &IAudioEndpointVolumeCallback::null(),
        );
      }
    }

    // Clean up device notification callback
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
    _pwstrDeviceId: &windows::core::PCWSTR,
    _key: &windows::Win32::UI::Shell::PropertiesSystem::PROPERTYKEY,
  ) -> windows::core::Result<()> {
    Ok(())
  }
}
