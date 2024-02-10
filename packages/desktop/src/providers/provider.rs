use async_trait::async_trait;
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::info;

use super::manager::ProviderOutput;

#[async_trait]
pub trait Provider {
  async fn on_start(
    &mut self,
    config_hash: String,
    emit_output_tx: Sender<ProviderOutput>,
  );

  async fn on_refresh(
    &mut self,
    config_hash: String,
    emit_output_tx: Sender<ProviderOutput>,
  );

  async fn on_stop(&mut self);

  async fn start(
    &mut self,
    config_hash: String,
    emit_output_tx: Sender<ProviderOutput>,
    mut refresh_rx: Receiver<()>,
    mut stop_rx: Receiver<()>,
  ) {
    let mut has_started = false;

    // Loop to avoid exiting the select on refresh.
    loop {
      let config_hash = config_hash.clone();
      let emit_output_tx = emit_output_tx.clone();

      tokio::select! {
        // Default match arm which handles initialization of the provider.
        // This has a precondition to avoid running again on refresh.
        _ = {
          info!("Starting provider: {}", config_hash);
          has_started = true;
          self.on_start(config_hash.clone(), emit_output_tx.clone())
        }, if !has_started => break,

        // On refresh, re-emit provider variables and continue looping.
        Some(_) = refresh_rx.recv() => {
          info!("Refreshing provider: {}", config_hash);
          _ = self.on_refresh(config_hash, emit_output_tx).await;
        },

        // On stop, perform any necessary clean up and exit the loop.
        Some(_) = stop_rx.recv() => {
          info!("Stopping provider: {}", config_hash);
          _ = self.on_stop().await;
          break;
        },
      }
    }
  }
}
