use anyhow::bail;
use serde::{Deserialize, Serialize};
use windows::Win32::{
  Globalization::{LCIDToLocaleName, LOCALE_ALLOW_NEUTRAL_NAMES},
  System::SystemServices::LOCALE_NAME_MAX_LENGTH,
  UI::{
    Input::KeyboardAndMouse::GetKeyboardLayout,
    WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId},
  },
};

use crate::{
  common::SyncInterval,
  providers::{
    CommonProviderState, Provider, ProviderInputMsg, RuntimeType,
  },
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KeyboardProviderConfig {
  pub refresh_interval: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyboardOutput {
  pub layout: String,
}

pub struct KeyboardProvider {
  config: KeyboardProviderConfig,
  common: CommonProviderState,
}

impl KeyboardProvider {
  pub fn new(
    config: KeyboardProviderConfig,
    common: CommonProviderState,
  ) -> KeyboardProvider {
    KeyboardProvider { config, common }
  }

  fn run_interval(&mut self) -> anyhow::Result<KeyboardOutput> {
    let keyboard_layout = unsafe {
      GetKeyboardLayout(GetWindowThreadProcessId(
        GetForegroundWindow(),
        None,
      ))
    };

    let lang_id = (keyboard_layout.0 as u32) & 0xffff;
    let mut locale_name = [0; LOCALE_NAME_MAX_LENGTH as usize];

    let result = unsafe {
      LCIDToLocaleName(
        lang_id,
        Some(&mut locale_name),
        LOCALE_ALLOW_NEUTRAL_NAMES,
      )
    };

    if result == 0 {
      bail!("Failed to get keyboard layout name.");
    }

    let layout_name =
      String::from_utf16_lossy(&locale_name[..result as usize]);

    Ok(KeyboardOutput {
      layout: layout_name,
    })
  }
}

impl Provider for KeyboardProvider {
  fn runtime_type(&self) -> RuntimeType {
    RuntimeType::Sync
  }

  fn start_sync(&mut self) {
    let mut interval = SyncInterval::new(self.config.refresh_interval);

    loop {
      crossbeam::select! {
        recv(interval.tick()) -> _ => {
          let output = self.run_interval();
          self.common.emitter.emit_output_cached(output);
        }
        recv(self.common.input.sync_rx) -> input => {
          if let Ok(ProviderInputMsg::Stop) = input {
            break;
          }
        }
      }
    }
  }
}
