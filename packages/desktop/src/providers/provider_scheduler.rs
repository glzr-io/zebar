use std::time::Duration;
use tokio::sync::mpsc::{Receiver, Sender};

use anyhow::Result;
use sysinfo::{System, SystemExt};
use tauri::{Manager, Runtime};
use tokio::{sync::mpsc, task, time};

use super::provider_config::ProviderConfig;

pub fn init<R: Runtime>(app: &mut tauri::App<R>) -> tauri::plugin::Result<()> {
  app.manage(ProviderScheduler::new(app.handle()));
  Ok(())
}

pub struct CreateProviderArgs {
  options_hash: String,
  options: ProviderConfig,
  tracked_access: Vec<String>,
}

pub struct ProviderScheduler {
  input_channel: (Sender<String>, Receiver<String>),
  output_channel: (Sender<String>, Receiver<String>),
}

impl ProviderScheduler {
  pub fn new<R: Runtime>(app: tauri::AppHandle<R>) -> ProviderScheduler {
    let (input_sender, input_receiver) = mpsc::channel(1);
    let (output_sender, mut output_receiver) = mpsc::channel(1);

    task::spawn(async move {
      Self::listen_provider_create(input_receiver, output_sender).await
    });

    ProviderScheduler {
      input_channel: mpsc::channel(1),
      output_channel: mpsc::channel(1),
    }
  }

  async fn listen_provider_create(
    mut input_rx: mpsc::Receiver<String>,
    output_tx: mpsc::Sender<String>,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    while let Some(input) = input_rx.recv().await {
      let output = input;
      output_tx.send(output).await?;
    }

    Ok(())
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
