use anyhow::bail;
use windows::Win32::{
  Globalization::{LCIDToLocaleName, LOCALE_ALLOW_NEUTRAL_NAMES},
  System::SystemServices::LOCALE_NAME_MAX_LENGTH,
  UI::{
    Input::KeyboardAndMouse::GetKeyboardLayout,
    WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId},
  },
};

use super::{KeyboardOutput, KeyboardProviderConfig};
use crate::{impl_interval_provider, providers::ProviderOutput};

pub struct KeyboardProvider {
  config: KeyboardProviderConfig,
}

impl KeyboardProvider {
  pub fn new(config: KeyboardProviderConfig) -> KeyboardProvider {
    KeyboardProvider { config }
  }

  fn refresh_interval_ms(&self) -> u64 {
    self.config.refresh_interval
  }

  async fn run_interval(&self) -> anyhow::Result<ProviderOutput> {
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

    Ok(ProviderOutput::Keyboard(KeyboardOutput {
      layout: layout_name,
    }))
  }
}

impl_interval_provider!(KeyboardProvider, false);
