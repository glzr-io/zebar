use async_trait::async_trait;
use tokio::sync::mpsc::Sender;

use super::ProviderResult;

#[async_trait]
pub trait Provider: Send + Sync {
  /// Callback for when the provider is started.
  async fn run(&self, emit_result_tx: Sender<ProviderResult>);

  /// Callback for when the provider is stopped.
  async fn on_stop(&self) {
    // No-op by default.
  }
}

/// Implements the `Provider` trait for the given struct.
///
/// Expects that the struct has a `refresh_interval_ms` and `run_interval`
/// method.
#[macro_export]
macro_rules! impl_interval_provider {
  ($type:ty) => {
    #[async_trait::async_trait]
    impl crate::providers::Provider for $type {
      async fn run(
        &self,
        emit_result_tx: tokio::sync::mpsc::Sender<
          crate::providers::ProviderResult,
        >,
      ) {
        let mut interval = tokio::time::interval(
          std::time::Duration::from_millis(self.refresh_interval_ms()),
        );

        loop {
          interval.tick().await;

          let res =
            emit_result_tx.send(self.run_interval().await.into()).await;

          if let Err(err) = res {
            tracing::error!("Error sending provider result: {:?}", err);
          }
        }
      }
    }
  };
}
