use async_trait::async_trait;

use super::{ProviderFunction, ProviderFunctionResponse};

#[async_trait]
pub trait Provider {
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
    function: ProviderFunction,
  ) -> anyhow::Result<ProviderFunctionResponse> {
    let _function = function;
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
    function: ProviderFunction,
  ) -> anyhow::Result<ProviderFunctionResponse> {
    let _function = function;
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
