use std::io::BufRead;
use std::io::BufReader;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::mpsc::Sender;

use crate::providers::{manager::ProviderOutput, provider::Provider};

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

const NAME: &str = "komokana.sock";

#[async_trait]
impl Provider for KomorebiProvider {
  async fn on_start(
    &mut self,
    config_hash: String,
    emit_output_tx: Sender<ProviderOutput>,
  ) {
    let socket = komorebi_client::subscribe(NAME).unwrap();
    println!("connected to komorebi");

    for incoming in socket.incoming() {
      println!("incoming socket message");

      match incoming {
        Ok(data) => {
          let reader = BufReader::new(data.try_clone().unwrap());

          for line in reader.lines().flatten() {
            println!("line: {}", line);

            let notification: komorebi_client::Notification =
              serde_json::from_str(&line).unwrap();

            println!("notification: {:?}", notification);
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
