use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use sysinfo::System;
use tokio::{
  sync::{mpsc, Mutex},
  task::AbortHandle,
  time,
};

use super::{CpuProviderConfig, CpuVariables};
use crate::providers::{
  provider::Provider, provider_manager::SharedProviderState,
  provider_ref::VariablesResult, variables::ProviderVariables,
};

pub struct CpuProvider {
  config: CpuProviderConfig,
}

impl CpuProvider {
  pub fn new(config: CpuProviderConfig) -> CpuProvider {
    CpuProvider { config }
  }
}

#[async_trait]
impl Provider for CpuProvider {
  async fn on_start(
    self: Arc<Self>,
    shared_state: SharedProviderState,
    emit_output_tx: mpsc::Sender<VariablesResult>,
  ) {
    let mut interval =
      time::interval(Duration::from_millis(self.config.refresh_interval));

    loop {
      interval.tick().await;

      let mut sysinfo = shared_state.sysinfo.lock().await;
      sysinfo.refresh_cpu();

      let res = Ok(ProviderVariables::Cpu(CpuVariables {
        usage: sysinfo.global_cpu_info().cpu_usage(),
        frequency: sysinfo.global_cpu_info().frequency(),
        logical_core_count: sysinfo.cpus().len(),
        physical_core_count: sysinfo
          .physical_core_count()
          .unwrap_or(sysinfo.cpus().len()),
        vendor: sysinfo.global_cpu_info().vendor_id().into(),
      }));

      emit_output_tx.send(res.into()).await;
    }
  }
}
