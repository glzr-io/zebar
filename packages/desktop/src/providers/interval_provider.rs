use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use tokio::{
  sync::{mpsc::Sender, Mutex},
  task::{self, AbortHandle},
  time,
};

use super::{provider::Provider, variables::ProviderVariables};

#[async_trait]
pub trait IntervalProvider {
  type State: Send + 'static;

  fn refresh_interval_ms(&self) -> u64;

  fn state(&self) -> Arc<Mutex<Self::State>>;

  fn abort_handle(&self) -> &Option<AbortHandle>;

  fn set_abort_handle(&mut self, abort_handle: AbortHandle);

  async fn refresh_and_emit(
    emit_output_tx: &Sender<ProviderVariables>,
    state: &Mutex<Self::State>,
  );
}

#[async_trait]
impl<T: IntervalProvider + Send> Provider for T {
  async fn on_start(&mut self, emit_output_tx: Sender<ProviderVariables>) {
    let refresh_interval_ms = self.refresh_interval_ms();
    let state = self.state();

    let forever = task::spawn(async move {
      let mut interval =
        time::interval(Duration::from_millis(refresh_interval_ms));

      T::refresh_and_emit(&emit_output_tx, &state).await;

      loop {
        interval.tick().await;
        T::refresh_and_emit(&emit_output_tx, &state).await;
      }
    });

    self.set_abort_handle(forever.abort_handle());
    _ = forever.await;
  }

  async fn on_refresh(&mut self, emit_output_tx: Sender<ProviderVariables>) {
    T::refresh_and_emit(&emit_output_tx, &self.state()).await;
  }

  async fn on_stop(&mut self) {
    if let Some(handle) = &self.abort_handle() {
      handle.abort();
    }
  }
}
