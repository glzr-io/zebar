use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use sysinfo::System;
use tokio::{
  sync::{mpsc, Mutex},
  time,
};

use super::{HostOutput, HostProviderConfig};
use crate::providers::{
  provider::Provider, provider_ref::ProviderResult,
  variables::ProviderOutput,
};

pub struct HostProvider {
  config: HostProviderConfig,
  sysinfo: Arc<Mutex<System>>,
}

impl HostProvider {
  pub fn new(
    config: HostProviderConfig,
    sysinfo: Arc<Mutex<System>>,
  ) -> HostProvider {
    HostProvider { config, sysinfo }
  }

  async fn interval_output() -> anyhow::Result<ProviderOutput> {
    Ok(ProviderOutput::Host(HostOutput {
      hostname: System::host_name(),
      os_name: System::name(),
      os_version: System::os_version(),
      friendly_os_version: System::long_os_version(),
      boot_time: System::boot_time() * 1000,
      uptime: System::uptime() * 1000,
    }))
  }
}

#[async_trait]
impl Provider for HostProvider {
  async fn on_start(&self, emit_result_tx: mpsc::Sender<ProviderResult>) {
    let mut interval =
      time::interval(Duration::from_millis(self.config.refresh_interval));

    loop {
      interval.tick().await;

      emit_result_tx
        .send(Self::interval_output().await.into())
        .await;
    }
  }
}
