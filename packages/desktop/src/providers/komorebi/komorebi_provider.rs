use async_trait::async_trait;
use komorebi_util::KomorebiClient;
use serde::Deserialize;

use crate::providers::{
  CommonProviderState, Provider, ProviderInputMsg, RuntimeType,
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KomorebiProviderConfig {}

pub type KomorebiOutput = komorebi_util::KomorebiOutput;

pub struct KomorebiProvider {
  common: CommonProviderState,
}

impl KomorebiProvider {
  pub fn new(
    _config: KomorebiProviderConfig,
    common: CommonProviderState,
  ) -> KomorebiProvider {
    KomorebiProvider { common }
  }
}

#[async_trait]
impl Provider for KomorebiProvider {
  fn runtime_type(&self) -> RuntimeType {
    RuntimeType::Async
  }

  async fn start_async(&mut self) {
    let Ok(mut client) = KomorebiClient::new("zebar.sock") else {
      self.common.emitter.emit_output::<KomorebiOutput>(Err(
        anyhow::anyhow!("Failed to initialize komorebi client."),
      ));

      return;
    };

    loop {
      tokio::select! {
        Ok(output) = client.output() => {
          self.common.emitter.emit_output(Ok(output));
        }
        Some(input) = self.common.input.async_rx.recv() => {
          if let ProviderInputMsg::Stop = input {
            break;
          }
        }
      }
    }
  }
}
