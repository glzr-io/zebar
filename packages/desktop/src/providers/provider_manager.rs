use std::sync::Arc;

use tokio::sync::{
  mpsc::{Receiver, Sender},
  Mutex,
};

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
  // active_providers: Vec<Box<dyn Provider + Sync + Send>>,
  // pub input_channel: (Sender<CreateProviderArgs>, Receiver<CreateProviderArgs>),
  // output_channel: (Sender<String>, Receiver<String>),
  // // active_providers: Vec<Arc<dyn Provider + Sync + Send>>,
  active_providers: Vec<Arc<Mutex<dyn Provider + Sync + Send>>>,
}

/// Initializes `ProviderManager` in Tauri state.
pub fn init<R: Runtime>(app: &mut tauri::App<R>) -> tauri::plugin::Result<()> {
  app.manage(ProviderManager::new(app.handle()));
  Ok(())
}

// fn create_manager() -> ProviderManager {
//   let (input_sender, input_receiver) = mpsc::channel(1);
//   let (output_sender, mut output_receiver) = mpsc::channel(1);

// }

fn handle_provider_create() -> Sender<String> {}

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

  // pub fn new<R: Runtime>(app: tauri::AppHandle<R>) -> ProviderManager {
  //   let (input_sender, input_receiver) = mpsc::channel(1);
  //   let (output_sender, mut output_receiver) = mpsc::channel(1);

  //   let mut manager = ProviderManager {
  //     input_sender,
  //     active_providers: vec![],
  //   };

  //   task::spawn(async {
  //     manager
  //       .handle_provider_create(input_receiver, output_sender)
  //       .await
  //   });

  //   task::spawn(async move {
  //     manager
  //       .handle_provider_emit(&mut output_receiver, &app.app_handle())
  //       .await
  //   });

  //   manager
  // }

  // pub fn new() -> ProviderManager {
  //   ProviderManager {
  //     input_channel: mpsc::channel(1),
  //     output_channel: mpsc::channel(1),
  //     active_providers: vec![],
  //   }
  // }

  // fn start<R: Runtime>(&self, app: tauri::AppHandle<R>) -> &ProviderManager {
  //   let (input_sender, input_receiver) = mpsc::channel(1);
  //   let (output_sender, mut output_receiver) = mpsc::channel(1);

  //   task::spawn(async move {
  //     self
  //       .handle_provider_create(input_receiver, output_sender)
  //       .await
  //   });

  //   task::spawn(async move {
  //     Self::handle_provider_emit(&mut output_receiver, &app.app_handle()).await
  //   });

  //   self
  //   // self
  // }

  async fn handle_provider_create(
    &mut self,
    mut input_receiver: mpsc::Receiver<CreateProviderArgs>,
    output_sender: mpsc::Sender<String>,
    // ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  ) {
    while let Some(input) = input_receiver.recv().await {
      let provider: Arc<Mutex<dyn Provider + Send + Sync + 'static>> =
        match input.options {
          ProviderConfig::Cpu(config) => {
            Arc::new(Mutex::new(CpuProvider::new(config)))
          }
          ProviderConfig::Network(config) => {
            Arc::new(Mutex::new(NetworkProvider::new(config)))
          }
          _ => panic!(),
        };

      let sender = output_sender.clone();
      let provider_clone = provider.clone();

      task::spawn(async move {
        let provider = provider_clone.lock().await;
        provider.start(sender).await; // Starts a long-running task
      });

      self.active_providers.push(provider);
    }
  }

  async fn on_delete(&self) {
    let provider = self.active_providers.get(1).unwrap().lock().await;
    provider.stop();
  }

  async fn handle_provider_emit<R: tauri::Runtime>(
    &self,
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
