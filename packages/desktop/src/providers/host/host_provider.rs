use serde::{Deserialize, Serialize};
use sysinfo::System;

use crate::{
  common::SyncInterval,
  providers::{CommonProviderState, Provider, RuntimeType},
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HostProviderConfig {
  pub refresh_interval: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HostOutput {
  pub hostname: Option<String>,
  pub os_name: Option<String>,
  pub os_version: Option<String>,
  pub friendly_os_version: Option<String>,
  pub boot_time: u64,
  pub uptime: u64,
}

pub struct HostProvider {
  config: HostProviderConfig,
  common: CommonProviderState,
}

impl HostProvider {
  pub fn new(
    config: HostProviderConfig,
    common: CommonProviderState,
  ) -> HostProvider {
    HostProvider { config, common }
  }

  fn run_interval(&mut self) -> anyhow::Result<HostOutput> {
    Ok(HostOutput {
      hostname: System::host_name(),
      os_name: System::name(),
      os_version: System::os_version(),
      friendly_os_version: System::long_os_version(),
      boot_time: System::boot_time() * 1000,
      uptime: System::uptime() * 1000,
    })
  }
}

impl Provider for HostProvider {
  fn runtime_type(&self) -> RuntimeType {
    RuntimeType::Sync
  }

  fn start_sync(&mut self) {
    let mut interval = SyncInterval::new(self.config.refresh_interval);

    loop {
      interval.tick();

      let output = self.run_interval();
      self.common.emitter.emit_output(output);
    }
  }
}
