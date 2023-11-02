use tokio::sync::mpsc::{Receiver, Sender};

use anyhow::Result;
use tauri::{Manager, Runtime};
use tokio::{sync::mpsc, task};
use tracing::info;

use crate::providers::provider::Provider;

use super::{
  cpu::CpuProvider, network_provider::NetworkProvider,
  provider_config::ProviderConfig,
};

pub struct CreateProviderArgs {
  pub options_hash: String,
  pub options: ProviderConfig,
  pub tracked_access: Vec<String>,
}

/// Handles the creation and deletion of providers.
pub struct ProviderManager {
  pub input_sender: Sender<CreateProviderArgs>,
  active_providers: Vec<Box<dyn Provider + Sync + Send>>,
}

/// Initializes `ProviderManager` in Tauri state.
pub fn init<R: Runtime>(app: &mut tauri::App<R>) -> tauri::plugin::Result<()> {
  app.manage(ProviderManager::new(app.handle()));
  Ok(())
}

impl ProviderManager {
  pub fn new<R: Runtime>(app: tauri::AppHandle<R>) -> ProviderManager {
    let (input_sender, input_receiver) = mpsc::channel(1);
    let (output_sender, mut output_receiver) = mpsc::channel(1);

    task::spawn(async move {
      Self::handle_provider_create(input_receiver, output_sender).await
    });

    task::spawn(async move {
      Self::handle_provider_emit(&mut output_receiver, &app.app_handle()).await
    });

    ProviderManager {
      input_sender,
      active_providers: vec![],
    }
  }

  async fn handle_provider_create(
    mut input_receiver: mpsc::Receiver<CreateProviderArgs>,
    output_sender: mpsc::Sender<String>,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    while let Some(input) = input_receiver.recv().await {
      let sender = output_sender.clone();

      task::spawn(async move {
        match input.options {
          ProviderConfig::Cpu(config) => {
            println!("creating cpu");
            let mut cpu_provider = CpuProvider::new(config);
            cpu_provider.start(sender).await;
          }
          ProviderConfig::Network(config) => {
            println!("creating network");
            let mut network_provider = NetworkProvider::new(config);
            network_provider.start(sender).await;
          }
        }
      });
    }

    Ok(())
  }

  async fn handle_provider_emit<R: tauri::Runtime>(
    output_receiver: &mut Receiver<String>,
    manager: &impl Manager<R>,
  ) {
    loop {
      if let Some(output) = output_receiver.recv().await {
        info!(?output, "handle_provider_emit");
        manager.emit_all("provider-emit", output).unwrap();
      }
    }
  }
}
