use std::time::Duration;

use async_trait::async_trait;
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::info;

use super::provider_ref::ProviderOutput;

#[async_trait]
pub trait Provider {
  // TODO: Change to `Option<Duration>`.
  fn min_refresh_interval(&self) -> Duration;

  async fn on_start(
    &mut self,
    config_hash: &str,
    emit_output_tx: Sender<ProviderOutput>,
  );

  async fn on_refresh(
    &mut self,
    config_hash: &str,
    emit_output_tx: Sender<ProviderOutput>,
  );

  async fn on_stop(&mut self);

  // TODO: Remove this.
  async fn start(
    &mut self,
    config_hash: &str,
    emit_output_tx: Sender<ProviderOutput>,
    mut refresh_rx: Receiver<()>,
    mut stop_rx: Receiver<()>,
  ) {
    info!("Starting provider: {}", config_hash);
    self.on_start(config_hash, emit_output_tx.clone()).await;

    // Loop to avoid exiting the select on refresh.
    loop {
      tokio::select! {
        // On refresh, re-emit provider variables and continue looping.
        Some(_) = refresh_rx.recv() => {
          info!("Refreshing provider: {}", config_hash);
          _ = self.on_refresh(config_hash, emit_output_tx.clone()).await;
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
