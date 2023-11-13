use std::sync::Arc;

use anyhow::{Context, Result};
use serde::Serialize;
use sysinfo::{System, SystemExt};
use tauri::{Manager, Runtime};
use tokio::{
  sync::{
    mpsc::{self, Sender},
    Mutex,
  },
  task,
};
use tracing::info;

use crate::providers::provider::Provider;

use super::{
  battery::BatteryProvider, config::ProviderConfig, cpu::CpuProvider,
  host::HostProvider, memory::MemoryProvider, network::NetworkProvider,
  variables::ProviderVariables,
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
pub struct ProviderRef {
  config_hash: String,
  refresh_tx: Sender<()>,
  stop_tx: Sender<()>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProviderOutput {
  pub config_hash: String,
  pub variables: ProviderVariables,
}

/// Wrapper around the creation and deletion of providers.
pub struct ProviderManager {
  listen_input_tx: Sender<ListenProviderArgs>,
  unlisten_input_tx: Sender<UnlistenProviderArgs>,
}

/// Initializes `ProviderManager` in Tauri state.
pub fn init<R: Runtime>(app: &mut tauri::App<R>) -> tauri::plugin::Result<()> {
  app.manage(ProviderManager::new(app.handle()));
  Ok(())
}

/// Create a channel for outputting provider variables to client.
fn handle_provider_emit_output<R: tauri::Runtime>(
  manager: (impl Manager<R> + Sync + Send + 'static),
) -> Sender<ProviderOutput> {
  let (output_sender, mut output_receiver) = mpsc::channel::<ProviderOutput>(1);

  task::spawn(async move {
    while let Some(output) = output_receiver.recv().await {
      info!("Emitting for provider: {}", output.config_hash);
      // TODO: Error handling.
      manager.emit_all("provider-emit", output).unwrap();
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
      let mut providers = active_providers.lock().await;

      // Find provider that matches given config hash.
      let found_provider = providers
        .iter()
        .find(|&provider| *provider.config_hash == input.config_hash);

      // If a provider with the given config already exists, refresh it and
      // return early.
      if let Some(found) = found_provider {
        _ = found.refresh_tx.send(()).await;
        continue;
      };

      // Otherwise, spawn a new provider.
      let mut new_provider: Box<dyn Provider + Send> = match input.config {
        ProviderConfig::Battery(config) => {
          Box::new(BatteryProvider::new(config))
        }
        ProviderConfig::Cpu(config) => {
          Box::new(CpuProvider::new(config, sysinfo.clone()))
        }
        ProviderConfig::Host(config) => {
          Box::new(HostProvider::new(config, sysinfo.clone()))
        }
        ProviderConfig::Memory(config) => {
          Box::new(MemoryProvider::new(config, sysinfo.clone()))
        }
        ProviderConfig::Network(config) => {
          Box::new(NetworkProvider::new(config, sysinfo.clone()))
        }
      };

      let (refresh_tx, refresh_rx) = mpsc::channel::<()>(1);
      let (stop_tx, stop_rx) = mpsc::channel::<()>(1);
      let emit_output_tx = emit_output_tx.clone();
      let config_hash = input.config_hash.clone();

      task::spawn(async move {
        new_provider
          .start(input.config_hash, emit_output_tx, refresh_rx, stop_rx)
          .await;
      });

      providers.push(ProviderRef {
        config_hash,
        refresh_tx,
        stop_tx,
      })
    }
  });

  listen_input_tx
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
        let provider = providers.get(found_index).unwrap();
        _ = provider.stop_tx.send(()).await;
        providers.remove(found_index);
      };
    }
  });

  unlisten_input_tx
}

impl ProviderManager {
  pub fn new<R: Runtime>(app: tauri::AppHandle<R>) -> ProviderManager {
    let emit_output_tx = handle_provider_emit_output(app.app_handle());

    let active_providers = Arc::new(Mutex::new(vec![]));
    let active_providers_clone = active_providers.clone();

    let listen_input_tx =
      handle_provider_listen_input(active_providers, emit_output_tx);

    let unlisten_input_tx =
      handle_provider_unlisten_input(active_providers_clone);

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
