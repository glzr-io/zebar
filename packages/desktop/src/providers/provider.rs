use async_trait::async_trait;
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::info;

use super::manager::ProviderOutput;

#[async_trait]
pub trait Provider {
  async fn on_start(
    &mut self,
    options_hash: String,
    emit_output_tx: Sender<ProviderOutput>,
  );

  async fn on_refresh(
    &mut self,
    options_hash: String,
    emit_output_tx: Sender<ProviderOutput>,
  );

  async fn on_stop(&mut self);

  async fn start(
    &mut self,
    options_hash: String,
    emit_output_tx: Sender<ProviderOutput>,
    mut refresh_rx: Receiver<()>,
    mut stop_rx: Receiver<()>,
  ) {
    // Loop to avoid exiting the select on refresh.
    loop {
      let options_hash = options_hash.clone();
      let emit_output_tx = emit_output_tx.clone();

      tokio::select! {
        _ = self.on_start(options_hash.clone(), emit_output_tx.clone()) => break,
        Some(_) = refresh_rx.recv() => {
          info!("Refreshing provider: {}", options_hash);
          _ = self.on_refresh(options_hash, emit_output_tx).await;
        },
        Some(_) = stop_rx.recv() => {
          info!("Stopping provider: {}", options_hash);
          _ = self.on_stop().await;
          break;
        },
      }
    }
  }
}
