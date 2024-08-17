use std::time::Duration;

use async_trait::async_trait;
use tokio::sync::mpsc::Sender;

use super::provider_ref::ProviderOutput;

#[async_trait]
pub trait Provider {
  async fn on_start(
    &mut self,
    config_hash: &str,
    emit_output_tx: Sender<ProviderOutput>,
  );

  async fn on_refresh(
    &mut self,
    config_hash: &str,
    emit_output_tx: Sender<ProviderOutput>,
  );

  async fn on_stop(&mut self);

  // TODO: Change to `Option<Duration>`.
  fn min_refresh_interval(&self) -> Duration;
}
