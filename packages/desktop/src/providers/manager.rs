use std::{
  collections::HashMap,
  sync::Arc,
  time::{Duration, Instant},
};

use anyhow::bail;
use serde::Serialize;
use sysinfo::{Networks, System};
use tauri::{App, Emitter, Manager, Runtime};
use tokio::{
  sync::{
    mpsc::{self, Sender},
    Mutex,
  },
  task,
};
use tracing::{info, warn};

use super::{
  battery::BatteryProvider, config::ProviderConfig, cpu::CpuProvider,
  host::HostProvider, ip::IpProvider, memory::MemoryProvider,
  network::NetworkProvider, provider_ref::ProviderRef,
  variables::ProviderVariables, weather::WeatherProvider,
};
use crate::providers::provider::Provider;

// TODO: Move to separate `commands` module.
pub struct ListenProviderArgs {
  pub config_hash: String,
  pub config: ProviderConfig,
  pub tracked_access: Vec<String>,
}

// TODO: Move to separate `commands` module.
pub struct UnlistenProviderArgs {
  pub config_hash: String,
}

/// Wrapper around the creation and deletion of providers.
pub struct ProviderManager {
  // app_handle: AppHandle<R>,
  providers: Arc<Mutex<HashMap<String, ProviderRef>>>,
  sysinfo: Arc<Mutex<System>>,
  netinfo: Arc<Mutex<Networks>>,
}

/// Initializes `ProviderManager` in Tauri state.
pub fn init<R: Runtime>(app: &mut App<R>) {
  // app.manage(ProviderManager::new(app.handle().clone()));
  app.manage(ProviderManager::new());
}

impl ProviderManager {
  pub fn new() -> Self {
    ProviderManager {
      providers: Arc::new(Mutex::new(HashMap::new())),
      sysinfo: Arc::new(Mutex::new(System::new_all())),
      netinfo: Arc::new(Mutex::new(Networks::new_with_refreshed_list())),
    }
  }

  /// Create a channel for outputting provider variables to client.
  fn handle_provider_emit_output<R: Runtime>(
    app_handle: (impl Emitter<R> + Sync + Send + 'static),
    active_providers: Arc<Mutex<Vec<ProviderRef>>>,
  ) -> Sender<ProviderOutput> {
    let (output_sender, mut output_receiver) =
      mpsc::channel::<ProviderOutput>(1);

    task::spawn(async move {
      while let Some(output) = output_receiver.recv().await {
        info!("Emitting for provider: {}", output.config_hash);
        let output = Box::new(output);

        if let Err(err) = app_handle.emit("provider-emit", output.clone())
        {
          warn!("Error emitting provider output: {:?}", err);
        }

        if let Ok(mut providers) = active_providers.try_lock() {
          // Find provider that matches given config hash.
          let found_provider = providers
            .iter_mut()
            .find(|provider| *provider.config_hash == output.config_hash);

          if let Some(found) = found_provider {
            found.prev_refresh = Some(Instant::now());
            found.prev_output = Some(output);
          }
        } else {
          warn!("Failed to update provider output cache.");
        }
      }
    });

    output_sender
  }

  /// Creates a provider with the given config.
  pub async fn listen(
    &self,
    config_hash: String,
    config: ProviderConfig,
    tracked_access: Vec<String>,
  ) -> anyhow::Result<()> {
    let found_provider = { self.providers.lock().await.get(&config_hash) };

    // If a provider with the given config already exists, refresh it
    // and return early.
    if let Some(found) = found_provider {
      // The previous output of providers is cached. If within the
      // minimum refresh interval, send the previous output.
      match (found.prev_refresh, found.prev_output.clone()) {
        (Some(prev_refresh), Some(prev_output))
          if prev_refresh.elapsed() < found.min_refresh_interval =>
        {
          _ = emit_output_tx.send(*prev_output).await
        }
        _ => _ = found.refresh_tx.send(()).await,
      }

      return Ok(());
    };

    let (refresh_tx, refresh_rx) = mpsc::channel::<()>(1);
    let (stop_tx, stop_rx) = mpsc::channel::<()>(1);
    let emit_output_tx = emit_output_tx.clone();

    // Attempt to create a new provider.
    let new_provider = Self::create_provider(
      input.config,
      sysinfo.clone(),
      netinfo.clone(),
    );

    if let Err(err) = new_provider {
      _ = emit_output_tx
        .send(ProviderOutput {
          config_hash: input.config_hash,
          variables: VariablesResult::Error(err.to_string()),
        })
        .await;

      continue;
    } else if let Ok(mut new_provider) = new_provider {
      let min_refresh_interval = new_provider.min_refresh_interval();

      let mut providers = active_providers.lock().await;
      providers.push(ProviderRef {
        config_hash: input.config_hash.clone(),
        min_refresh_interval,
        prev_refresh: None,
        prev_output: None,
        refresh_tx,
        stop_tx,
      });

      task::spawn(async move {
        new_provider
          .start(input.config_hash, emit_output_tx, refresh_rx, stop_rx)
          .await
      });
    }

    Ok(())
  }

  /// Destroys and cleans up a provider with the given config.
  pub async fn unlisten(&self, config_hash: String) -> anyhow::Result<()> {
    let mut providers = self.providers.lock().await;

    if let Some(provider) = providers.get_mut(&config_hash) {
      provider.stop().await;
    }

    providers.remove(&config_hash);

    Ok(())
  }
}
