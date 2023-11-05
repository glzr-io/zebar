use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use sysinfo::{CpuExt, RefreshKind, System, SystemExt};
use tokio::{
  sync::{mpsc::Sender, Mutex},
  task::{self, AbortHandle},
  time,
};

use crate::providers::{provider::Provider, variables::ProviderVariables};

use super::{CpuProviderConfig, CpuVariables};

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
    emit_output_tx: &Sender<ProviderVariables>,
  ) {
    let mut sysinfo = sysinfo.lock().await;
    sysinfo.refresh_cpu();

    _ = emit_output_tx
      .send(ProviderVariables::Cpu(CpuVariables {
        usage: sysinfo.global_cpu_info().cpu_usage(),
        frequency: sysinfo.global_cpu_info().frequency(),
        logical_core_count: sysinfo.cpus().len(),
        physical_core_count: sysinfo
          .physical_core_count()
          .unwrap_or(sysinfo.cpus().len()),
        brand: sysinfo.global_cpu_info().brand().into(),
        vendor_id: sysinfo.global_cpu_info().vendor_id().into(),
      }))
      .await;
  }
}

#[async_trait]
impl Provider for CpuProvider {
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
