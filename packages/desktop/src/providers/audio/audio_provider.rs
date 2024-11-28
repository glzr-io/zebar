use std::collections::{HashMap, HashSet};

use anyhow::Context;
use crossbeam::channel;
use serde::{Deserialize, Serialize};
use tracing::debug;
use windows::Win32::{
  Devices::FunctionDiscovery::PKEY_Device_FriendlyName,
  Media::Audio::{
    eCapture, eRender, EDataFlow, ERole,
    Endpoints::{
      IAudioEndpointVolume, IAudioEndpointVolumeCallback,
      IAudioEndpointVolumeCallback_Impl,
    },
    IMMDevice, IMMDeviceEnumerator, IMMNotificationClient,
    IMMNotificationClient_Impl, MMDeviceEnumerator,
    AUDIO_VOLUME_NOTIFICATION_DATA, DEVICE_STATE, DEVICE_STATE_ACTIVE,
  },
  System::Com::{CoCreateInstance, CLSCTX_ALL, STGM_READ},
  UI::Shell::PropertiesSystem::{IPropertyStore, PROPERTYKEY},
};
use windows_core::PCWSTR;

use crate::{
  common::windows::COM_INIT,
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
  Hybrid,
}

impl From<EDataFlow> for DeviceType {
  fn from(flow: EDataFlow) -> Self {
    match flow {
      e if e == eRender => Self::Playback,
      e if e == eCapture => Self::Recording,
      _ => Self::Hybrid,
    }
  }
}

/// Events that can be emitted from audio state changes.
#[derive(Debug)]
enum AudioEvent {
  DeviceAdded(String),
  DeviceRemoved(String),
  DeviceStateChanged(String, DEVICE_STATE),
  DefaultDeviceChanged(String, EDataFlow),
  VolumeChanged(String, f32),
}

/// Holds the state of an audio device.
#[derive(Clone)]
struct DeviceState {
  device: IMMDevice,
  output: AudioDevice,
  volume_callback: IAudioEndpointVolume,
}

pub struct AudioProvider {
  common: CommonProviderState,
  device_enumerator: Option<IMMDeviceEnumerator>,
  default_playback_id: Option<String>,
  default_recording_id: Option<String>,
  devices: HashMap<String, DeviceState>,
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
      device_enumerator: None,
      default_playback_id: None,
      default_recording_id: None,
      devices: HashMap::new(),
      event_tx,
      event_rx,
    }
  }

  /// Main entry point.
  fn start(&mut self) -> anyhow::Result<()> {
    COM_INIT.with(|_| unsafe {
      let device_enumerator: IMMDeviceEnumerator =
        CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;

      // Register device add/remove callback.
      device_enumerator.RegisterEndpointNotificationCallback(
        &IMMNotificationClient::from(DeviceCallback {
          event_sender: self.event_tx.clone(),
        }),
      )?;

      self.device_enumerator = Some(device_enumerator);

      // Emit initial state.
      self.update_device_state()?;

      // Listen to audio-related events.
      while let Ok(event) = self.event_rx.recv() {
        if let Err(err) = self.handle_event(event) {
          debug!("Error handling audio event: {}", err);
        }
      }

      Ok(())
    })
  }

  /// Enumerates active devices of a specific type.
  fn enumerate_devices(
    &self,
    flow: EDataFlow,
  ) -> anyhow::Result<Vec<IMMDevice>> {
    let collection = unsafe {
      self
        .device_enumerator
        .as_ref()
        .context("Device enumerator not initialized.")?
        .EnumAudioEndpoints(flow, DEVICE_STATE_ACTIVE)
    }?;

    let count = unsafe { collection.GetCount() }?;
    let devices = (0..count)
      .filter_map(|i| unsafe { collection.Item(i).ok() })
      .collect::<Vec<_>>();

    Ok(devices)
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

  /// Registers volume callbacks for a device.
  fn register_volume_callback(
    &self,
    device: &IMMDevice,
    device_id: String,
  ) -> anyhow::Result<IAudioEndpointVolume> {
    let endpoint_volume: IAudioEndpointVolume =
      unsafe { device.Activate(CLSCTX_ALL, None) }?;

    let callback = VolumeCallback {
      device_id,
      event_sender: self.event_tx.clone(),
    };

    unsafe {
      endpoint_volume.RegisterControlChangeNotify(
        &IAudioEndpointVolumeCallback::from(callback),
      )
    }?;

    Ok(endpoint_volume)
  }

  /// Emits an `AudioOutput` update through the provider's emitter.
  fn emit_output(&mut self) {
    let default_playback_device = self
      .default_playback_id
      .as_ref()
      .and_then(|id| self.devices.get(id))
      .map(|state| state.output.clone());

    let default_recording_device = self
      .default_recording_id
      .as_ref()
      .and_then(|id| self.devices.get(id))
      .map(|state| state.output.clone());

    let playback_devices = self
      .devices
      .values()
      .filter(|state| state.output.device_type == DeviceType::Playback)
      .map(|state| state.output.clone())
      .collect();

    let recording_devices = self
      .devices
      .values()
      .filter(|state| state.output.device_type == DeviceType::Recording)
      .map(|state| state.output.clone())
      .collect();

    let all_devices = self
      .devices
      .values()
      .map(|state| state.output.clone())
      .collect();

    self.common.emitter.emit_output_cached(Ok(AudioOutput {
      playback_devices,
      recording_devices,
      all_devices,
      default_playback_device,
      default_recording_device,
    }));
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
            output: device_info,
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
      AudioEvent::DeviceAdded(..)
      | AudioEvent::DeviceRemoved(..)
      | AudioEvent::DeviceStateChanged(..)
      | AudioEvent::DefaultDeviceChanged(..) => {
        self.update_device_state()?;
      }
      AudioEvent::VolumeChanged(device_id, new_volume) => {
        if let Some(state) = self.devices.get_mut(&device_id) {
          state.output.volume = (new_volume * 100.0).round() as u32;
          self.common.emitter.emit_output(Ok(self.build_output()));
        }
      }
    }

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
  event_sender: channel::Sender<AudioEvent>,
}

impl IAudioEndpointVolumeCallback_Impl for VolumeCallback_Impl {
  fn OnNotify(
    &self,
    data: *mut AUDIO_VOLUME_NOTIFICATION_DATA,
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

/// Callback handler for device change notifications.
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
    device_id: &PCWSTR,
  ) -> windows::core::Result<()> {
    if let Ok(id) = unsafe { device_id.to_string() } {
      let _ = self.event_sender.send(AudioEvent::DeviceAdded(id));
    }

    Ok(())
  }

  fn OnDeviceRemoved(
    &self,
    device_id: &PCWSTR,
  ) -> windows::core::Result<()> {
    if let Ok(id) = unsafe { device_id.to_string() } {
      let _ = self.event_sender.send(AudioEvent::DeviceRemoved(id));
    }

    Ok(())
  }

  fn OnDeviceStateChanged(
    &self,
    device_id: &PCWSTR,
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
    default_device_id: &PCWSTR,
  ) -> windows::core::Result<()> {
    if let Ok(id) = unsafe { default_device_id.to_string() } {
      let _ = self
        .event_sender
        .send(AudioEvent::DefaultDeviceChanged(id, flow));
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
