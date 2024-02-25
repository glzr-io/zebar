use std::{
  io::{BufRead, BufReader},
  sync::Arc,
};

use async_trait::async_trait;
use tokio::{sync::mpsc::Sender, task};
use tracing::{debug, error, info};

use crate::providers::{
  komorebi::KomorebiVariables,
  manager::{ProviderOutput, VariablesResult},
  provider::Provider,
  variables::ProviderVariables,
};

use super::KomorebiProviderConfig;

pub struct KomorebiProvider {
  pub config: Arc<KomorebiProviderConfig>,
}

impl KomorebiProvider {
  pub fn new(config: KomorebiProviderConfig) -> KomorebiProvider {
    KomorebiProvider {
      config: Arc::new(config),
    }
  }
}

const NAME: &str = "zebar.sock";

#[async_trait]
impl Provider for KomorebiProvider {
  async fn on_start(
    &mut self,
    config_hash: String,
    emit_output_tx: Sender<ProviderOutput>,
  ) {
    let forever = task::spawn(async move {
      let socket = komorebi_client::subscribe(NAME).unwrap();
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
  }

  async fn on_refresh(
    &mut self,
    config_hash: String,
    emit_output_tx: Sender<ProviderOutput>,
  ) {
    //
  }

  async fn on_stop(&mut self) {
    //
  }
}
