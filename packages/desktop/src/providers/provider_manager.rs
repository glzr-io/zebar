use std::{collections::HashMap, sync::Arc};

use anyhow::{bail, Context};
use sysinfo::System;
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
  ProviderConfig, ProviderEmission, ProviderFunction,
  ProviderFunctionResult, RuntimeType,
};
#[cfg(windows)]
use super::{
  keyboard::KeyboardProvider, komorebi::KomorebiProvider,
  media::MediaProvider,
};

/// Reference to an active provider.
pub struct ProviderRef {
  /// Sender channel for stopping the provider.
  destroy_tx: mpsc::Sender<()>,

  /// Sender channel for sending function calls to the provider.
  function_tx: mpsc::Sender<(
    ProviderFunction,
    oneshot::Sender<ProviderFunctionResult>,
  )>,
}

/// State shared between providers.
#[derive(Clone)]
pub struct SharedProviderState {
  pub sysinfo: Arc<Mutex<System>>,
}

/// Manages the creation and cleanup of providers.
pub struct ProviderManager {
  /// Handle to the Tauri application.
  app_handle: AppHandle,

  /// Map of active provider refs.
  provider_refs: Arc<Mutex<HashMap<String, ProviderRef>>>,

  /// Cache of provider emissions.
  emit_cache: Arc<Mutex<HashMap<String, ProviderEmission>>>,

  /// Shared state between providers.
  shared_state: SharedProviderState,

  /// Sender channel for provider emissions.
  emit_tx: mpsc::UnboundedSender<ProviderEmission>,
}

impl ProviderManager {
  /// Creates a new provider manager.
  ///
  /// Returns a tuple containing the manager and a channel for provider
  /// emissions.
  pub fn new(
    app_handle: &AppHandle,
  ) -> (Arc<Self>, mpsc::UnboundedReceiver<ProviderEmission>) {
    let (emit_tx, emit_rx) = mpsc::unbounded_channel::<ProviderEmission>();

    (
      Arc::new(Self {
        app_handle: app_handle.clone(),
        provider_refs: Arc::new(Mutex::new(HashMap::new())),
        emit_cache: Arc::new(Mutex::new(HashMap::new())),
        shared_state: SharedProviderState {
          sysinfo: Arc::new(Mutex::new(System::new_all())),
        },
        emit_tx,
      }),
      emit_rx,
    )
  }

  /// Creates a provider with the given config.
  pub async fn create(
    &self,
    config_hash: &str,
    config: ProviderConfig,
  ) -> anyhow::Result<()> {
    // If a provider with the given config already exists, re-emit its
    // latest emission and return early.
    {
      if let Some(found_emit) =
        self.emit_cache.lock().await.get(config_hash)
      {
        self.app_handle.emit("provider-emit", found_emit);
        return Ok(());
      };
    }

    let (stop_tx, stop_rx) = mpsc::channel::<()>(1);
    let mut provider = self.create_provider(config, stop_rx)?;

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

    let provider_ref = ProviderRef {
      destroy_tx: self.emit_tx.clone(),
      function_tx: self.emit_tx.clone(),
    };

    let mut providers = self.provider_refs.lock().await;
    providers.insert(config_hash.to_string(), provider_ref);

    Ok(())
  }

  fn create_provider(
    &self,
    config: ProviderConfig,
  ) -> anyhow::Result<Box<dyn Provider>> {
    let provider: Box<dyn Provider> = match config {
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

  /// Sends a function call through a channel to be executed by the
  /// provider.
  ///
  /// Returns the result of the function execution.
  async fn call_function(
    &self,
    config_hash: &str,
    function: ProviderFunction,
  ) -> anyhow::Result<ProviderFunctionResult> {
    let mut provider_refs = self.provider_refs.lock().await;
    let provider_ref = provider_refs
      .get(config_hash)
      .context("No provider found with config.")?;

    let (tx, rx) = oneshot::channel();
    provider_ref.function_tx.send((function, tx))?;
    rx.await?
  }

  /// Destroys and cleans up the provider with the given config.
  pub async fn stop(&self, config_hash: &str) -> anyhow::Result<()> {
    let mut provider_refs = self.provider_refs.lock().await;
    let provider_ref = provider_refs
      .remove(config_hash)
      .context("No provider found with config.")?;

    let (tx, rx) = oneshot::channel();
    provider_ref.stop_tx.send(tx)?;
    rx.await?
  }

  /// Updates the cache with the given provider emission.
  pub async fn update_cache(&self, emit: ProviderEmission) {
    let mut cache = self.emit_cache.lock().await;
    cache.insert(emit.config_hash, emit);
  }
}
