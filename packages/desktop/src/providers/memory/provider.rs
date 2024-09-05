use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use sysinfo::System;
use tokio::{
  sync::{mpsc, Mutex},
  time,
};

use super::{MemoryOutput, MemoryProviderConfig};
use crate::providers::{
  provider::Provider, provider_ref::ProviderResult,
  variables::ProviderOutput,
};

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

  async fn interval_output(
    sysinfo: Arc<Mutex<System>>,
  ) -> anyhow::Result<ProviderOutput> {
    let mut sysinfo = sysinfo.lock().await;
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

#[async_trait]
impl Provider for MemoryProvider {
  async fn on_start(
    self: Arc<Self>,
    emit_result_tx: mpsc::Sender<ProviderResult>,
  ) {
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
