use std::sync::Arc;

use sysinfo::System;
use tokio::sync::Mutex;

use super::{CpuOutput, CpuProviderConfig};
use crate::{
  impl_interval_provider, providers::variables::ProviderOutput,
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

  fn refresh_interval_ms(&self) -> u64 {
    self.config.refresh_interval
  }

  async fn run_interval(&self) -> anyhow::Result<ProviderOutput> {
    let mut sysinfo = self.sysinfo.lock().await;
    sysinfo.refresh_cpu();

    Ok(ProviderOutput::Cpu(CpuOutput {
      usage: sysinfo.global_cpu_info().cpu_usage(),
      frequency: sysinfo.global_cpu_info().frequency(),
      logical_core_count: sysinfo.cpus().len(),
      physical_core_count: sysinfo
        .physical_core_count()
        .unwrap_or(sysinfo.cpus().len()),
      vendor: sysinfo.global_cpu_info().vendor_id().into(),
    }))
  }
}

impl_interval_provider!(CpuProvider);
