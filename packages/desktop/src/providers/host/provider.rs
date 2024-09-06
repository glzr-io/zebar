use sysinfo::System;

use super::{HostOutput, HostProviderConfig};
use crate::{
  impl_interval_provider, providers::ProviderOutput,
};

pub struct HostProvider {
  config: HostProviderConfig,
}

impl HostProvider {
  pub fn new(config: HostProviderConfig) -> HostProvider {
    HostProvider { config }
  }

  fn refresh_interval_ms(&self) -> u64 {
    self.config.refresh_interval
  }

  async fn run_interval(&self) -> anyhow::Result<ProviderOutput> {
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

impl_interval_provider!(HostProvider);
