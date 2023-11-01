use std::time::Duration;
use tokio::sync::mpsc::{Receiver, Sender};

use anyhow::Result;
use sysinfo::{System, SystemExt};
use tauri::{Manager, Runtime};
use tokio::{sync::mpsc, task, time};
use tracing::info;

use super::{cpu::CpuProvider, provider_config::ProviderConfig};

pub struct CreateProviderArgs {
  pub options_hash: String,
  pub options: ProviderConfig,
  pub tracked_access: Vec<String>,
}

pub struct ProviderManager {
  pub input_sender: Sender<CreateProviderArgs>,
}

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

    ProviderManager { input_sender }
  }

  async fn handle_provider_create(
    mut input_receiver: mpsc::Receiver<CreateProviderArgs>,
    output_sender: mpsc::Sender<String>,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    while let Some(input) = input_receiver.recv().await {
      let sender = output_sender.clone();

      match input.options {
        ProviderConfig::Cpu(config) => {
          let mut cpu_provider = CpuProvider::new(config);
          cpu_provider.start(sender).await;
        }
        ProviderConfig::Network(_) => todo!(),
      }
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
