use std::time::Duration;
use tokio::sync::mpsc::{Receiver, Sender};

use anyhow::Result;
use sysinfo::{System, SystemExt};
use tauri::{Manager, Runtime};
use tokio::{sync::mpsc, task, time};
use tracing::info;

use super::provider_config::ProviderConfig;

pub fn init<R: Runtime>(app: &mut tauri::App<R>) -> tauri::plugin::Result<()> {
  app.manage(ProviderScheduler::new(app.handle()));
  Ok(())
}

pub struct CreateProviderArgs {
  pub options_hash: String,
  pub options: ProviderConfig,
  pub tracked_access: Vec<String>,
}

pub struct ProviderScheduler {
  pub input_sender: Sender<CreateProviderArgs>,
}

impl ProviderScheduler {
  pub fn new<R: Runtime>(app: tauri::AppHandle<R>) -> ProviderScheduler {
    let (input_sender, input_receiver) = mpsc::channel(1);
    let (output_sender, mut output_receiver) = mpsc::channel(1);

    task::spawn(async move {
      Self::handle_provider_create(input_receiver, output_sender).await
    });

    task::spawn(async move {
      Self::handle_provider_emit(&mut output_receiver, &app.app_handle()).await
    });

    ProviderScheduler { input_sender }
  }

  async fn handle_provider_create(
    mut input_receiver: mpsc::Receiver<CreateProviderArgs>,
    output_sender: mpsc::Sender<String>,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    while let Some(input) = input_receiver.recv().await {
      let output = input;
      output_tx.send(output).await?;
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
        manager
          .emit_all("rs2js", format!("rs: {}", output))
          .unwrap();
      }
    }
  }

  pub async fn register(
    &self,
    options_hash: &str,
    options: ProviderConfig,
    tracked_access: Vec<&str>,
  ) -> Result<()> {
    match options {
      ProviderConfig::Cpu(_) => {
        let forever = task::spawn(async {
          let mut interval = time::interval(Duration::from_millis(5000));
          let mut sys = System::new_all();

          loop {
            interval.tick().await;
            sys.refresh_all();
            println!("=> system:");
            println!("total memory: {} bytes", sys.total_memory());
          }
        });

        forever.await;
        Ok(())
      }
      ProviderConfig::Network(_) => todo!(),
    }
  }
}
