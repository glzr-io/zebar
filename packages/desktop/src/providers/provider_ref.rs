use std::time::{Duration, Instant};

use anyhow::bail;
use serde::Serialize;
use tokio::sync::mpsc;
use tracing::info;

#[cfg(windows)]
use super::komorebi::KomorebiProvider;
use super::{
  battery::BatteryProvider, config::ProviderConfig, cpu::CpuProvider,
  host::HostProvider, ip::IpProvider, memory::MemoryProvider,
  network::NetworkProvider, provider::Provider,
  provider_manager::SharedProviderState, variables::ProviderVariables,
  weather::WeatherProvider,
};

/// Reference to an active provider.
pub struct ProviderRef {
  pub config_hash: String,
  pub min_refresh_interval: Option<Duration>,
  pub cache: Option<ProviderCache>,
  pub emit_output_tx: mpsc::Sender<ProviderOutput>,
  provider: Box<dyn Provider + Send>,
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
    config_hash: String,
    config: ProviderConfig,
    emit_output_tx: mpsc::Sender<ProviderOutput>,
    shared_state: &SharedProviderState,
  ) -> anyhow::Result<Self> {
    let mut provider = Self::create_provider(config, shared_state)?;
    let min_refresh_interval = provider.min_refresh_interval();

    info!("Starting provider: {}", config_hash);
    provider
      .on_start(&config_hash, emit_output_tx.clone())
      .await;

    Ok(Self {
      config_hash,
      min_refresh_interval,
      cache: None,
      emit_output_tx,
      provider,
    })
  }

  fn create_provider(
    config: ProviderConfig,
    shared_state: &SharedProviderState,
  ) -> anyhow::Result<Box<dyn Provider + Send>> {
    let provider: Box<dyn Provider + Send> = match config {
      ProviderConfig::Battery(config) => {
        Box::new(BatteryProvider::new(config)?)
      }
      ProviderConfig::Cpu(config) => {
        Box::new(CpuProvider::new(config, shared_state.sysinfo.clone()))
      }
      ProviderConfig::Host(config) => {
        Box::new(HostProvider::new(config, shared_state.sysinfo.clone()))
      }
      ProviderConfig::Ip(config) => Box::new(IpProvider::new(config)),
      #[cfg(windows)]
      ProviderConfig::Komorebi(config) => {
        Box::new(KomorebiProvider::new(config))
      }
      ProviderConfig::Memory(config) => {
        Box::new(MemoryProvider::new(config, shared_state.sysinfo.clone()))
      }
      ProviderConfig::Network(config) => Box::new(NetworkProvider::new(
        config,
        shared_state.netinfo.clone(),
      )),
      ProviderConfig::Weather(config) => {
        Box::new(WeatherProvider::new(config))
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
    let min_refresh_interval =
      self.min_refresh_interval.unwrap_or(Duration::MAX);

    match &self.cache {
      Some(cache) if cache.timestamp.elapsed() >= min_refresh_interval => {
        self.emit_output_tx.send(*cache.output.clone()).await?;
      }
      _ => {
        info!("Refreshing provider: {}", self.config_hash);
        self
          .provider
          .on_refresh(&self.config_hash, self.emit_output_tx.clone())
          .await;
      }
    };

    Ok(())
  }

  /// Stops the given provider.
  ///
  /// This triggers any necessary cleanup.
  pub async fn stop(&mut self) -> anyhow::Result<()> {
    info!("Stopping provider: {}", self.config_hash);
    _ = self.provider.on_stop().await;
    info!("Provider stopped: {}", self.config_hash);

    Ok(())
  }
}
