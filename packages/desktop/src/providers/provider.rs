use async_trait::async_trait;
use tokio::{
  sync::mpsc::{self, Receiver, Sender},
  task,
};

use crate::providers::manager::ProviderOutput;

use super::variables::ProviderVariables;

#[async_trait]
pub trait Provider {
  async fn on_start(&mut self, emit_output_tx: Sender<ProviderVariables>);

  async fn on_refresh(&mut self, emit_output_tx: Sender<ProviderVariables>);

  async fn on_stop(&mut self);

  async fn start(
    &mut self,
    options_hash: String,
    emit_output_tx: Sender<ProviderOutput>,
    mut refresh_rx: Receiver<()>,
    mut stop_rx: Receiver<()>,
  ) {
    tokio::select! {
    _ = async {
      let (variable_tx, mut variable_rx) = mpsc::channel::<ProviderVariables>(1);

      let options_hash = options_hash.clone();
      let emit_output_tx = emit_output_tx.clone();
      let x = task::spawn(async move {
      while let Some(variables) = variable_rx.recv().await {
        _ = emit_output_tx.send(ProviderOutput { options_hash: options_hash.clone(), variables}).await;
      }
    });
      self.on_start(variable_tx).await;
      x.abort();
    } => (),
    _ = stop_rx.recv() => self.on_stop().await,
    _ = refresh_rx.recv() => {
      let (variable_tx, mut variable_rx) = mpsc::channel::<ProviderVariables>(1);
      self.on_refresh(variable_tx).await;

      if let Some(variables) = variable_rx.recv().await {
        _ = emit_output_tx.send(ProviderOutput { options_hash: options_hash.clone(), variables}).await;
      }
    },
    }
  }
}
