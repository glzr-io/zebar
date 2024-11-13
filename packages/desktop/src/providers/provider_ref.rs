use std::sync::Arc;

use anyhow::bail;
use serde::Serialize;
use serde_json::json;
use tauri::{AppHandle, Emitter};
use tokio::{
  sync::{mpsc, Mutex},
  task,
};
use tracing::{info, warn};

use super::{
  battery::BatteryProvider, cpu::CpuProvider, disk::DiskProvider,
  host::HostProvider, ip::IpProvider, memory::MemoryProvider,
  network::NetworkProvider, weather::WeatherProvider, Provider,
  ProviderConfig, ProviderOutput, SharedProviderState,
};
#[cfg(windows)]
use super::{
  keyboard::KeyboardProvider, komorebi::KomorebiProvider,
  media::MediaProvider,
};

/// Reference to an active provider.
pub struct ProviderRef {
  /// Cache for provider output.
  cache: Arc<Mutex<Option<Box<ProviderResult>>>>,

  /// Sender channel for emitting provider output/error to frontend
  /// clients.
  emit_result_tx: mpsc::Sender<ProviderResult>,

  /// Sender channel for stopping the provider.
  stop_tx: mpsc::Sender<()>,
}

/// Provider output/error emitted to frontend clients.
///
/// This is used instead of a normal `Result` type in order to serialize it
/// in a nicer way.
#[derive(Debug, Clone, PartialEq, Serialize)]
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
    config: ProviderConfig,
    config_hash: String,
    shared_state: SharedProviderState,
  ) -> anyhow::Result<Self> {
    let cache = Arc::new(Mutex::new(None));

    let (stop_tx, stop_rx) = mpsc::channel::<()>(1);
    let (emit_result_tx, emit_result_rx) =
      mpsc::channel::<ProviderResult>(1);

    Self::start_output_listener(
      app_handle.clone(),
      config_hash.clone(),
      cache.clone(),
      emit_result_rx,
    );

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

  fn start_output_listener(
    app_handle: AppHandle,
    config_hash: String,
    cache: Arc<Mutex<Option<Box<ProviderResult>>>>,
    mut emit_result_rx: mpsc::Receiver<ProviderResult>,
  ) {
    task::spawn(async move {
      while let Some(output) = emit_result_rx.recv().await {
        info!("Emitting for provider: {}", config_hash);

        let output = Box::new(output);
        let payload = json!({
          "configHash": config_hash.clone(),
          "result": *output.clone(),
        });

        if let Err(err) = app_handle.emit("provider-emit", payload) {
          warn!("Error emitting provider output: {:?}", err);
        }

        // Update the provider's output cache.
        if let Ok(mut providers) = cache.try_lock() {
          *providers = Some(output);
        } else {
          warn!("Failed to update provider output cache.");
        }
      }
    });
  }

  /// Starts the provider in a separate task.
  fn start_provider(
    config: ProviderConfig,
    config_hash: String,
    shared_state: SharedProviderState,
    emit_result_tx: mpsc::Sender<ProviderResult>,
    mut stop_rx: mpsc::Receiver<()>,
  ) -> anyhow::Result<()> {
    let provider = Self::create_provider(config, shared_state)?;

    task::spawn(async move {
      // TODO: Add arc `should_stop` to be passed to `run`.

      let run = provider.run(emit_result_tx);
      tokio::pin!(run);

      // Ref: https://tokio.rs/tokio/tutorial/select#resuming-an-async-operation
      loop {
        tokio::select! {
          // Default match arm which continuously runs the provider.
          _ = run => break,

          // On stop, perform any necessary clean up and exit the loop.
          Some(_) = stop_rx.recv() => {
            info!("Stopping provider: {}", config_hash);
            _ = provider.on_stop().await;
            break;
          },
        }
      }

      info!("Provider stopped: {}", config_hash);
    });

    Ok(())
  }

  fn create_provider(
    config: ProviderConfig,
    shared_state: SharedProviderState,
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
      ProviderConfig::Disk(config) => {
        Box::new(DiskProvider::new(config, shared_state.diskinfo.clone()))
      }
      ProviderConfig::Network(config) => Box::new(NetworkProvider::new(
        config,
        shared_state.netinfo.clone(),
      )),
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
