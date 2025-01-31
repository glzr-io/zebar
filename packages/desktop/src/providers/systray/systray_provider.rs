use serde::{Deserialize, Serialize};
use systray_util::{Systray, SystrayEvent};

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

#[async_trait]
impl Provider for SystrayProvider {
  fn runtime_type(&self) -> RuntimeType {
    RuntimeType::Async
  }

  async fn start_async(&mut self) {
    let systray = Systray::new();

    loop {
      tokio::select! {
        Some(event) = systray.icons_changed.recv() => {
          match event {
            SystrayEvent::IconAdd(icon) => {
              self.common.emitter.emit_output(SystrayOutput { icons: vec![icon] });
            }
          }
        }
        Some(message) = self.common.input.async_rx.recv() => {
          if let ProviderInputMsg::Stop = message {
            break;
          }
        }
      }
    }
  }
}
