use std::time::Duration;

use async_trait::async_trait;
use sysinfo::{System, SystemExt};
use tokio::{
  sync::mpsc::Sender,
  task::{self, AbortHandle},
  time,
};

use super::{provider::Provider, provider_config::NetworkProviderConfig};

pub struct NetworkProvider {
  pub config: NetworkProviderConfig,
  abort_handle: Option<AbortHandle>,
}

impl NetworkProvider {
  pub fn new(config: NetworkProviderConfig) -> NetworkProvider {
    NetworkProvider {
      config,
      abort_handle: None,
    }
  }
}

#[async_trait]
impl Provider for NetworkProvider {
  async fn start(&mut self, output_sender: Sender<String>) {
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

    self.abort_handle = Some(forever.abort_handle());
    _ = forever.await;
  }

  async fn stop(&mut self) {
    match &self.abort_handle {
      None => (),
      Some(handle) => handle.abort(),
    }
  }
}
