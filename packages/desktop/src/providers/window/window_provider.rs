use serde::{Deserialize, Serialize};
use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW},
};
use windows_core::PWSTR;

use crate::{
    common::SyncInterval,
    providers::{CommonProviderState, Provider, ProviderInputMsg, RuntimeType},
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WindowProviderConfig {
    pub refresh_interval: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowOutput {
    pub title: String,
}

pub struct WindowProvider {
    config: WindowProviderConfig,
    common: CommonProviderState,
    fallback_titles: Vec<&'static str>, // Titles to replace with "File Explorer"
}

impl WindowProvider {
  pub fn new(
      config: WindowProviderConfig,
      common: CommonProviderState,
  ) -> WindowProvider {
      WindowProvider {
          config,
          common,
          fallback_titles: vec![
            "Zebar - macos/macos",
            "Program Manager",
            "Explorer",
            "Desktop",
          ],
      }
  }

  fn get_active_window_title(&self) -> anyhow::Result<String> {
      unsafe {
          let hwnd: HWND = GetForegroundWindow();
          let default_title = "File Explorer".to_string();

          if hwnd.0.is_null() {
              return Ok(default_title);
          }

          let mut buffer = [0u16; 512]; // Increased buffer size slightly just in case
          let length = GetWindowTextW(hwnd, &mut buffer);

          if length > 0 {
              let full_title = String::from_utf16_lossy(&buffer[..length as usize]);
              let full_title = full_title.trim();

              if full_title.is_empty() {
                  return Ok(default_title);
              }

              if self.fallback_titles.iter().any(|&t| t == full_title) {
                  return Ok(default_title);
              }

              let processed_title = full_title
              .replace(" – ", " - ") // Replace en-dash with space-hyphen-space
              .replace(" — ", " - "); // Replace em-dash with space-hyphen-space

              let app_title = processed_title
                  .split(" - ")
                  .last()
                  .map(|s| s.trim())
                  .filter(|s| !s.is_empty())
                  .unwrap_or(full_title);

              Ok(app_title.to_string())
          } else {
              Ok(default_title)
          }
      }
  }

  fn run_interval(&self) -> anyhow::Result<WindowOutput> {
      let title = self.get_active_window_title()?;
      Ok(WindowOutput { title })
  }
}

impl Provider for WindowProvider {
    fn runtime_type(&self) -> RuntimeType {
        RuntimeType::Sync
    }

    fn start_sync(&mut self) {
        let mut interval = SyncInterval::new(self.config.refresh_interval);

        loop {
            crossbeam::select! {
                recv(interval.tick()) -> _ => {
                    let output = self.run_interval();
                    self.common.emitter.emit_output(output);
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