use std::{
  sync::Arc,
  time::{Duration, Instant},
};

use anyhow::bail;
use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tokio::{sync::mpsc, task};
use tracing::{info, warn};

// #[cfg(windows)]
// use super::komorebi::KomorebiProvider;
use super::{
  battery::BatteryProvider,
  config::ProviderConfig,
  cpu::CpuProvider,
  // host::HostProvider, ip::IpProvider, memory::MemoryProvider,
  // network::NetworkProvider,
  provider::Provider,
  provider_manager::SharedProviderState,
  variables::ProviderVariables,
  // weather::WeatherProvider,
};

/// Reference to an active provider.
pub struct ProviderRef {
  pub config_hash: String,
  pub cache: Option<ProviderCache>,
  provider: Arc<dyn Provider>,
  emit_output_tx: mpsc::Sender<VariablesResult>,
}

/// Cache for provider output.
#[derive(Debug, Clone)]
pub struct ProviderCache {
  pub timestamp: Instant,
  pub output: Box<ProviderOutput>,
}

/// Output emitted to frontend clients.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProviderOutput {
  pub config_hash: String,
  pub variables: VariablesResult,
}

/// Provider variable output emitted to frontend clients.
///
/// This is used instead of a normal `Result` type to serialize it in a
/// nicer way.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum VariablesResult {
  Data(ProviderVariables),
  Error(String),
}

/// Implements conversion from an `anyhow::Result`.
impl From<anyhow::Result<ProviderVariables>> for VariablesResult {
  fn from(result: anyhow::Result<ProviderVariables>) -> Self {
    match result {
      Ok(data) => VariablesResult::Data(data),
      Err(err) => VariablesResult::Error(err.to_string()),
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

    let (emit_output_tx, mut emit_output_rx) =
      mpsc::channel::<VariablesResult>(1);

    let config_hash_clone = config_hash.clone();
    let app_handle = app_handle.clone();
    task::spawn(async move {
      while let Some(output) = emit_output_rx.recv().await {
        info!("Emitting for provider: {}", config_hash_clone);

        let output = Box::new(output);
        let xx = ProviderOutput {
          config_hash: config_hash_clone.clone(),
          variables: *output.clone(),
        };

        if let Err(err) = app_handle.emit("provider-emit", xx) {
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
    let shared_state = shared_state.clone();
    let emit_output_tx_clone = emit_output_tx.clone();
    task::spawn(async move {
      provider_clone
        .on_start(shared_state.clone(), emit_output_tx_clone)
        .await;
    });

    Ok(Self {
      config_hash,
      cache: None,
      provider,
      emit_output_tx,
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
      ProviderConfig::Cpu(config) => Arc::new(CpuProvider::new(config)),
      // ProviderConfig::Host(config) => {
      //   Box::new(HostProvider::new(config,
      // shared_state.sysinfo.clone())) }
      // ProviderConfig::Ip(config) => Box::new(IpProvider::new(config)),
      // #[cfg(windows)]
      // ProviderConfig::Komorebi(config) => {
      //   Box::new(KomorebiProvider::new(config))
      // }
      // ProviderConfig::Memory(config) => {
      //   Box::new(MemoryProvider::new(config,
      // shared_state.sysinfo.clone())) }
      // ProviderConfig::Network(config) => Box::new(NetworkProvider::new(
      //   config,
      //   shared_state.netinfo.clone(),
      // )),
      // ProviderConfig::Weather(config) => {
      //   Box::new(WeatherProvider::new(config))
      // }
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
    //   self.emit_output_tx.send(*cache.output.clone()).await?;
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
