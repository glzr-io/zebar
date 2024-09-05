use std::{sync::Arc, time::Instant};

use anyhow::bail;
use serde::Serialize;
use serde_json::json;
use tauri::{AppHandle, Emitter};
use tokio::{sync::mpsc, task};
use tracing::{info, warn};

#[cfg(windows)]
use super::komorebi::KomorebiProvider;
use super::{
  battery::BatteryProvider, config::ProviderConfig, cpu::CpuProvider,
  host::HostProvider, ip::IpProvider, memory::MemoryProvider,
  network::NetworkProvider, provider::Provider,
  provider_manager::SharedProviderState, variables::ProviderOutput,
  weather::WeatherProvider,
};

/// Reference to an active provider.
pub struct ProviderRef {
  pub config_hash: String,
  pub cache: Option<ProviderCache>,
  provider: Arc<dyn Provider>,
  emit_result_tx: mpsc::Sender<ProviderResult>,
}

/// Cache for provider output.
#[derive(Debug, Clone)]
pub struct ProviderCache {
  pub timestamp: Instant,
  pub output: Box<ProviderOutput>,
}

/// Provider output/error emitted to frontend clients.
///
/// This is used instead of a normal `Result` type in order to serialize it
/// in a nicer way.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ProviderResult {
  Output(ProviderOutput),
  Error(String),
}

/// Implements conversion from `anyhow::Result`.
impl From<anyhow::Result<ProviderOutput>> for ProviderResult {
  fn from(result: anyhow::Result<ProviderOutput>) -> Self {
    match result {
      Ok(output) => ProviderResult::Output(output),
      Err(err) => ProviderResult::Error(err.to_string()),
    }
  }
}

impl ProviderRef {
  /// Creates a new `ProviderRef` instance.
  pub async fn new(
    app_handle: &AppHandle,
    config_hash: String,
    config: ProviderConfig,
    shared_state: &SharedProviderState,
  ) -> anyhow::Result<Self> {
    let provider = Self::create_provider(config, shared_state)?;

    let (emit_result_tx, mut emit_result_rx) =
      mpsc::channel::<ProviderResult>(1);

    let config_hash_clone = config_hash.clone();
    let app_handle = app_handle.clone();
    task::spawn(async move {
      while let Some(output) = emit_result_rx.recv().await {
        info!("Emitting for provider: {}", config_hash_clone);

        let output = Box::new(output);
        let payload = json!({
          "config_hash": config_hash_clone.clone(),
          "result": *output.clone(),
        });

        if let Err(err) = app_handle.emit("provider-emit", payload) {
          warn!("Error emitting provider output: {:?}", err);
        }

        // Update the provider's output cache.
        // if let Ok(mut providers) = providers.try_lock() {
        //   if let Some(found_provider) =
        //     providers.get_mut(&output.config_hash)
        //   {
        //     found_provider.update_cache(output);
        //   }
        // } else {
        //   warn!("Failed to update provider output cache.");
        // }
      }
    });

    info!("Starting provider: {}", config_hash);
    let provider_clone = provider.clone();
    let emit_result_tx_clone = emit_result_tx.clone();
    task::spawn(async move {
      provider_clone.on_start(emit_result_tx_clone).await;
    });

    Ok(Self {
      config_hash,
      cache: None,
      provider,
      emit_result_tx,
    })
  }

  fn create_provider(
    config: ProviderConfig,
    shared_state: &SharedProviderState,
  ) -> anyhow::Result<Arc<dyn Provider>> {
    let provider: Arc<dyn Provider> = match config {
      ProviderConfig::Battery(config) => {
        Arc::new(BatteryProvider::new(config)?)
      }
      ProviderConfig::Cpu(config) => {
        Arc::new(CpuProvider::new(config, shared_state.sysinfo.clone()))
      }
      ProviderConfig::Host(config) => {
        Arc::new(HostProvider::new(config, shared_state.sysinfo.clone()))
      }
      ProviderConfig::Ip(config) => Arc::new(IpProvider::new(config)),
      #[cfg(windows)]
      ProviderConfig::Komorebi(config) => {
        Arc::new(KomorebiProvider::new(config))
      }
      ProviderConfig::Memory(config) => {
        Arc::new(MemoryProvider::new(config, shared_state.sysinfo.clone()))
      }
      ProviderConfig::Network(config) => Arc::new(NetworkProvider::new(
        config,
        shared_state.netinfo.clone(),
      )),
      ProviderConfig::Weather(config) => {
        Arc::new(WeatherProvider::new(config))
      }
      #[allow(unreachable_patterns)]
      _ => bail!("Provider not supported on this operating system."),
    };

    Ok(provider)
  }

  /// Updates cache with the given output.
  pub fn update_cache(&mut self, output: Box<ProviderOutput>) {
    self.cache = Some(ProviderCache {
      timestamp: Instant::now(),
      output,
    });
  }

  /// Refreshes the provider.
  ///
  /// Since the previous output of providers is cached, if within the
  /// minimum refresh interval, send the previous output.
  pub async fn refresh(&mut self) -> anyhow::Result<()> {
    // if let Some(cache) = self.cache {
    //   self.emit_result_tx.send(*cache.output.clone()).await?;
    // }

    Ok(())
  }

  /// Stops the given provider.
  ///
  /// This triggers any necessary cleanup.
  pub async fn stop(&mut self) -> anyhow::Result<()> {
    info!("Stopping provider: {}", self.config_hash);
    _ = self.provider.clone().on_stop().await;
    info!("Provider stopped: {}", self.config_hash);

    Ok(())
  }
}
