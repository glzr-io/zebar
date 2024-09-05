use std::sync::Arc;

use async_trait::async_trait;
use sysinfo::System;
use tokio::{sync::Mutex, task::AbortHandle};

use super::{HostProviderConfig, HostOutput};
use crate::providers::{
  provider::IntervalProvider, variables::ProviderOutput,
};

pub struct HostProvider {
  pub config: Arc<HostProviderConfig>,
  abort_handle: Option<AbortHandle>,
  sysinfo: Arc<Mutex<System>>,
}

impl HostProvider {
  pub fn new(
    config: HostProviderConfig,
    sysinfo: Arc<Mutex<System>>,
  ) -> HostProvider {
    HostProvider {
      config: Arc::new(config),
      abort_handle: None,
      sysinfo,
    }
  }
}

#[async_trait]
impl IntervalProvider for HostProvider {
  type Config = HostProviderConfig;
  type State = Mutex<System>;

  fn config(&self) -> Arc<HostProviderConfig> {
    self.config.clone()
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
    _: &HostProviderConfig,
    __: &Mutex<System>,
  ) -> anyhow::Result<ProviderOutput> {
    Ok(ProviderOutput::Host(HostOutput {
      hostname: System::host_name(),
      os_name: System::name(),
      os_version: System::os_version(),
      friendly_os_version: System::long_os_version(),
      boot_time: System::boot_time() * 1000,
      uptime: System::uptime() * 1000,
    }))
  }
}
