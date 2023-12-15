use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use sysinfo::{NetworkExt, System, SystemExt};
use tokio::{sync::Mutex, task::AbortHandle};

use crate::providers::{
  interval_provider::IntervalProvider,
  network::{NetworkInterface, NetworkVariables},
  variables::ProviderVariables,
};

use super::NetworkProviderConfig;

pub struct NetworkProvider {
  pub config: NetworkProviderConfig,
  abort_handle: Option<AbortHandle>,
  sysinfo: Arc<Mutex<System>>,
}

impl NetworkProvider {
  pub fn new(
    config: NetworkProviderConfig,
    sysinfo: Arc<Mutex<System>>,
  ) -> NetworkProvider {
    NetworkProvider {
      config,
      abort_handle: None,
      sysinfo,
    }
  }
}

#[async_trait]
impl IntervalProvider for NetworkProvider {
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
  ) -> Result<ProviderVariables> {
    let mut sysinfo = sysinfo.lock().await;
    sysinfo.refresh_networks();

    let mut interfaces = vec![];

    for (name, data) in sysinfo.networks() {
      interfaces.push(NetworkInterface {
        name: name.into(),
        mac_address: data.mac_address().to_string(),
        transmitted: data.transmitted(),
        total_transmitted: data.total_transmitted(),
        received: data.received(),
        total_received: data.total_received(),
      });
    }

    Ok(ProviderVariables::Network(NetworkVariables { interfaces }))
  }
}
