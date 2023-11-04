use async_trait::async_trait;
use tokio::sync::mpsc::{Receiver, Sender};

#[async_trait]
pub trait Provider {
  async fn on_start(&mut self, output_sender: Sender<String>);
  async fn on_stop(&mut self);
  async fn on_refresh(&mut self);

  async fn start(
    &mut self,
    emit_output_tx: Sender<String>,
    mut refresh_rx: Receiver<()>,
    mut stop_rx: Receiver<()>,
  ) {
    tokio::select! {
        output = self.on_start(emit_output_tx) => output,
        _ = stop_rx.recv() => self.on_stop().await,
        _ = refresh_rx.recv() => self.on_refresh().await
    }
  }
}
