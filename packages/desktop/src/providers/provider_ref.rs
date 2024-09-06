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
  config_hash: String,
  cache: Option<ProviderCache>,
  emit_result_tx: mpsc::Sender<ProviderResult>,
  stop_tx: mpsc::Sender<()>,
}

/// Cache for provider output.
#[derive(Debug, Clone)]
pub struct ProviderCache {
  timestamp: Instant,
  output: Box<ProviderOutput>,
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
    config: ProviderConfig,
    config_hash: String,
    shared_state: SharedProviderState,
  ) -> anyhow::Result<Self> {
    let (emit_result_tx, mut emit_result_rx) =
      mpsc::channel::<ProviderResult>(1);

    let config_hash_clone = config_hash.clone();
    let app_handle = app_handle.clone();
    task::spawn(async move {
      while let Some(output) = emit_result_rx.recv().await {
        info!("Emitting for provider: {}", config_hash_clone);

        let output = Box::new(output);
        let payload = json!({
          "configHash": config_hash_clone.clone(),
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

    let (stop_tx, stop_rx) = mpsc::channel::<()>(1);

    Self::start_provider(
      config,
      config_hash.clone(),
      shared_state,
      emit_result_tx.clone(),
      stop_rx,
    );

    Ok(Self {
      config_hash,
      cache: None,
      emit_result_tx,
      stop_tx,
    })
  }

  /// Starts the provider in a separate task.
  fn start_provider(
    config: ProviderConfig,
    config_hash: String,
    shared_state: SharedProviderState,
    emit_result_tx: mpsc::Sender<ProviderResult>,
    mut stop_rx: mpsc::Receiver<()>,
  ) {
    task::spawn(async move {
      // TODO: Remove unwrap.
      let provider = Self::create_provider(config, shared_state).unwrap();

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
  }

  fn create_provider(
    config: ProviderConfig,
    shared_state: SharedProviderState,
  ) -> anyhow::Result<Box<dyn Provider>> {
    let provider: Box<dyn Provider> = match config {
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
    // if let Some(cache) = self.cache {
    //   self.emit_result_tx.send(*cache.output.clone()).await?;
    // }

    Ok(())
  }

  /// Stops the given provider.
  ///
  /// This triggers any necessary cleanup.
  pub async fn stop(&mut self) -> anyhow::Result<()> {
    self.stop_tx.send(()).await?;
    Ok(())
  }
}
