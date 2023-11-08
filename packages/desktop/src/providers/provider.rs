use std::time::Duration;

use async_trait::async_trait;
use tokio::{
  sync::{
    mpsc::{Receiver, Sender},
    Mutex,
  },
  task::{self, AbortHandle},
  time,
};

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

#[async_trait]
pub trait RefreshableProvider<T: Send + 'static>: Provider {
  fn refresh_interval_ms(&self) -> u64;

  fn state(&self) -> Mutex<T>;

  fn abort_handle(&self) -> Option<AbortHandle>;

  fn set_abort_handle(&self, abort_handle: AbortHandle);

  async fn refresh_and_emit(
    emit_output_tx: &Sender<ProviderVariables>,
    state: &Mutex<T>,
  );

  async fn on_start(&mut self, emit_output_tx: Sender<ProviderVariables>) {
    let refresh_interval_ms = self.refresh_interval_ms();
    let state = self.state();

    let forever = task::spawn(async move {
      let mut interval =
        time::interval(Duration::from_millis(refresh_interval_ms));

      Self::refresh_and_emit(&emit_output_tx, &state).await;

      loop {
        interval.tick().await;
        Self::refresh_and_emit(&emit_output_tx, &state).await;
      }
    });

    self.set_abort_handle(forever.abort_handle());
    _ = forever.await;
  }

  async fn on_refresh(&mut self, emit_output_tx: Sender<ProviderVariables>) {
    Self::refresh_and_emit(&emit_output_tx, &self.state()).await;
  }

  async fn stop(&mut self) {
    if let Some(handle) = &self.abort_handle() {
      handle.abort();
    }
  }
}
