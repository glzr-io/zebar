use std::{collections::HashMap, sync::Arc};

use sysinfo::{Networks, System};
use tauri::{AppHandle, Emitter};
use tokio::{
  sync::{
    mpsc::{self},
    Mutex,
  },
  task,
};
use tracing::{info, warn};

use super::{
  config::ProviderConfig,
  provider_ref::{ProviderOutput, ProviderRef},
};

/// State shared between providers.
pub struct SharedProviderState {
  pub sysinfo: Arc<Mutex<System>>,
  pub netinfo: Arc<Mutex<Networks>>,
}

/// Manages the creation and cleanup of providers.
pub struct ProviderManager {
  emit_output_tx: mpsc::Sender<ProviderOutput>,
  providers: Arc<Mutex<HashMap<String, ProviderRef>>>,
  shared_state: SharedProviderState,
}

impl ProviderManager {
  pub fn new(app_handle: &AppHandle) -> Self {
    let (emit_output_tx, emit_output_rx) =
      mpsc::channel::<ProviderOutput>(1);

    let providers = Arc::new(Mutex::new(HashMap::new()));
    Self::start_listener(app_handle, emit_output_rx, providers.clone());

    Self {
      emit_output_tx,
      providers,
      shared_state: SharedProviderState {
        sysinfo: Arc::new(Mutex::new(System::new_all())),
        netinfo: Arc::new(Mutex::new(Networks::new_with_refreshed_list())),
      },
    }
  }

  /// Starts listening for provider outputs and emits them to frontend
  /// clients.
  fn start_listener(
    app_handle: &AppHandle,
    mut emit_output_rx: mpsc::Receiver<ProviderOutput>,
    providers: Arc<Mutex<HashMap<String, ProviderRef>>>,
  ) {
    let app_handle = app_handle.clone();

    task::spawn(async move {
      while let Some(output) = emit_output_rx.recv().await {
        info!("Emitting for provider: {}", output.config_hash);

        let output = Box::new(output);

        if let Err(err) = app_handle.emit("provider-emit", output.clone())
        {
          warn!("Error emitting provider output: {:?}", err);
        }

        // Update the provider's output cache.
        if let Ok(mut providers) = providers.try_lock() {
          if let Some(found_provider) =
            providers.get_mut(&output.config_hash)
          {
            found_provider.update_cache(output);
          }
        } else {
          warn!("Failed to update provider output cache.");
        }
      }
    });
  }

  /// Creates a provider with the given config.
  pub async fn create(
    &self,
    config_hash: String,
    config: ProviderConfig,
    _tracked_access: Vec<String>,
  ) -> anyhow::Result<()> {
    let found_provider =
      { self.providers.lock().await.get(&config_hash).cloned() };

    // If a provider with the given config already exists, refresh it
    // and return early.
    if let Some(found_provider) = found_provider {
      if let Err(err) = found_provider.refresh().await {
        warn!("Error refreshing provider: {:?}", err);
      }

      return Ok(());
    };

    let provider_ref = ProviderRef::new(
      config_hash.clone(),
      config,
      self.emit_output_tx.clone(),
      &self.shared_state,
    )?;

    let mut providers = self.providers.lock().await;
    providers.insert(config_hash, provider_ref);

    Ok(())
  }

  /// Destroys and cleans up the provider with the given config.
  pub async fn destroy(&self, config_hash: String) -> anyhow::Result<()> {
    let mut providers = self.providers.lock().await;

    if let Some(found_provider) = providers.get_mut(&config_hash) {
      if let Err(err) = found_provider.stop().await {
        warn!("Error stopping provider: {:?}", err);
      }
    }

    providers.remove(&config_hash);

    Ok(())
  }
}
