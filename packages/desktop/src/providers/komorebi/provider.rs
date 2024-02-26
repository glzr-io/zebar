use std::{
  io::{BufRead, BufReader},
  sync::Arc,
};

use async_trait::async_trait;
use tokio::{
  sync::mpsc::Sender,
  task::{self, AbortHandle},
};
use tracing::debug;

use crate::providers::{
  komorebi::KomorebiVariables,
  manager::{ProviderOutput, VariablesResult},
  provider::Provider,
  variables::ProviderVariables,
};

use super::KomorebiProviderConfig;

const SOCKET_NAME: &str = "zebar.sock";

pub struct KomorebiProvider {
  pub config: Arc<KomorebiProviderConfig>,
  abort_handle: Option<AbortHandle>,
}

impl KomorebiProvider {
  pub fn new(config: KomorebiProviderConfig) -> KomorebiProvider {
    KomorebiProvider {
      config: Arc::new(config),
      abort_handle: None,
    }
  }
}

#[async_trait]
impl Provider for KomorebiProvider {
  async fn on_start(
    &mut self,
    config_hash: String,
    emit_output_tx: Sender<ProviderOutput>,
  ) {
    let task_handle = task::spawn(async move {
      let socket = komorebi_client::subscribe(SOCKET_NAME).unwrap();
      debug!("Connected to Komorebi socket.");

      for incoming in socket.incoming() {
        debug!("Incoming Komorebi socket message.");

        match incoming {
          Ok(data) => {
            let reader = BufReader::new(data.try_clone().unwrap());

            for line in reader.lines().flatten() {
              let notification: komorebi_client::Notification =
                serde_json::from_str(&line).unwrap();

              _ = emit_output_tx
                .send(ProviderOutput {
                  config_hash: config_hash.clone(),
                  variables: VariablesResult::Data(
                    ProviderVariables::Komorebi(KomorebiVariables {
                      monitors: notification.state.monitors,
                    }),
                  ),
                })
                .await;
            }
          }
          Err(error) => {
            _ = emit_output_tx
              .send(ProviderOutput {
                config_hash: config_hash.clone(),
                variables: VariablesResult::Error(error.to_string()),
              })
              .await;
          }
        }
      }
    });

    self.abort_handle = Some(task_handle.abort_handle());
    _ = task_handle.await;
  }

  async fn on_refresh(
    &mut self,
    _config_hash: String,
    _emit_output_tx: Sender<ProviderOutput>,
  ) {
    // No-op.
  }

  async fn on_stop(&mut self) {
    if let Some(handle) = &self.abort_handle {
      handle.abort();
    }
  }
}
