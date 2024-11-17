use std::{collections::HashMap, sync::Arc};

use anyhow::{bail, Context};
use tauri::{AppHandle, Emitter};
use tokio::{
  sync::{mpsc, oneshot, Mutex},
  task,
};
use tracing::info;

use super::{
  battery::BatteryProvider, cpu::CpuProvider, disk::DiskProvider,
  host::HostProvider, ip::IpProvider, memory::MemoryProvider,
  network::NetworkProvider, weather::WeatherProvider, Provider,
  ProviderConfig, ProviderFunction, ProviderFunctionResult,
  ProviderOutput, RuntimeType,
};
#[cfg(windows)]
use super::{
  keyboard::KeyboardProvider, komorebi::KomorebiProvider,
  media::MediaProvider,
};

/// Common fields for a provider.
pub struct CommonProviderState {
  /// Receiver channel for stopping the provider.
  pub stop_rx: oneshot::Receiver<()>,

  /// Receiver channel for incoming function calls to the provider.
  pub function_rx: mpsc::Receiver<(
    ProviderFunction,
    oneshot::Sender<ProviderFunctionResult>,
  )>,

  /// Sender channel for outgoing provider emissions.
  pub emit_tx: mpsc::UnboundedSender<ProviderEmission>,

  /// Hash of the provider's config.
  pub config_hash: String,

  /// Shared `sysinfo` instance.
  pub sysinfo: Arc<Mutex<sysinfo::System>>,
}

/// A thread-safe `Result` type for provider outputs and errors.
pub type ProviderEmission = Result<ProviderOutput, String>;

/// Reference to an active provider.
struct ProviderRef {
  /// Sender channel for stopping the provider.
  stop_tx: oneshot::Sender<()>,

  /// Sender channel for sending function calls to the provider.
  function_tx: mpsc::Sender<(
    ProviderFunction,
    oneshot::Sender<ProviderFunctionResult>,
  )>,

  /// Handle to the provider's task.
  task_handle: task::JoinHandle<()>,
}

/// Manages the creation and cleanup of providers.
pub struct ProviderManager {
  /// Handle to the Tauri application.
  app_handle: AppHandle,

  /// Map of active provider refs.
  provider_refs: Arc<Mutex<HashMap<String, ProviderRef>>>,

  /// Cache of provider emissions.
  emit_cache: Arc<Mutex<HashMap<String, ProviderEmission>>>,

  /// Sender channel for provider emissions.
  emit_tx: mpsc::UnboundedSender<ProviderEmission>,

  /// Shared `sysinfo` instance.
  sysinfo: Arc<Mutex<sysinfo::System>>,
}

impl ProviderManager {
  /// Creates a new provider manager.
  ///
  /// Returns a tuple containing the `ProviderManager` instance and a
  /// channel for provider emissions.
  pub fn new(
    app_handle: &AppHandle,
  ) -> (Arc<Self>, mpsc::UnboundedReceiver<ProviderEmission>) {
    let (emit_tx, emit_rx) = mpsc::unbounded_channel::<ProviderEmission>();

    (
      Arc::new(Self {
        app_handle: app_handle.clone(),
        provider_refs: Arc::new(Mutex::new(HashMap::new())),
        emit_cache: Arc::new(Mutex::new(HashMap::new())),
        sysinfo: Arc::new(Mutex::new(sysinfo::System::new_all())),
        emit_tx,
      }),
      emit_rx,
    )
  }

