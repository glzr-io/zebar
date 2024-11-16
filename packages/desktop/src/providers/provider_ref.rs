use std::sync::Arc;

use anyhow::bail;
use serde::Serialize;
use tokio::{
  sync::{
    mpsc::{self, UnboundedSender},
    Mutex,
  },
  task,
};
use tracing::info;

#[cfg(windows)]
use super::{
  audio::AudioProvider, keyboard::KeyboardProvider,
  komorebi::KomorebiProvider, media::MediaProvider,
};
use super::{
  battery::BatteryProvider, cpu::CpuProvider, disk::DiskProvider,
  host::HostProvider, ip::IpProvider, memory::MemoryProvider,
  network::NetworkProvider, weather::WeatherProvider, Provider,
  ProviderConfig, ProviderOutput, RuntimeType, SharedProviderState,
};

/// Reference to an active provider.
pub struct ProviderRef {
  /// Cache for provider output.
  cache: Arc<Mutex<Option<Box<ProviderEmission>>>>,

  /// Sender channel for emitting provider output/error to frontend
  /// clients.
  emit_result_tx: mpsc::UnboundedSender<ProviderEmission>,

  /// Sender channel for stopping the provider.
  stop_tx: mpsc::Sender<()>,
}

/// Provider output/error emitted to frontend clients.
///
/// This is used instead of a normal `Result` type in order to serialize it
/// in a nicer way.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ProviderEmission {
  Output(ProviderOutput),
  Error(String),
}

/// Implements conversion from `anyhow::Result`.
impl From<anyhow::Result<ProviderOutput>> for ProviderEmission {
  fn from(result: anyhow::Result<ProviderOutput>) -> Self {
    match result {
      Ok(output) => ProviderEmission::Output(output),
      Err(err) => ProviderEmission::Error(err.to_string()),
    }
  }
}

impl ProviderRef {
  /// Creates a new `ProviderRef` instance.
  pub async fn new(
    emit_result_tx: UnboundedSender<ProviderEmission>,
    config: ProviderConfig,
    config_hash: String,
    shared_state: SharedProviderState,
  ) -> anyhow::Result<Self> {
    let cache = Arc::new(Mutex::new(None));

    let (stop_tx, stop_rx) = mpsc::channel::<()>(1);

    Self::start_provider(
      config,
      config_hash,
      shared_state,
      emit_result_tx.clone(),
      stop_rx,
    )?;

    Ok(Self {
      cache,
      emit_result_tx,
      stop_tx,
    })
  }

  /// Starts the provider in a separate task.
  fn start_provider(
    config: ProviderConfig,
    config_hash: String,
    shared_state: SharedProviderState,
    emit_result_tx: mpsc::UnboundedSender<ProviderEmission>,
    mut stop_rx: mpsc::Receiver<()>,
  ) -> anyhow::Result<()> {
    let mut provider = Self::create_provider(config, shared_state)?;

    let _ = match provider.runtime_type() {
      RuntimeType::Async => task::spawn(async move {
        provider.start_async(emit_result_tx, stop_rx).await;
        info!("Provider stopped: {}", config_hash);
      }),
      RuntimeType::Sync => task::spawn_blocking(move || {
        provider.start_sync(emit_result_tx, stop_rx);
        info!("Provider stopped: {}", config_hash);
      }),
    };

    Ok(())
  }

  fn create_provider(
    config: ProviderConfig,
    shared_state: SharedProviderState,
  ) -> anyhow::Result<Box<dyn Provider>> {
    let provider: Box<dyn Provider> = match config {
      #[cfg(windows)]
      ProviderConfig::Audio(config) => {
        Box::new(AudioProvider::new(config))
      }
      ProviderConfig::Battery(config) => {
        Box::new(BatteryProvider::new(config))
      }
      ProviderConfig::Cpu(config) => {
        Box::new(CpuProvider::new(config, shared_state.sysinfo.clone()))
      }
      ProviderConfig::Host(config) => Box::new(HostProvider::new(config)),
      ProviderConfig::Ip(config) => Box::new(IpProvider::new(config)),
      #[cfg(windows)]
      ProviderConfig::Komorebi(config) => {
        Box::new(KomorebiProvider::new(config))
      }
      #[cfg(windows)]
      ProviderConfig::Media(config) => {
        Box::new(MediaProvider::new(config))
      }
      ProviderConfig::Memory(config) => {
        Box::new(MemoryProvider::new(config, shared_state.sysinfo.clone()))
      }
      ProviderConfig::Disk(config) => Box::new(DiskProvider::new(config)),
      ProviderConfig::Network(config) => {
        Box::new(NetworkProvider::new(config))
      }
      ProviderConfig::Weather(config) => {
        Box::new(WeatherProvider::new(config))
      }
      #[cfg(windows)]
      ProviderConfig::Keyboard(config) => {
        Box::new(KeyboardProvider::new(config))
      }
      #[allow(unreachable_patterns)]
      _ => bail!("Provider not supported on this operating system."),
    };

    Ok(provider)
  }

  /// Re-emits the latest provider output.
  ///
  /// No-ops if the provider hasn't outputted yet, since the provider will
  /// anyways emit its output after initialization.
  pub async fn refresh(&self) -> anyhow::Result<()> {
    let cache = { self.cache.lock().await.clone() };

    if let Some(cache) = cache {
      self.emit_result_tx.send(*cache).await?;
    }

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
