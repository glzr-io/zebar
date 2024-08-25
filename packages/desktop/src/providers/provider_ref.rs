use std::time::{Duration, Instant};

use anyhow::bail;
use serde::Serialize;
use tokio::{sync::mpsc, task};
use tracing::info;

#[cfg(windows)]
use super::komorebi::KomorebiProvider;
#[cfg(windows)]
use super::language::LanguageProvider;
use super::{
  battery::BatteryProvider, config::ProviderConfig, cpu::CpuProvider,
  host::HostProvider, ip::IpProvider, memory::MemoryProvider,
  network::NetworkProvider, provider::Provider,
  provider_manager::SharedProviderState, variables::ProviderVariables,
  weather::WeatherProvider,
};

/// Reference to an active provider.
#[derive(Debug, Clone)]
pub struct ProviderRef {
  pub config_hash: String,
  pub min_refresh_interval: Option<Duration>,
  pub cache: Option<ProviderCache>,
  pub emit_output_tx: mpsc::Sender<ProviderOutput>,
  pub refresh_tx: mpsc::Sender<()>,
  pub stop_tx: mpsc::Sender<()>,
}

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
  pub fn new(
    config_hash: String,
    config: ProviderConfig,
    emit_output_tx: mpsc::Sender<ProviderOutput>,
    shared_state: &SharedProviderState,
  ) -> anyhow::Result<Self> {
    let provider = Self::create_provider(config, shared_state)?;

    let (refresh_tx, refresh_rx) = mpsc::channel::<()>(1);
    let (stop_tx, stop_rx) = mpsc::channel::<()>(1);

    let min_refresh_interval = provider.min_refresh_interval();
    let config_hash_clone = config_hash.clone();
    let emit_output_tx_clone = emit_output_tx.clone();

    task::spawn(async move {
      Self::start_provider(
        provider,
        config_hash_clone,
        emit_output_tx_clone,
        refresh_rx,
        stop_rx,
      )
      .await;
    });

    Ok(Self {
      config_hash,
      min_refresh_interval,
      cache: None,
      emit_output_tx,
      refresh_tx,
      stop_tx,
    })
  }

  /// Starts the provider.
  async fn start_provider(
    mut provider: Box<dyn Provider + Send>,
    config_hash: String,
    emit_output_tx: mpsc::Sender<ProviderOutput>,
    mut refresh_rx: mpsc::Receiver<()>,
    mut stop_rx: mpsc::Receiver<()>,
  ) {
    let mut has_started = false;

    // Loop to avoid exiting the select on refresh.
    loop {
      let config_hash = config_hash.clone();
      let emit_output_tx = emit_output_tx.clone();

      tokio::select! {
        // Default match arm which handles initialization of the provider.
        // This has a precondition to avoid running again on refresh.
        _ = {
          info!("Starting provider: {}", config_hash);
          has_started = true;
          provider.on_start(&config_hash, emit_output_tx.clone())
        }, if !has_started => break,

        // On refresh, re-emit provider variables and continue looping.
        Some(_) = refresh_rx.recv() => {
          info!("Refreshing provider: {}", config_hash);
          _ = provider.on_refresh(&config_hash, emit_output_tx).await;
        },

        // On stop, perform any necessary clean up and exit the loop.
        Some(_) = stop_rx.recv() => {
          info!("Stopping provider: {}", config_hash);
          _ = provider.on_stop().await;
          break;
        },
      }
    }

    info!("Provider stopped: {}", config_hash);
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
      #[cfg(windows)]
      ProviderConfig::Language(config) => {
        Box::new(LanguageProvider::new(config))
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
  pub async fn refresh(&self) -> anyhow::Result<()> {
    let min_refresh_interval =
      self.min_refresh_interval.unwrap_or(Duration::MAX);

    match &self.cache {
      Some(cache) if cache.timestamp.elapsed() >= min_refresh_interval => {
        self.emit_output_tx.send(*cache.output.clone()).await?;
      }
      _ => self.refresh_tx.send(()).await?,
    };

    Ok(())
  }

  /// Stops the given provider.
  ///
  /// This triggers any necessary cleanup.
  pub async fn stop(&self) -> anyhow::Result<()> {
    self.stop_tx.send(()).await?;

    Ok(())
  }
}
