use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::mpsc::Sender;

use super::{
  provider_manager::SharedProviderState, provider_ref::VariablesResult,
};

#[async_trait]
pub trait Provider: Send + Sync {
  /// Callback for when the provider is started.
  async fn on_start(
    self: Arc<Self>,
    shared_state: SharedProviderState,
    emit_output_tx: Sender<VariablesResult>,
  );

  /// Callback for when the provider is stopped.
  async fn on_stop(self: Arc<Self>) {
    // No-op by default.
  }
}
