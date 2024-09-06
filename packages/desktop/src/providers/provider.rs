use std::time::Duration;

use async_trait::async_trait;
use tokio::{sync::mpsc::Sender, time};
use tracing::error;

use super::{provider_ref::ProviderResult, variables::ProviderOutput};

#[async_trait]
pub trait Provider: Send + Sync {
  /// Callback for when the provider is started.
  async fn run(&self, emit_result_tx: Sender<ProviderResult>);

  /// Callback for when the provider is stopped.
  async fn on_stop(&self) {
    // No-op by default.
  }
}

#[async_trait]
pub trait IntervalProvider {
  /// Refresh interval in milliseconds.
  fn refresh_interval_ms(&self) -> u64;

  /// Callback for when the provider is started.
  async fn run_interval(&self) -> anyhow::Result<ProviderOutput>;
}

#[async_trait]
impl<T: IntervalProvider + Send + Sync> Provider for T {
  async fn run(&self, emit_result_tx: Sender<ProviderResult>) {
    let mut interval =
      time::interval(Duration::from_millis(self.refresh_interval_ms()));

    loop {
      interval.tick().await;

      let res =
        emit_result_tx.send(self.run_interval().await.into()).await;

      if let Err(err) = res {
        error!("Error sending provider result: {:?}", err);
      }
    }
  }
}
