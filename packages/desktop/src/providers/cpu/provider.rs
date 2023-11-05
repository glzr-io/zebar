use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use sysinfo::{System, SystemExt};
use tokio::{
  sync::{mpsc::Sender, Mutex},
  task::{self, AbortHandle},
  time,
};

use crate::providers::provider::Provider;

use super::CpuProviderConfig;

pub struct CpuProvider {
  pub config: CpuProviderConfig,
  abort_handle: Option<AbortHandle>,
  sysinfo: Arc<Mutex<System>>,
}

impl CpuProvider {
  pub fn new(
    config: CpuProviderConfig,
    sysinfo: Arc<Mutex<System>>,
  ) -> CpuProvider {
    CpuProvider {
      config,
      abort_handle: None,
      sysinfo,
    }
  }

  async fn refresh_and_emit(
    sysinfo: &Mutex<System>,
    emit_output_tx: &Sender<String>,
  ) {
    let sysinfo = sysinfo.lock().await;

    _ = emit_output_tx
      .send(format!("total memory: {}", sysinfo.total_memory()))
      .await;
  }
}

#[async_trait]
impl Provider for CpuProvider {
  async fn on_start(&mut self, emit_output_tx: Sender<String>) {
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

  async fn on_refresh(&mut self, emit_output_tx: Sender<String>) {
    Self::refresh_and_emit(&self.sysinfo, &emit_output_tx).await;
  }

  async fn on_stop(&mut self) {
    if let Some(handle) = &self.abort_handle {
      handle.abort();
    }
  }
}
