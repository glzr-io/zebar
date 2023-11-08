use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use sysinfo::{System, SystemExt};
use tokio::{
  sync::{mpsc::Sender, Mutex},
  task::{self, AbortHandle},
  time,
};

use crate::providers::{provider::Provider, variables::ProviderVariables};

use super::{HostProviderConfig, HostVariables};

pub struct HostProvider {
  pub config: HostProviderConfig,
  abort_handle: Option<AbortHandle>,
  sysinfo: Arc<Mutex<System>>,
}

impl HostProvider {
  pub fn new(
    config: HostProviderConfig,
    sysinfo: Arc<Mutex<System>>,
  ) -> HostProvider {
    HostProvider {
      config,
      abort_handle: None,
      sysinfo,
    }
  }

  async fn refresh_and_emit(
    sysinfo: &Mutex<System>,
    emit_output_tx: &Sender<ProviderVariables>,
  ) {
    let sysinfo = sysinfo.lock().await;

    _ = emit_output_tx
      .send(ProviderVariables::Host(HostVariables {
        hostname: sysinfo.host_name(),
        os_name: sysinfo.name(),
        os_version: sysinfo.os_version(),
        friendly_os_version: sysinfo.long_os_version(),
        boot_time: sysinfo.boot_time() * 1000,
        uptime: sysinfo.uptime() * 1000,
      }))
      .await;
  }
}

#[async_trait]
impl Provider for HostProvider {
  async fn on_start(&mut self, emit_output_tx: Sender<ProviderVariables>) {
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

  async fn on_refresh(&mut self, emit_output_tx: Sender<ProviderVariables>) {
    Self::refresh_and_emit(&self.sysinfo, &emit_output_tx).await;
  }

  async fn on_stop(&mut self) {
    if let Some(handle) = &self.abort_handle {
      handle.abort();
    }
  }
}
