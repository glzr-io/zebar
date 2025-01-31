use serde::{Deserialize, Serialize};
use systray_util::Systray;

use crate::providers::{
  CommonProviderState, Provider, ProviderInputMsg, RuntimeType,
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SystrayProviderConfig {}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystrayOutput {
  pub icons: Vec<SystrayIcon>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystrayIcon {
  pub id: String,
  pub title: String,
  pub icon: String,
}

pub struct SystrayProvider {
  config: SystrayProviderConfig,
  common: CommonProviderState,
}

impl SystrayProvider {
  pub fn new(
    config: SystrayProviderConfig,
    common: CommonProviderState,
  ) -> SystrayProvider {
    SystrayProvider { config, common }
  }
}

impl Provider for SystrayProvider {
  fn runtime_type(&self) -> RuntimeType {
    RuntimeType::Sync
  }

  fn start_sync(&mut self) {
    // TODO: Error handling.
    let mut systray = Systray::new().unwrap();

    while let Some(event) = systray.changes() {
      self.common.emitter.emit_output(Ok(SystrayOutput {
        // TODO: Convert IconData to SystrayIcon.
        icons: systray.icons.values().cloned().collect(),
      }));
    }
  }
}
