use std::sync::Arc;

use async_trait::async_trait;
use sysinfo::{System, SystemExt};
use tokio::{
  sync::{mpsc::Sender, Mutex},
  task::AbortHandle,
};

use crate::providers::{
  interval_provider::IntervalProvider, variables::ProviderVariables,
};

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
}

#[async_trait]
impl IntervalProvider for HostProvider {
  type State = System;

  fn refresh_interval_ms(&self) -> u64 {
    self.config.refresh_interval_ms
  }

  fn state(&self) -> Arc<Mutex<System>> {
    self.sysinfo.clone()
  }

  fn abort_handle(&self) -> &Option<AbortHandle> {
    &self.abort_handle
  }

  fn set_abort_handle(&mut self, abort_handle: AbortHandle) {
    self.abort_handle = Some(abort_handle)
  }

  async fn refresh_and_emit(
    emit_output_tx: &Sender<ProviderVariables>,
    sysinfo: &Mutex<System>,
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
