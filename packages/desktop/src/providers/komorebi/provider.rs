use std::{
  io::{BufRead, BufReader},
  sync::Arc,
};

use async_trait::async_trait;
use komorebi_client::Monitor;
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

use super::{KomorebiMonitor, KomorebiProviderConfig};

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

  fn transform_response(
    state: komorebi_client::State,
  ) -> KomorebiVariables {
    let monitors = state
      .monitors
      .elements()
      .into_iter()
      .map(|&monitor| KomorebiMonitor {
        id: monitor.id(),
        name: monitor.name().to_string(),
        device_id: monitor.device_id().clone(),
        size: monitor.size(),
        work_area_size: KomorebiRect {
          left: size.left,
          top: size.top,
          right: size.right,
          bottom: size.bottom,
        },
        work_area_offset: monitor.work_area_offset(),
        work_area_size: monitor.work_area_size(),
        workspaces: monitor.workspaces(),
      })
      .collect();

    KomorebiVariables { monitors }
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
