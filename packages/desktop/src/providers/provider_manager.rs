use std::sync::Arc;

use anyhow::{Context, Result};
use tauri::{Manager, Runtime};
use tokio::sync::{mpsc::Sender, Mutex};
use tokio::{sync::mpsc, task};
use tracing::info;

use crate::providers::provider::Provider;

use super::{
  cpu::CpuProvider, network_provider::NetworkProvider,
  provider_config::ProviderConfig,
};

pub struct ListenProviderArgs {
  pub options_hash: String,
  pub options: ProviderConfig,
  pub tracked_access: Vec<String>,
}

pub struct UnlistenProviderArgs {
  pub options_hash: String,
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

fn handle_provider_emit_output<R: tauri::Runtime>(
  manager: (impl Manager<R> + Sync + Send + 'static),
) -> Sender<String> {
  let (output_sender, mut output_receiver) = mpsc::channel(1);

  task::spawn(async move {
    while let Some(output) = output_receiver.recv().await {
      info!(?output, "handle_provider_emit");
      manager.emit_all("provider-emit", output).unwrap();
    }
  });

  output_sender
}

fn handle_provider_listen_input(
  active_providers: Arc<Mutex<Vec<&dyn Provider>>>,
  emit_output_tx: Sender<String>,
) -> Sender<ListenProviderArgs> {
  let (listen_input_tx, mut listen_input_rx) =
    mpsc::channel::<ListenProviderArgs>(1);

  task::spawn(async move {
    while let Some(input) = listen_input_rx.recv().await {
      let provider: Arc<Mutex<dyn Provider + Send + Sync + 'static>> =
        match input.options {
          ProviderConfig::Cpu(config) => {
            Arc::new(Mutex::new(CpuProvider::new(config)))
          }
          ProviderConfig::Network(config) => {
            Arc::new(Mutex::new(NetworkProvider::new(config)))
          }
        };

      let sender = emit_output_tx.clone();
      let provider_clone = provider.clone();

      task::spawn(async move {
        let provider = provider_clone.lock().await;
        provider.start(sender).await; // Starts a long-running task
      });
    }
  });

  listen_input_tx
}

fn handle_provider_unlisten_input(
  active_providers: Arc<Mutex<Vec<&dyn Provider>>>,
) -> Sender<UnlistenProviderArgs> {
  let (unlisten_input_tx, mut unlisten_input_rx) =
    mpsc::channel::<UnlistenProviderArgs>(1);

  task::spawn(async move {
    while let Some(input) = unlisten_input_rx.recv().await {
      // let providers = active_providers.lock().await;
      // let provider = providers
      //   .iter()
      //   .find(|&provider| *provider.options_hash == input.options_hash);
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
    options_hash: String,
    options: ProviderConfig,
    tracked_access: Vec<String>,
  ) -> Result<()> {
    self
      .listen_input_tx
      .send(ListenProviderArgs {
        options_hash,
        options,
        tracked_access,
      })
      .await
      .context("error msg")
  }

  /// Destroy and clean up a provider with the given config.
  pub async fn unlisten(&self, options_hash: String) -> Result<()> {
    self
      .unlisten_input_tx
      .send(UnlistenProviderArgs { options_hash })
      .await
      .context("error msg")
  }
}
