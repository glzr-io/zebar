use serde::{Deserialize, Serialize};

use crate::{
  common::SyncInterval,
  providers::{CommonProviderState, Provider, RuntimeType},
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CpuProviderConfig {
  pub refresh_interval: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuOutput {
  pub frequency: u64,
  pub usage: f32,
  pub logical_core_count: usize,
  pub physical_core_count: usize,
  pub vendor: String,
}

pub struct CpuProvider {
  config: CpuProviderConfig,
  common: CommonProviderState,
}

impl CpuProvider {
  pub fn new(
    config: CpuProviderConfig,
    common: CommonProviderState,
  ) -> CpuProvider {
    CpuProvider { config, common }
  }

  fn run_interval(&self) -> anyhow::Result<CpuOutput> {
    let mut sysinfo = self.common.sysinfo.blocking_lock();
    sysinfo.refresh_cpu();

    Ok(CpuOutput {
      usage: sysinfo.global_cpu_info().cpu_usage(),
      frequency: sysinfo.global_cpu_info().frequency(),
      logical_core_count: sysinfo.cpus().len(),
      physical_core_count: sysinfo
        .physical_core_count()
        .unwrap_or(sysinfo.cpus().len()),
      vendor: sysinfo.global_cpu_info().vendor_id().into(),
    })
  }
}

impl Provider for CpuProvider {
  fn runtime_type(&self) -> RuntimeType {
    RuntimeType::Sync
  }

  fn start_sync(&mut self) {
    let mut interval = SyncInterval::new(self.config.refresh_interval);

    loop {
      interval.tick();

      let output = self.run_interval();
      self.common.emitter.emit_output(output);
    }
  }
}
