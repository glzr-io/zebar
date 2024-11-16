use std::{collections::HashMap, sync::Arc};

use anyhow::Context;
use sysinfo::System;
use tauri::{AppHandle, Emitter};
use tokio::{
  sync::{mpsc, Mutex},
  task,
};
use tracing::warn;

use super::{
  ProviderConfig, ProviderEmission, ProviderFunction,
  ProviderFunctionResult, ProviderRef, RuntimeType,
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
    config_hash: String,
    config: ProviderConfig,
  ) -> anyhow::Result<()> {
    // If a provider with the given config already exists, re-emit its
    // latest emission and return early.
    {
      if let Some(found_emit) =
        self.emit_cache.lock().await.get(&config_hash)
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

  /// Calls the given function on the provider with the given config hash.
  async fn call_function(
    &self,
    config_hash: &str,
    function: ProviderFunction,
  ) -> anyhow::Result<ProviderFunctionResult> {
    let mut providers = self.provider_refs.lock().await;
    let provider = providers
      .get_mut(config_hash)
      .context("No provider found with config.")?;

    match provider.runtime_type() {
      RuntimeType::Async => provider.call_async_function(function).await,
      RuntimeType::Sync => {
        task::spawn_blocking(move || provider.call_sync_function(function))
          .await
          .map_err(|err| format!("Function execution failed: {}", err))?
      }
    }
  }

  /// Destroys and cleans up the provider with the given config.
  pub async fn destroy(&self, config_hash: String) -> anyhow::Result<()> {
    let mut providers = self.provider_refs.lock().await;

    if let Some(found_provider) = providers.get_mut(&config_hash) {
      if let Err(err) = found_provider.stop().await {
        warn!("Error stopping provider: {:?}", err);
      }
    }

    providers.remove(&config_hash);

    Ok(())
  }

  /// Updates the cache with the given provider emission.
  pub async fn update_cache(&self, emit: ProviderEmission) {
    let mut cache = self.emit_cache.lock().await;
    cache.insert(emit.config_hash, emit);
  }
}
