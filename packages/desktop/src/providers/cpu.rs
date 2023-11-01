use std::time::Duration;

use sysinfo::{System, SystemExt};
use tokio::{
  sync::mpsc::Sender,
  task::{self, AbortHandle},
  time,
};

use super::provider_config::CpuProviderConfig;

pub struct CpuProvider {
  // pub output_sender: Sender<String>,
  pub config: CpuProviderConfig,
  abort_handle: Option<AbortHandle>,
}

impl CpuProvider {
  pub fn new(
    // output_sender: Sender<String>,
    config: CpuProviderConfig,
  ) -> CpuProvider {
    CpuProvider {
      // output_sender,
      config,
      abort_handle: None,
    }
  }

  pub async fn start(&mut self, output_sender: Sender<String>) {
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

    self.abort_handle = Some(forever.abort_handle());
    _ = forever.await;
  }

  pub fn stop(&self) {
    match &self.abort_handle {
      None => (),
      Some(handle) => handle.abort(),
    }
  }
}
