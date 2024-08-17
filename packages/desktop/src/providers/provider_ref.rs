use std::{
  sync::Arc,
  time::{Duration, Instant},
};

use anyhow::bail;
use serde::Serialize;
use sysinfo::{Networks, System};
use tokio::sync::{mpsc, Mutex};
use tracing::warn;

#[cfg(windows)]
use super::komorebi::KomorebiProvider;
use super::{
  battery::BatteryProvider, config::ProviderConfig, cpu::CpuProvider,
  host::HostProvider, ip::IpProvider, memory::MemoryProvider,
  network::NetworkProvider, provider::Provider,
  variables::ProviderVariables, weather::WeatherProvider,
};

/// Reference to an active provider.
#[derive(Debug, Clone)]
pub struct ProviderRef {
  config_hash: String,
  min_refresh_interval: Duration,
  prev_refresh: Option<Instant>,
  prev_output: Option<Box<ProviderOutput>>,
  refresh_tx: mpsc::Sender<()>,
  stop_tx: mpsc::Sender<()>,
}

/// Output sent via IPC to the frontend.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProviderOutput {
  pub config_hash: String,
  pub variables: VariablesResult,
}

/// Result from a provider.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum VariablesResult {
  Data(ProviderVariables),
  Error(String),
}

impl ProviderRef {
  pub fn new() -> Self {}

  fn create_provider(
    config: ProviderConfig,
    sysinfo: Arc<Mutex<System>>,
    netinfo: Arc<Mutex<Networks>>,
  ) -> anyhow::Result<Box<dyn Provider + Send>> {
    let provider: Box<dyn Provider + Send> = match config {
      ProviderConfig::Battery(config) => {
        Box::new(BatteryProvider::new(config)?)
      }
      ProviderConfig::Cpu(config) => {
        Box::new(CpuProvider::new(config, sysinfo))
      }
      ProviderConfig::Host(config) => {
        Box::new(HostProvider::new(config, sysinfo))
      }
      ProviderConfig::Ip(config) => Box::new(IpProvider::new(config)),
      #[cfg(all(windows, target_arch = "x86_64"))]
      ProviderConfig::Komorebi(config) => {
        Box::new(KomorebiProvider::new(config))
      }
      ProviderConfig::Memory(config) => {
        Box::new(MemoryProvider::new(config, sysinfo))
      }
      ProviderConfig::Network(config) => {
        Box::new(NetworkProvider::new(config, netinfo))
      }
      ProviderConfig::Weather(config) => {
        Box::new(WeatherProvider::new(config))
      }
      #[allow(unreachable_patterns)]
      _ => bail!("Provider not supported on this operating system."),
    };

    Ok(provider)
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
