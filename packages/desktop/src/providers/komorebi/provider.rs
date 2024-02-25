use std::{
  io::{BufRead, BufReader},
  sync::Arc,
};

use anyhow::Result;
use async_trait::async_trait;
use komorebi_client::{SocketMessage, UnixListener};
use tokio::{
  sync::mpsc::Sender,
  task::{self, AbortHandle},
};
use tracing::{debug, info};

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
  socket: Arc<UnixListener>,
  abort_handle: Option<AbortHandle>,
}

impl KomorebiProvider {
  pub fn new(config: KomorebiProviderConfig) -> Result<KomorebiProvider> {
    let socket = komorebi_client::subscribe(SOCKET_NAME)?;
    debug!("Connected to Komorebi socket.");

    Ok(KomorebiProvider {
      config: Arc::new(config),
      socket: Arc::new(socket),
      abort_handle: None,
    })
  }
}

#[async_trait]
impl Provider for KomorebiProvider {
  async fn on_start(
    &mut self,
    config_hash: String,
    emit_output_tx: Sender<ProviderOutput>,
  ) {
    let socket = self.socket.clone();

    let task_handle = task::spawn(async move {
      // let socket = komorebi_client::subscribe(SOCKET_NAME).unwrap();
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
          Err(error) => { /* log any errors */ }
        }
      }
    });

    self.abort_handle = Some(task_handle.abort_handle());
    _ = task_handle.await;
  }

  async fn on_refresh(
    &mut self,
    config_hash: String,
    emit_output_tx: Sender<ProviderOutput>,
  ) {
    //
    let res = komorebi_client::send_query(&SocketMessage::State);
    info!("Komorebi state: {:?}", res);
  }

  async fn on_stop(&mut self) {
    //
  }
}
