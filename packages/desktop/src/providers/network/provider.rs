use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use sysinfo::Networks;
use tokio::{sync::Mutex, task::AbortHandle};

use crate::providers::{
  interval_provider::IntervalProvider,
  network::{NetworkInterface, NetworkVariables},
  variables::ProviderVariables,
};

use super::NetworkProviderConfig;

pub struct NetworkProvider {
  pub config: Arc<NetworkProviderConfig>,
  abort_handle: Option<AbortHandle>,
  networks: Arc<Mutex<Networks>>,
}

impl NetworkProvider {
  pub fn new(config: NetworkProviderConfig) -> NetworkProvider {
    NetworkProvider {
      config: Arc::new(config),
      abort_handle: None,
      networks: Arc::new(Mutex::new(Networks::new_with_refreshed_list())),
    }
  }
}

#[async_trait]
impl IntervalProvider for NetworkProvider {
  type Config = NetworkProviderConfig;
  type State = Mutex<Networks>;

  fn config(&self) -> Arc<NetworkProviderConfig> {
    self.config.clone()
  }

  fn state(&self) -> Arc<Mutex<Networks>> {
    self.networks.clone()
  }

  fn abort_handle(&self) -> &Option<AbortHandle> {
    &self.abort_handle
  }

  fn set_abort_handle(&mut self, abort_handle: AbortHandle) {
    self.abort_handle = Some(abort_handle)
  }

  async fn get_refreshed_variables(
    _: &NetworkProviderConfig,
    sysinfo: &Mutex<Networks>,
  ) -> Result<ProviderVariables> {
    let mut networks = sysinfo.lock().await;
    networks.refresh();

    let mut interfaces = vec![];

    for (name, data) in networks.into_iter() {
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
