use std::{
  io::{BufRead, BufReader},
  sync::Arc,
};

use async_trait::async_trait;
use tokio::sync::mpsc::Sender;
use tracing::{debug, info};

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
    info!("fdsjaiofdsajo");
    let socket = komorebi_client::subscribe(NAME).unwrap();
    info!("Connected to Komorebi socket.");

    for incoming in socket.incoming() {
      info!("Incoming Komorebi socket message.");

      match incoming {
        Ok(data) => {
          let reader = BufReader::new(data.try_clone().unwrap());

          for line in reader.lines().flatten() {
            println!("line: {}", line);

            emit_output_tx
              .send(ProviderOutput {
                config_hash: config_hash.clone(),
                variables: VariablesResult::Data(
                  ProviderVariables::Komorebi(KomorebiVariables {
                    state_json: line.clone(),
                  }),
                ),
              })
              .await
              .unwrap();

            // let notification: komorebi_client::Notification =
            //   serde_json::from_str(&line).unwrap();
          }
        }
        Err(error) => { /* log any errors */ }
      }
    }
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
