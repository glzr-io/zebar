use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use tokio::{
  sync::mpsc::Sender,
  task::{self, AbortHandle},
  time,
};

use super::{
  provider::Provider, provider_ref::ProviderOutput,
  variables::ProviderVariables,
};

/// Require interval providers to have a refresh interval in their config.
pub trait IntervalConfig {
  fn refresh_interval(&self) -> u64;
}

#[macro_export]
macro_rules! impl_interval_config {
  ($struct_name:ident) => {
    use crate::providers::interval_provider::IntervalConfig;

    impl IntervalConfig for $struct_name {
      fn refresh_interval(&self) -> u64 {
        self.refresh_interval
      }
    }
  };
}

#[async_trait]
pub trait IntervalProvider {
  type Config: Sync + Send + 'static + IntervalConfig;
  type State: Sync + Send + 'static;

  /// Default to 2 seconds as the minimum refresh interval.
  fn min_refresh_interval(&self) -> Duration {
    Duration::from_secs(2)
  }

  fn config(&self) -> Arc<Self::Config>;

  fn state(&self) -> Arc<Self::State>;

  fn abort_handle(&self) -> &Option<AbortHandle>;

  fn set_abort_handle(&mut self, abort_handle: AbortHandle);

  async fn get_refreshed_variables(
    config: &Self::Config,
    state: &Self::State,
  ) -> anyhow::Result<ProviderVariables>;
}

#[async_trait]
impl<T: IntervalProvider + Send> Provider for T {
  fn min_refresh_interval(&self) -> Duration {
    T::min_refresh_interval(self)
  }

  async fn on_start(
    &mut self,
    config_hash: &str,
    emit_output_tx: Sender<ProviderOutput>,
  ) {
    let config = self.config();
    let state = self.state();
    let config_hash = config_hash.to_string();

    let interval_task = task::spawn(async move {
      let mut interval =
        time::interval(Duration::from_millis(config.refresh_interval()));

      loop {
        // The first tick fires immediately.
        interval.tick().await;

        _ = emit_output_tx
          .send(ProviderOutput {
            config_hash: config_hash.clone(),
            variables: T::get_refreshed_variables(&config, &state)
              .await
              .into(),
          })
          .await;
      }
    });

    self.set_abort_handle(interval_task.abort_handle());
    _ = interval_task.await;
  }

  async fn on_refresh(
    &mut self,
    config_hash: &str,
    emit_output_tx: Sender<ProviderOutput>,
  ) {
    _ = emit_output_tx
      .send(ProviderOutput {
        config_hash: config_hash.to_string(),
        variables: T::get_refreshed_variables(
          &self.config(),
          &self.state(),
        )
        .await
        .into(),
      })
      .await;
  }

  async fn on_stop(&mut self) {
    if let Some(handle) = &self.abort_handle() {
      handle.abort();
    }
  }
}
