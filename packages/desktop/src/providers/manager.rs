use std::{
  sync::Arc,
  time::{Duration, Instant},
};

use anyhow::{bail, Context, Result};
use serde::Serialize;
use sysinfo::System;
use tauri::{App, AppHandle, Manager, Runtime};
use tokio::{
  sync::{
    mpsc::{self, Sender},
    Mutex,
  },
  task,
};
use tracing::{info, warn};

use crate::providers::provider::Provider;

#[cfg(all(windows, target_arch = "x86_64"))]
use super::komorebi::KomorebiProvider;
use super::{
  battery::BatteryProvider, config::ProviderConfig, cpu::CpuProvider,
  host::HostProvider, ip::IpProvider, memory::MemoryProvider,
  network::NetworkProvider, variables::ProviderVariables,
  weather::WeatherProvider,
};

pub struct ListenProviderArgs {
  pub config_hash: String,
  pub config: ProviderConfig,
  pub tracked_access: Vec<String>,
}

pub struct UnlistenProviderArgs {
  pub config_hash: String,
}

/// Reference to a currently active provider.
#[derive(Debug, Clone)]
pub struct ProviderRef {
  config_hash: String,
  min_refresh_interval: Duration,
  prev_refresh: Option<Instant>,
  prev_output: Option<Box<ProviderOutput>>,
  refresh_tx: Sender<()>,
  stop_tx: Sender<()>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProviderOutput {
  pub config_hash: String,
  pub variables: VariablesResult,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum VariablesResult {
  Data(ProviderVariables),
  Error(String),
}

/// Wrapper around the creation and deletion of providers.
pub struct ProviderManager {
  listen_input_tx: Sender<ListenProviderArgs>,
  unlisten_input_tx: Sender<UnlistenProviderArgs>,
}

/// Initializes `ProviderManager` in Tauri state.
pub fn init<R: Runtime>(app: &mut App<R>) -> Result<()> {
  app.manage(ProviderManager::new(app.handle().clone()));
  Ok(())
}

/// Create a channel for outputting provider variables to client.
fn handle_provider_emit_output<R: Runtime>(
  app_handle: (impl Manager<R> + Sync + Send + 'static),
  active_providers: Arc<Mutex<Vec<ProviderRef>>>,
) -> Sender<ProviderOutput> {
  let (output_sender, mut output_receiver) =
    mpsc::channel::<ProviderOutput>(1);

  task::spawn(async move {
    while let Some(output) = output_receiver.recv().await {
      info!("Emitting for provider: {}", output.config_hash);
      let output = Box::new(output);

      if let Err(err) = app_handle.emit("provider-emit", output.clone()) {
        warn!("Error emitting provider output: {:?}", err);
      }

      if let Ok(mut providers) = active_providers.try_lock() {
        // Find provider that matches given config hash.
        let found_provider = providers
          .iter_mut()
          .find(|provider| *provider.config_hash == output.config_hash);

        if let Some(found) = found_provider {
          found.prev_refresh = Some(Instant::now());
          found.prev_output = Some(output);
        }
      } else {
        warn!("Failed to update provider output cache.");
      }
    }
  });

  output_sender
}

/// Create a channel for handling provider listen commands from client.
fn handle_provider_listen_input(
  active_providers: Arc<Mutex<Vec<ProviderRef>>>,
  emit_output_tx: Sender<ProviderOutput>,
) -> Sender<ListenProviderArgs> {
  let (listen_input_tx, mut listen_input_rx) =
    mpsc::channel::<ListenProviderArgs>(1);

  let sysinfo = Arc::new(Mutex::new(System::new_all()));

  task::spawn(async move {
    while let Some(input) = listen_input_rx.recv().await {
      // Find provider that matches given config hash.
      let found_provider = {
        active_providers
          .lock()
          .await
          .iter()
          .find(|&provider| *provider.config_hash == input.config_hash)
          .cloned()
      };

      // If a provider with the given config already exists, refresh it and
      // return early.
      if let Some(found) = found_provider {
        // The previous output of providers is cached. If within the
        // minimum refresh interval, send the previous output.
        match (found.prev_refresh, found.prev_output.clone()) {
          (Some(prev_refresh), Some(prev_output))
            if prev_refresh.elapsed() < found.min_refresh_interval =>
          {
            _ = emit_output_tx.send(*prev_output).await
          }
          _ => _ = found.refresh_tx.send(()).await,
        }

        continue;
      };

      let (refresh_tx, refresh_rx) = mpsc::channel::<()>(1);
      let (stop_tx, stop_rx) = mpsc::channel::<()>(1);
      let emit_output_tx = emit_output_tx.clone();

      // Attempt to create a new provider.
      let new_provider = create_provider(input.config, sysinfo.clone());

      if let Err(err) = new_provider {
        _ = emit_output_tx
          .send(ProviderOutput {
            config_hash: input.config_hash,
            variables: VariablesResult::Error(err.to_string()),
          })
          .await;

        continue;
      } else if let Ok(mut new_provider) = new_provider {
        let min_refresh_interval = new_provider.min_refresh_interval();

        let mut providers = active_providers.lock().await;
        providers.push(ProviderRef {
          config_hash: input.config_hash.clone(),
          min_refresh_interval,
          prev_refresh: None,
          prev_output: None,
          refresh_tx,
          stop_tx,
        });

        task::spawn(async move {
          new_provider
            .start(input.config_hash, emit_output_tx, refresh_rx, stop_rx)
            .await
        });
      }
    }
  });

  listen_input_tx
}

fn create_provider(
  config: ProviderConfig,
  sysinfo: Arc<Mutex<System>>,
) -> Result<Box<dyn Provider + Send>> {
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
      Box::new(NetworkProvider::new(config))
    }
    ProviderConfig::Weather(config) => {
      Box::new(WeatherProvider::new(config))
    }
    #[allow(unreachable_patterns)]
    _ => bail!("Provider not supported on this operating system."),
  };

  Ok(provider)
}

/// Create a channel for handling provider unlisten commands from client.
fn handle_provider_unlisten_input(
  active_providers: Arc<Mutex<Vec<ProviderRef>>>,
) -> Sender<UnlistenProviderArgs> {
  let (unlisten_input_tx, mut unlisten_input_rx) =
    mpsc::channel::<UnlistenProviderArgs>(1);

  task::spawn(async move {
    while let Some(input) = unlisten_input_rx.recv().await {
      // Find provider that matches given config hash.
      let mut providers = active_providers.lock().await;
      let found_index = providers
        .iter()
        .position(|provider| provider.config_hash == input.config_hash);

      // Stop the given provider. This triggers any necessary cleanup.
      if let Some(found_index) = found_index {
        if let Some(provider) = providers.get(found_index) {
          let _ = provider.stop_tx.send(()).await;
          providers.remove(found_index);
        }
      };
    }
  });

  unlisten_input_tx
}

impl ProviderManager {
  pub fn new<R: Runtime>(app_handle: AppHandle<R>) -> ProviderManager {
    let active_providers = Arc::new(Mutex::new(vec![]));

    let emit_output_tx =
      handle_provider_emit_output(app_handle, active_providers.clone());

    let listen_input_tx = handle_provider_listen_input(
      active_providers.clone(),
      emit_output_tx,
    );

    let unlisten_input_tx =
      handle_provider_unlisten_input(active_providers);

    ProviderManager {
      listen_input_tx,
      unlisten_input_tx,
    }
  }

  /// Create a provider with the given config.
  pub async fn listen(
    &self,
    config_hash: String,
    config: ProviderConfig,
    tracked_access: Vec<String>,
  ) -> Result<()> {
    self
      .listen_input_tx
      .send(ListenProviderArgs {
        config_hash,
        config,
        tracked_access,
      })
      .await
      .context("error msg")
  }

  /// Destroy and clean up a provider with the given config.
  pub async fn unlisten(&self, config_hash: String) -> Result<()> {
    self
      .unlisten_input_tx
      .send(UnlistenProviderArgs { config_hash })
      .await
      .context("error msg")
  }
}
