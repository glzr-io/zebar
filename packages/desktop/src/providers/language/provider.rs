use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use tokio::{
  sync::mpsc::Sender,
  task::{self, AbortHandle},
  time::sleep,
};
use windows::Win32::{
  Globalization::{LCIDToLocaleName, LOCALE_ALLOW_NEUTRAL_NAMES},
  System::SystemServices::LOCALE_NAME_MAX_LENGTH,
  UI::{
    Input::KeyboardAndMouse::GetKeyboardLayout,
    WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId},
  },
};

use super::{LanguageProviderConfig, LanguageVariables};
use crate::providers::{
  provider::Provider,
  provider_ref::{ProviderOutput, VariablesResult},
  variables::ProviderVariables,
};

pub struct LanguageProvider {
  pub config: Arc<LanguageProviderConfig>,
  abort_handle: Option<AbortHandle>,
}

impl LanguageProvider {
  pub fn new(config: LanguageProviderConfig) -> LanguageProvider {
    LanguageProvider {
      config: Arc::new(config),
      abort_handle: None,
    }
  }
}

#[async_trait]
impl Provider for LanguageProvider {
  async fn on_start(
    &mut self,
    config_hash: &str,
    emit_output_tx: Sender<ProviderOutput>,
  ) {
    let config_hash = config_hash.to_string();
    let task_handle = task::spawn(async move {
      let mut previous = 0;
      loop {
        sleep(Duration::from_millis(150)).await;

        let keyboard_layout = unsafe {
          GetKeyboardLayout(GetWindowThreadProcessId(
            GetForegroundWindow(),
            None,
          ))
        };
        let lang_id = (keyboard_layout.0 as u32) & 0xffff;

        if lang_id == previous {
          continue;
        }
        previous = lang_id;

        let mut locale_name = [0; LOCALE_NAME_MAX_LENGTH as usize];

        let result = unsafe {
          LCIDToLocaleName(
            lang_id,
            Some(&mut locale_name),
            LOCALE_ALLOW_NEUTRAL_NAMES,
          )
        };

        if result > 0 {
          let language_name =
            String::from_utf16_lossy(&locale_name[..result as usize]);
          _ = emit_output_tx
            .send(ProviderOutput {
              config_hash: config_hash.clone(),
              variables: VariablesResult::Data(
                ProviderVariables::Language(LanguageVariables {
                  language: language_name,
                }),
              ),
            })
            .await;
        }
      }
    });

    self.abort_handle = Some(task_handle.abort_handle());
    _ = task_handle.await;
  }

  async fn on_refresh(
    &mut self,
    _config_hash: &str,
    _emit_output_tx: Sender<ProviderOutput>,
  ) {
  }

  async fn on_stop(&mut self) {}

  fn min_refresh_interval(&self) -> Option<Duration> {
    None
  }
}