  /// Creates a provider with the given config.
  pub async fn create(
    &self,
    config_hash: String,
    config: ProviderConfig,
  ) -> anyhow::Result<()> {
    // If a provider with the given config already exists, re-emit its
    // latest emission and return early.
    {
      if let Some(found_emit) =
        self.emit_cache.lock().await.get(&config_hash)
      {
        self.app_handle.emit("provider-emit", found_emit);
        return Ok(());
      };
    }

    let (stop_tx, stop_rx) = oneshot::channel();
    let (function_tx, function_rx) = mpsc::channel(1);

    let common = CommonProviderState {
      stop_rx,
      function_rx,
      emit_tx: self.emit_tx.clone(),
      config_hash: config_hash.clone(),
      sysinfo: self.sysinfo.clone(),
    };

    let task_handle =
      self.create_instance(config, config_hash.clone(), common)?;

    let provider_ref = ProviderRef {
      stop_tx,
      function_tx,
      task_handle,
    };

    let mut providers = self.provider_refs.lock().await;
    providers.insert(config_hash, provider_ref);

    Ok(())
  }

  /// Creates a new provider instance.
  fn create_instance(
    &self,
    config: ProviderConfig,
    config_hash: String,
    common: CommonProviderState,
  ) -> anyhow::Result<task::JoinHandle<()>> {
    let mut provider: Box<dyn Provider> = match config {
      ProviderConfig::Battery(config) => {
        Box::new(BatteryProvider::new(config, common))
      }
      ProviderConfig::Cpu(config) => {
        Box::new(CpuProvider::new(config, common))
      }
      ProviderConfig::Host(config) => {
        Box::new(HostProvider::new(config, common))
      }
      ProviderConfig::Ip(config) => {
        Box::new(IpProvider::new(config, common))
      }
      #[cfg(windows)]
      ProviderConfig::Komorebi(config) => {
        Box::new(KomorebiProvider::new(config, common))
      }
      #[cfg(windows)]
      ProviderConfig::Media(config) => {
        Box::new(MediaProvider::new(config, common))
      }
      ProviderConfig::Memory(config) => {
        Box::new(MemoryProvider::new(config, common))
      }
      ProviderConfig::Disk(config) => {
        Box::new(DiskProvider::new(config, common))
      }
      ProviderConfig::Network(config) => {
        Box::new(NetworkProvider::new(config, common))
      }
      ProviderConfig::Weather(config) => {
        Box::new(WeatherProvider::new(config, common))
      }
      #[cfg(windows)]
      ProviderConfig::Keyboard(config) => {
        Box::new(KeyboardProvider::new(config, common))
      }
      #[allow(unreachable_patterns)]
      _ => bail!("Provider not supported on this operating system."),
    };

    // Spawn the provider's task based on its runtime type.
    let task_handle = match provider.runtime_type() {
      RuntimeType::Async => task::spawn(async move {
        provider.start_async().await;
        info!("Provider stopped: {}", config_hash);
      }),
      RuntimeType::Sync => task::spawn_blocking(move || {
        provider.start_sync();
        info!("Provider stopped: {}", config_hash);
      }),
    };

    Ok(task_handle)
  }

  /// Sends a function call through a channel to be executed by the
  /// provider.
  ///
  /// Returns the result of the function execution.
  async fn call_function(
    &self,
    config_hash: String,
    function: ProviderFunction,
  ) -> anyhow::Result<ProviderFunctionResult> {
    let provider_refs = self.provider_refs.lock().await;
    let provider_ref = provider_refs
      .get(&config_hash)
      .context("No provider found with config.")?;

    let (tx, rx) = oneshot::channel();
    provider_ref.function_tx.send((function, tx)).await?;

    Ok(rx.await?)
  }

  /// Destroys and cleans up the provider with the given config.
  pub async fn stop(&self, config_hash: String) -> anyhow::Result<()> {
    let mut provider_refs = self.provider_refs.lock().await;
    let provider_ref = provider_refs
      .remove(&config_hash)
      .context("No provider found with config.")?;

    // Send shutdown signal to the provider and wait for it to stop.
    provider_ref.stop_tx.send(());
    provider_ref.task_handle.await?;

    Ok(())
  }

  /// Updates the cache with the given provider emission.
  pub async fn update_cache(&self, emit: ProviderEmission) {
    let mut cache = self.emit_cache.lock().await;
    cache.insert(emit.config_hash, emit);
  }
}
