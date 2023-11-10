use async_trait::async_trait;
use tokio::{
  sync::mpsc::{self, Receiver, Sender},
  task::{self, JoinHandle},
};

use crate::providers::manager::ProviderOutput;

use super::variables::ProviderVariables;

fn create_variable_channel(
  options_hash: &str,
  emit_output_tx: Sender<ProviderOutput>,
) -> (Sender<ProviderVariables>, JoinHandle<()>) {
  let (variable_tx, mut variable_rx) = mpsc::channel::<ProviderVariables>(1);
  let options_hash = options_hash.to_owned();

  let output_task = task::spawn(async move {
    while let Some(variables) = variable_rx.recv().await {
      _ = emit_output_tx
        .send(ProviderOutput {
          options_hash: options_hash.clone(),
          variables,
        })
        .await;
    }
  });

  (variable_tx, output_task)
}

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
        let (variable_tx, output_task) =
          create_variable_channel(&options_hash, emit_output_tx.clone());

        // Start provider and wrap variable outputs with its options hash.
        self.on_start(variable_tx).await;
        output_task.abort();
      } => (),
      _ = refresh_rx.recv() => {
        let (variable_tx, output_task) =
          create_variable_channel(&options_hash, emit_output_tx);

        // Refresh provider and wrap variable outputs with its options hash.
        self.on_refresh(variable_tx).await;
        output_task.abort();
      },
      _ = stop_rx.recv() => self.on_stop().await,
    }
  }
}
