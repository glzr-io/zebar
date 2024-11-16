use std::{collections::HashMap, sync::Arc};

use anyhow::Context;
use sysinfo::System;
use tauri::{AppHandle, Emitter};
use tokio::sync::{mpsc, oneshot, Mutex};

use super::{
  ProviderConfig, ProviderEmission, ProviderFunction,
  ProviderFunctionResult, ProviderRef,
};

/// State shared between providers.
#[derive(Clone)]
pub struct SharedProviderState {
  pub sysinfo: Arc<Mutex<System>>,
}

/// Manages the creation and cleanup of providers.
pub struct ProviderManager {
  app_handle: AppHandle,
  provider_refs: Arc<Mutex<HashMap<String, ProviderRef>>>,
  emit_cache: Arc<Mutex<HashMap<String, ProviderEmission>>>,
  shared_state: SharedProviderState,
  emit_tx: mpsc::UnboundedSender<ProviderEmission>,
}

impl ProviderManager {
  /// Creates a new provider manager.
  ///
  /// Returns a tuple containing the manager and a channel for provider
  /// emissions.
  pub fn new(
    app_handle: &AppHandle,
  ) -> (Arc<Self>, mpsc::UnboundedReceiver<ProviderEmission>) {
    let (emit_tx, emit_rx) = mpsc::unbounded_channel::<ProviderEmission>();

    (
      Arc::new(Self {
        app_handle: app_handle.clone(),
        provider_refs: Arc::new(Mutex::new(HashMap::new())),
        emit_cache: Arc::new(Mutex::new(HashMap::new())),
        shared_state: SharedProviderState {
          sysinfo: Arc::new(Mutex::new(System::new_all())),
        },
        emit_tx,
      }),
      emit_rx,
    )
  }

  /// Creates a provider with the given config.
  pub async fn create(
    &self,
    config_hash: &str,
    config: ProviderConfig,
  ) -> anyhow::Result<()> {
    // If a provider with the given config already exists, re-emit its
    // latest emission and return early.
    {
      if let Some(found_emit) =
        self.emit_cache.lock().await.get(config_hash)
      {
        self.app_handle.emit("provider-emit", found_emit);
        return Ok(());
      };
    }

    let provider_ref = ProviderRef::new(
      self.emit_tx.clone(),
      config,
      config_hash.clone(),
      self.shared_state.clone(),
    )
    .await?;

    let mut providers = self.provider_refs.lock().await;
    providers.insert(config_hash, provider_ref);

    Ok(())
  }

  /// Sends a function call through a channel to be executed by the
  /// provider.
  ///
  /// Returns the result of the function execution.
  async fn call_function(
    &self,
    config_hash: &str,
    function: ProviderFunction,
  ) -> anyhow::Result<ProviderFunctionResult> {
    let mut provider_refs = self.provider_refs.lock().await;
    let provider_ref = provider_refs
      .get(config_hash)
      .context("No provider found with config.")?;

    let (tx, rx) = oneshot::channel();
    provider_ref.function_tx.send((function, tx))?;
    rx.await?
  }

  /// Destroys and cleans up the provider with the given config.
  pub async fn destroy(&self, config_hash: &str) -> anyhow::Result<()> {
    let mut provider_refs = self.provider_refs.lock().await;
    let provider_ref = provider_refs
      .remove(config_hash)
      .context("No provider found with config.")?;

    let (tx, rx) = oneshot::channel();
    provider_ref.stop_tx.send(tx)?;
    rx.await?
  }

  /// Updates the cache with the given provider emission.
  pub async fn update_cache(&self, emit: ProviderEmission) {
    let mut cache = self.emit_cache.lock().await;
    cache.insert(emit.config_hash, emit);
  }
}
