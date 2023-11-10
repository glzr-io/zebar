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

use super::{MemoryProviderConfig, MemoryVariables};

pub struct MemoryProvider {
  pub config: MemoryProviderConfig,
  abort_handle: Option<AbortHandle>,
  sysinfo: Arc<Mutex<System>>,
}

impl MemoryProvider {
  pub fn new(
    config: MemoryProviderConfig,
    sysinfo: Arc<Mutex<System>>,
  ) -> MemoryProvider {
    MemoryProvider {
      config,
      abort_handle: None,
      sysinfo,
    }
  }
}

#[async_trait]
impl IntervalProvider for MemoryProvider {
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
    let mut sysinfo = sysinfo.lock().await;
    sysinfo.refresh_memory();

    _ = emit_output_tx
      .send(ProviderVariables::Memory(MemoryVariables {
        free_memory: sysinfo.free_memory(),
        used_memory: sysinfo.used_memory(),
        total_memory: sysinfo.total_memory(),
        free_swap: sysinfo.free_swap(),
        used_swap: sysinfo.used_swap(),
        total_swap: sysinfo.total_swap(),
      }))
      .await;
  }
}
