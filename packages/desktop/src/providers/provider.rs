use std::time::Duration;

use async_trait::async_trait;
use tokio::sync::mpsc::Sender;

use super::provider_ref::ProviderOutput;

#[async_trait]
pub trait Provider {
  /// Callback for when the provider is started.
  async fn on_start(
    &mut self,
    config_hash: &str,
    emit_output_tx: Sender<ProviderOutput>,
  );

  /// Callback for when the provider is refreshed.
  async fn on_refresh(
    &mut self,
    config_hash: &str,
    emit_output_tx: Sender<ProviderOutput>,
  );

  /// Callback for when the provider is stopped.
  async fn on_stop(&mut self);

  /// Minimum interval between refreshes.
  ///
  /// Affects how the provider output is cached.
  fn min_refresh_interval(&self) -> Option<Duration>;
}
