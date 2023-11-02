use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use sysinfo::{System, SystemExt};
use tokio::{
  sync::{mpsc::Sender, Mutex},
  task::{self, AbortHandle},
  time,
};

use super::{provider::Provider, provider_config::CpuProviderConfig};

pub struct CpuProvider {
  pub config: CpuProviderConfig,
  abort_handle: Arc<Mutex<Option<AbortHandle>>>,
}

impl CpuProvider {
  pub fn new(config: CpuProviderConfig) -> CpuProvider {
    CpuProvider {
      config,
      abort_handle: Arc::new(Mutex::new(None)),
    }
  }
}

#[async_trait]
impl Provider for CpuProvider {
  async fn start(&self, output_sender: Sender<String>) {
    let forever = task::spawn(async move {
      let mut interval = time::interval(Duration::from_millis(5000));
      let mut sys = System::new_all();

      loop {
        interval.tick().await;
        sys.refresh_all();
        println!("=> system:");
        println!("total memory: {} bytes", sys.total_memory());

        _ = output_sender
          .send(format!("total memory: {} bytes", sys.total_memory()))
          .await;
      }
    });

    // TODO: Need to manually re-lock?
    let mut abort_handle = self.abort_handle.lock().await;
    *abort_handle = Some(forever.abort_handle());

    _ = forever.await;
  }

  async fn stop(&self) {
    let abort_handle = self.abort_handle.lock().await;

    if let Some(handle) = &*abort_handle {
      handle.abort();
    }
  }
}
