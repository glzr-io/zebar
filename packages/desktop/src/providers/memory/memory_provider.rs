use serde::{Deserialize, Serialize};

use crate::{
  common::SyncInterval,
  providers::{CommonProviderState, Provider, RuntimeType},
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MemoryProviderConfig {
  pub refresh_interval: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryOutput {
  pub usage: f32,
  pub free_memory: u64,
  pub used_memory: u64,
  pub total_memory: u64,
  pub free_swap: u64,
  pub used_swap: u64,
  pub total_swap: u64,
}

pub struct MemoryProvider {
  config: MemoryProviderConfig,
  common: CommonProviderState,
}

impl MemoryProvider {
  pub fn new(
    config: MemoryProviderConfig,
    common: CommonProviderState,
  ) -> MemoryProvider {
    MemoryProvider { config, common }
  }

  fn run_interval(&mut self) -> anyhow::Result<MemoryOutput> {
    let mut sysinfo = self.common.sysinfo.blocking_lock();
    sysinfo.refresh_memory();

    let usage = (sysinfo.used_memory() as f32
      / sysinfo.total_memory() as f32)
      * 100.0;

    Ok(MemoryOutput {
      usage,
      free_memory: sysinfo.free_memory(),
      used_memory: sysinfo.used_memory(),
      total_memory: sysinfo.total_memory(),
      free_swap: sysinfo.free_swap(),
      used_swap: sysinfo.used_swap(),
      total_swap: sysinfo.total_swap(),
    })
  }
}

impl Provider for MemoryProvider {
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
