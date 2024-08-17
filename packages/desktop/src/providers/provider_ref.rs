use std::time::{Duration, Instant};

use anyhow::bail;
use serde::Serialize;
use tokio::{sync::mpsc, task};
use tracing::warn;

#[cfg(windows)]
use super::komorebi::KomorebiProvider;
use super::{
  battery::BatteryProvider, config::ProviderConfig, cpu::CpuProvider,
  host::HostProvider, ip::IpProvider, manager::SharedProviderState,
  memory::MemoryProvider, network::NetworkProvider, provider::Provider,
  variables::ProviderVariables, weather::WeatherProvider,
};

/// Reference to an active provider.
#[derive(Debug, Clone)]
pub struct ProviderRef {
  pub config_hash: String,
  pub min_refresh_interval: Duration,
  pub prev_refresh: Option<Instant>,
  pub prev_output: Option<Box<ProviderOutput>>,
  pub emit_output_tx: mpsc::Sender<ProviderOutput>,
  pub refresh_tx: mpsc::Sender<()>,
  pub stop_tx: mpsc::Sender<()>,
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
    let mut provider = Self::create_provider(config, shared_state)?;

    let (refresh_tx, refresh_rx) = mpsc::channel::<()>(1);
    let (stop_tx, stop_rx) = mpsc::channel::<()>(1);

    let min_refresh_interval = provider.min_refresh_interval();
    let config_hash_clone = config_hash.clone();
    let emit_output_tx_clone = emit_output_tx.clone();

    task::spawn(async move {
      provider
        .start(
          &config_hash_clone,
          emit_output_tx_clone,
          refresh_rx,
          stop_rx,
        )
        .await
    });

    Ok(Self {
      config_hash,
      min_refresh_interval,
      prev_refresh: None,
      prev_output: None,
      emit_output_tx,
      refresh_tx,
      stop_tx,
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
    self.prev_refresh = Some(Instant::now());
    self.prev_output = Some(output);
  }

  /// Refreshes the provider.
  ///
  /// Since the previous output of providers is cached, if within the
  /// minimum refresh interval, send the previous output.
  pub async fn refresh(&self) {
    match (self.prev_refresh, self.prev_output.clone()) {
      (Some(prev_refresh), Some(prev_output))
        if prev_refresh.elapsed() < self.min_refresh_interval =>
      {
        _ = self.emit_output_tx.send(*prev_output).await
      }
      _ => _ = self.refresh_tx.send(()).await,
    }
  }

  /// Stops the given provider.
  ///
  /// This triggers any necessary cleanup.
  pub async fn stop(&self) {
    if let Err(err) = self.stop_tx.send(()).await {
      warn!("Error stopping provider: {:?}", err);
    }
  }
}