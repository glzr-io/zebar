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
  abort_handle: Arc<Mutex<Option<AbortHandle>>>,
}

impl NetworkProvider {
  pub fn new(config: NetworkProviderConfig) -> NetworkProvider {
    NetworkProvider {
      config,
      abort_handle: Arc::new(Mutex::new(None)),
    }
  }
}

#[async_trait]
impl Provider for NetworkProvider {
  async fn start(&self, output_sender: Sender<String>) {
    let refresh_interval = self.config.refresh_interval_ms;

    let forever = task::spawn(async move {
      let mut interval =
        time::interval(Duration::from_millis(refresh_interval));
      let mut sys = System::new_all();

      loop {
        interval.tick().await;
        sys.refresh_all();
        println!("hostname: {}", sys.host_name().unwrap_or("".into()));

        _ = output_sender
          .send(format!(
            "hostname: {}",
            sys.host_name().unwrap_or("".into())
          ))
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
