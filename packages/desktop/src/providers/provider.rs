use async_trait::async_trait;

use super::{ProviderFunction, ProviderFunctionResult};

#[async_trait]
pub trait Provider: Send + Sync {
  fn runtime_type(&self) -> RuntimeType;

  /// Callback for when the provider is started.
  ///
  /// # Panics
  ///
  /// Panics if wrong runtime type is used.
  fn start_sync(&mut self) {
    match self.runtime_type() {
      RuntimeType::Sync => {
        unreachable!("Sync providers must implement `start_sync`.")
      }
      RuntimeType::Async => {
        panic!("Cannot call sync function on async provider.")
      }
    }
  }

  /// Callback for when the provider is started.
  ///
  /// # Panics
  ///
  /// Panics if wrong runtime type is used.
  async fn start_async(&mut self) {
    match self.runtime_type() {
      RuntimeType::Async => {
        unreachable!("Async providers must implement `start_async`.")
      }
      RuntimeType::Sync => {
        panic!("Cannot call async function on sync provider.")
      }
    }
  }

  /// Runs the given function.
  ///
  /// # Panics
  ///
  /// Panics if wrong runtime type is used.
  fn call_function_sync(
    &self,
    #[allow(unused_variables)] function: ProviderFunction,
  ) -> anyhow::Result<ProviderFunctionResult> {
    match self.runtime_type() {
      RuntimeType::Sync => {
        unreachable!("Sync providers must implement `call_function_sync`.")
      }
      RuntimeType::Async => {
        panic!("Cannot call sync function on async provider.")
      }
    }
  }

  /// Runs the given function.
  ///
  /// # Panics
  ///
  /// Panics if wrong runtime type is used.
  async fn call_function_async(
    &self,
    #[allow(unused_variables)] function: ProviderFunction,
  ) -> anyhow::Result<ProviderFunctionResult> {
    match self.runtime_type() {
      RuntimeType::Async => {
        unreachable!(
          "Async providers must implement `call_function_async`."
        )
      }
      RuntimeType::Sync => {
        panic!("Cannot call async function on sync provider.")
      }
    }
  }
}

/// Determines whether `start_sync` or `start_async` is called.
pub enum RuntimeType {
  Sync,
  Async,
}

/// Implements the `Provider` trait for the given struct.
///
/// Expects that the struct has a `refresh_interval_ms` and `run_interval`
/// method.
#[macro_export]
macro_rules! impl_interval_provider {
  ($type:ty, $allow_identical_emits:expr) => {
    #[async_trait::async_trait]
    impl crate::providers::Provider for $type {
      fn runtime_type(&self) -> crate::providers::RuntimeType {
        crate::providers::RuntimeType::Async
      }

      async fn start_async(&mut self) {
        let mut interval = tokio::time::interval(
          std::time::Duration::from_millis(self.refresh_interval_ms()),
        );

        // Skip missed ticks when the interval runs. This prevents a burst
        // of backlogged ticks after a delay.
        interval
          .set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

        let mut last_interval_res: Option<
          crate::providers::ProviderEmission,
        > = None;

        loop {
          interval.tick().await;

          let interval_res = self.run_interval().await.into();

          if $allow_identical_emits
            || last_interval_res.as_ref() != Some(&interval_res)
          {
            let send_res = emit_result_tx.send(interval_res.clone()).await;

            if let Err(err) = send_res {
              tracing::error!("Error sending provider result: {:?}", err);
            }

            last_interval_res = Some(interval_res);
          }
        }
      }
    }
  };
}
