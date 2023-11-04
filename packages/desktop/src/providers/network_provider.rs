use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use sysinfo::{System, SystemExt};
use tokio::{
  sync::{mpsc::Sender, Mutex},
  task::{self, AbortHandle},
  time,
};

use super::{provider::Provider, provider_config::NetworkProviderConfig};

pub struct NetworkProvider {
  pub config: NetworkProviderConfig,
  abort_handle: Option<AbortHandle>,
  sysinfo: Arc<Mutex<System>>,
}

impl NetworkProvider {
  pub fn new(
    config: NetworkProviderConfig,
    sysinfo: Arc<Mutex<System>>,
  ) -> NetworkProvider {
    NetworkProvider {
      config,
      abort_handle: None,
      sysinfo,
    }
  }
}

#[async_trait]
impl Provider for NetworkProvider {
  async fn on_start(&mut self, output_sender: Sender<String>) {
    let refresh_interval_ms = self.config.refresh_interval_ms;
    let sysinfo = self.sysinfo.clone();

    let forever = task::spawn(async move {
      let mut interval =
        time::interval(Duration::from_millis(refresh_interval_ms));

      loop {
        interval.tick().await;
        let mut sysinfo = sysinfo.lock().await;
        sysinfo.refresh_all();
        println!("hostname: {}", sysinfo.host_name().unwrap_or("".into()));

        _ = output_sender
          .send(format!(
            "hostname: {}",
            sysinfo.host_name().unwrap_or("".into())
          ))
          .await;
      }
    });

    self.abort_handle = Some(forever.abort_handle());
    _ = forever.await;
  }

  async fn on_refresh(&mut self, emit_output_tx: Sender<String>) {
  }

  async fn on_stop(&mut self) {
    if let Some(handle) = &self.abort_handle {
      handle.abort();
    }
  }
}
