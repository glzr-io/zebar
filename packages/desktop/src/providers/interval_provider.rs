use std::{sync::Arc, time::Duration};

use anyhow::Result;
use async_trait::async_trait;
use tokio::{
  sync::mpsc::Sender,
  task::{self, AbortHandle},
  time,
};

use super::{
  manager::{ProviderOutput, VariablesResult},
  provider::Provider,
  variables::ProviderVariables,
};

#[async_trait]
pub trait IntervalProvider {
  type State: Sync + Send + 'static;

  fn refresh_interval_ms(&self) -> u64;

  fn state(&self) -> Arc<Self::State>;

  fn abort_handle(&self) -> &Option<AbortHandle>;

  fn set_abort_handle(&mut self, abort_handle: AbortHandle);

  async fn get_refreshed_variables(
    state: &Self::State,
  ) -> Result<ProviderVariables>;
}

#[async_trait]
impl<T: IntervalProvider + Send> Provider for T {
  async fn on_start(
    &mut self,
    config_hash: String,
    emit_output_tx: Sender<ProviderOutput>,
  ) {
    let refresh_interval_ms = self.refresh_interval_ms();
    let state = self.state();

    let forever = task::spawn(async move {
      let mut interval =
        time::interval(Duration::from_millis(refresh_interval_ms));

      loop {
        // The first tick fires immediately.
        interval.tick().await;

        _ = emit_output_tx
          .send(ProviderOutput {
            config_hash: config_hash.clone(),
            variables: to_variables_result(
              T::get_refreshed_variables(&state).await,
            ),
          })
          .await;
      }
    });

    self.set_abort_handle(forever.abort_handle());
    _ = forever.await;
  }

  async fn on_refresh(
    &mut self,
    config_hash: String,
    emit_output_tx: Sender<ProviderOutput>,
  ) {
    _ = emit_output_tx
      .send(ProviderOutput {
        config_hash,
        variables: to_variables_result(
          T::get_refreshed_variables(&self.state()).await,
        ),
      })
      .await;
  }

  async fn on_stop(&mut self) {
    if let Some(handle) = &self.abort_handle() {
      handle.abort();
    }
  }
}

fn to_variables_result(result: Result<ProviderVariables>) -> VariablesResult {
  match result {
    Ok(variables) => VariablesResult::Data(variables),
    Err(err) => VariablesResult::Error(err.to_string()),
  }
}
