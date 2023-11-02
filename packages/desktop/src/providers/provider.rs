use async_trait::async_trait;
use tokio::sync::mpsc::{Receiver, Sender};

#[async_trait]
pub trait Provider {
  // TODO: Rename to `on_start`
  async fn start(&mut self, output_sender: Sender<String>);
  async fn start2(
    &mut self,
    emit_output_tx: Sender<String>,
    mut refresh_rx: Receiver<()>,
    mut stop_rx: Receiver<()>,
  ) {
    tokio::select! {
        output = self.start(emit_output_tx) => output,
        // _ = stop_rx.recv() => ()
        _ = stop_rx.recv() => self.stop().await

    }
  }
  // TODO: Rename to `on_stop`
  async fn stop(&mut self);
}
