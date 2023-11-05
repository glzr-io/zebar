use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use sysinfo::{System, SystemExt};
use tokio::{
  sync::{mpsc::Sender, Mutex},
  task::{self, AbortHandle},
  time,
};

use crate::providers::{
  provider::Provider, provider_config::NetworkProviderConfig,
};

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

  async fn refresh_and_emit(
    sysinfo: &Mutex<System>,
    emit_output_tx: &Sender<String>,
  ) {
    let sysinfo = sysinfo.lock().await;
    println!("hostname: {}", sysinfo.host_name().unwrap_or("".into()));
    // sysinfo.networks().into_iter().map(|e|e.1.)

    // Network interfaces name, data received and data transmitted:
    let networks = sysinfo.networks();
    println!("=> networks:");
    for (interface_name, data) in networks {
      println!("{}", interface_name);
      println!("{:#?}", data);
    }

    _ = emit_output_tx
      .send(format!(
        "hostname: {}",
        sysinfo.host_name().unwrap_or("".into())
      ))
      .await;
  }
}

#[async_trait]
impl Provider for NetworkProvider {
  async fn on_start(&mut self, emit_output_tx: Sender<String>) {
    let refresh_interval_ms = self.config.refresh_interval_ms;
    let sysinfo = self.sysinfo.clone();

    let forever = task::spawn(async move {
      let mut interval =
        time::interval(Duration::from_millis(refresh_interval_ms));

      Self::refresh_and_emit(&sysinfo, &emit_output_tx).await;

      loop {
        interval.tick().await;
        Self::refresh_and_emit(&sysinfo, &emit_output_tx).await;
      }
    });

    self.abort_handle = Some(forever.abort_handle());
    _ = forever.await;
  }

  async fn on_refresh(&mut self, emit_output_tx: Sender<String>) {
    Self::refresh_and_emit(&self.sysinfo, &emit_output_tx).await;
  }

  async fn on_stop(&mut self) {
    if let Some(handle) = &self.abort_handle {
      handle.abort();
    }
  }
}
