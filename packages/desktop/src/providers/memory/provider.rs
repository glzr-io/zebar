use std::sync::Arc;

use sysinfo::System;
use tokio::sync::Mutex;

use super::{MemoryOutput, MemoryProviderConfig};
use crate::{impl_interval_provider, providers::ProviderOutput};

pub struct MemoryProvider {
  config: MemoryProviderConfig,
  sysinfo: Arc<Mutex<System>>,
}

impl MemoryProvider {
  pub fn new(
    config: MemoryProviderConfig,
    sysinfo: Arc<Mutex<System>>,
  ) -> MemoryProvider {
    MemoryProvider { config, sysinfo }
  }

  fn refresh_interval_ms(&self) -> u64 {
    self.config.refresh_interval
  }

  async fn run_interval(&self) -> anyhow::Result<ProviderOutput> {
    let mut sysinfo = self.sysinfo.lock().await;
    sysinfo.refresh_memory();

    let usage = (sysinfo.used_memory() as f32
      / sysinfo.total_memory() as f32)
      * 100.0;

    Ok(ProviderOutput::Memory(MemoryOutput {
      usage,
      free_memory: sysinfo.free_memory(),
      used_memory: sysinfo.used_memory(),
      total_memory: sysinfo.total_memory(),
      free_swap: sysinfo.free_swap(),
      used_swap: sysinfo.used_swap(),
      total_swap: sysinfo.total_swap(),
    }))
  }
}

impl_interval_provider!(MemoryProvider, true);
