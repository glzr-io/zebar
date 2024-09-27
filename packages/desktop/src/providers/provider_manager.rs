use std::{collections::HashMap, sync::Arc};

use sysinfo::{Networks, System};
use tauri::AppHandle;
use tokio::sync::Mutex;
use tracing::warn;

use super::{ProviderConfig, ProviderRef};

/// State shared between providers.
#[derive(Clone)]
pub struct SharedProviderState {
  pub sysinfo: Arc<Mutex<System>>,
  pub netinfo: Arc<Mutex<Networks>>,
}

/// Manages the creation and cleanup of providers.
pub struct ProviderManager {
  app_handle: AppHandle,
  providers: Arc<Mutex<HashMap<String, ProviderRef>>>,
  shared_state: SharedProviderState,
}

impl ProviderManager {
  pub fn new(app_handle: &AppHandle) -> Self {
    Self {
      app_handle: app_handle.clone(),
      providers: Arc::new(Mutex::new(HashMap::new())),
      shared_state: SharedProviderState {
        sysinfo: Arc::new(Mutex::new(System::new_all())),
        netinfo: Arc::new(Mutex::new(Networks::new_with_refreshed_list())),
      },
    }
  }

  /// Creates a provider with the given config.
  pub async fn create(
    &self,
    config_hash: String,
    config: ProviderConfig,
  ) -> anyhow::Result<()> {
    {
      let mut providers = self.providers.lock().await;

      // If a provider with the given config already exists, refresh it
      // and return early.
      if let Some(found_provider) = providers.get_mut(&config_hash) {
        if let Err(err) = found_provider.refresh().await {
          warn!("Error refreshing provider: {:?}", err);
        }

        return Ok(());
      };
    }

    let provider_ref = ProviderRef::new(
      &self.app_handle,
      config,
      config_hash.clone(),
      self.shared_state.clone(),
    )
    .await?;

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
