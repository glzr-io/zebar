use async_trait::async_trait;
use tokio::sync::mpsc::{Receiver, Sender};

use super::variables::ProviderVariables;

#[async_trait]
pub trait Provider {
  async fn on_start(&mut self, emit_output_tx: Sender<ProviderVariables>);
  async fn on_refresh(&mut self, emit_output_tx: Sender<ProviderVariables>);
  async fn on_stop(&mut self);

  async fn start(
    &mut self,
    emit_output_tx: Sender<ProviderVariables>,
    mut refresh_rx: Receiver<()>,
    mut stop_rx: Receiver<()>,
  ) {
    tokio::select! {
        output = self.on_start(emit_output_tx.clone()) => output,
        _ = stop_rx.recv() => self.on_stop().await,
        _ = refresh_rx.recv() => self.on_refresh(emit_output_tx).await
    }
  }
}
