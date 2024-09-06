use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use sysinfo::System;
use tokio::{
  sync::{mpsc, Mutex},
  time,
};

use super::{CpuOutput, CpuProviderConfig};
use crate::providers::{
  provider::Provider, provider_ref::ProviderResult,
  variables::ProviderOutput,
};

pub struct CpuProvider {
  config: CpuProviderConfig,
  sysinfo: Arc<Mutex<System>>,
}

impl CpuProvider {
  pub fn new(
    config: CpuProviderConfig,
    sysinfo: Arc<Mutex<System>>,
  ) -> CpuProvider {
    CpuProvider { config, sysinfo }
  }
}

#[async_trait]
impl Provider for CpuProvider {
  async fn on_start(&self, emit_result_tx: mpsc::Sender<ProviderResult>) {
    let mut interval =
      time::interval(Duration::from_millis(self.config.refresh_interval));

    loop {
      interval.tick().await;

      let mut sysinfo = self.sysinfo.lock().await;
      sysinfo.refresh_cpu();

      let res = Ok(ProviderOutput::Cpu(CpuOutput {
        usage: sysinfo.global_cpu_info().cpu_usage(),
        frequency: sysinfo.global_cpu_info().frequency(),
        logical_core_count: sysinfo.cpus().len(),
        physical_core_count: sysinfo
          .physical_core_count()
          .unwrap_or(sysinfo.cpus().len()),
        vendor: sysinfo.global_cpu_info().vendor_id().into(),
      }));

      emit_result_tx.send(res.into()).await;
    }
  }
}
