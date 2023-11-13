use std::sync::Arc;

use async_trait::async_trait;
use sysinfo::{CpuExt, System, SystemExt};
use tokio::{sync::Mutex, task::AbortHandle};

use crate::providers::{
  interval_provider::IntervalProvider, variables::ProviderVariables,
};

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
}

#[async_trait]
impl IntervalProvider for CpuProvider {
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

  async fn get_refreshed_variables(
    sysinfo: &Mutex<System>,
  ) -> ProviderVariables {
    let mut sysinfo = sysinfo.lock().await;
    sysinfo.refresh_cpu();

    ProviderVariables::Cpu(CpuVariables {
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
