use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use sysinfo::{System, SystemExt};
use tokio::{
  sync::{mpsc::Sender, Mutex},
  task::{self, AbortHandle},
  time,
};

use crate::providers::{provider::Provider, variables::ProviderVariables};

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

  async fn refresh_and_emit(
    sysinfo: &Mutex<System>,
    emit_output_tx: &Sender<ProviderVariables>,
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

#[async_trait]
impl Provider for MemoryProvider {
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
