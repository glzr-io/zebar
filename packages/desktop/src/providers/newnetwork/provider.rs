use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use default_net::gateway::Gateway;
use wifiscanner::Wifi;
use tokio::{sync::Mutex, task::AbortHandle};

use crate::providers::{
  interval_provider::IntervalProvider, variables::ProviderVariables
};

use super::{NewNetworkProviderConfig, NewNetworkVariables};

pub struct NewNetworkProvider {
  pub config: Arc<NewNetworkProviderConfig>,
  abort_handle: Option<AbortHandle>,
  net_elements: Arc<(Mutex<Gateway>, Mutex<Vec<Wifi>>)>,
}

impl NewNetworkProvider {
  pub fn new(config: NewNetworkProviderConfig) -> NewNetworkProvider {
    NewNetworkProvider {
      config: Arc::new(config),
      abort_handle: None,
      net_elements: Arc::new((
        Mutex::new(default_net::get_default_gateway().unwrap()), 
        Mutex::new(wifiscanner::scan().unwrap()),
      ))
    }
  }
}

#[async_trait]
impl IntervalProvider for NewNetworkProvider {
  type Config = NewNetworkProviderConfig;
  type State = (Mutex<Gateway>, Mutex<Vec<Wifi>>);

  fn config(&self) -> Arc<NewNetworkProviderConfig> {
    self.config.clone()
  }

  fn state(&self) -> Arc<(Mutex<Gateway>, Mutex<Vec<Wifi>>)> {
    self.net_elements.clone()
  }

  fn abort_handle(&self) -> &Option<AbortHandle> {
    &self.abort_handle
  }

  fn set_abort_handle(&mut self, abort_handle: AbortHandle) {
    self.abort_handle = Some(abort_handle)
  }

  async fn get_refreshed_variables(
    _: &NewNetworkProviderConfig,
    net_elements: &(Mutex<Gateway>, Mutex<Vec<Wifi>>),
  ) -> Result<ProviderVariables> {
    let default_net = net_elements.0.lock().await;
    let wifis = net_elements.1.lock().await;

    let matched_network = wifis.iter().find(|network| network.mac == default_net.mac_addr.to_string()).unwrap();

    let mut network_variables = NewNetworkVariables {
      name: "".to_string(),
      mac_address: "".to_string(),
      strength: "".to_string(),
    };

    network_variables.mac_address = matched_network.mac.to_owned();
    network_variables.strength = matched_network.signal_level.to_owned();
    network_variables.name = matched_network.ssid.to_owned();
    
    Ok(ProviderVariables::NewNetwork(NewNetworkVariables { ..network_variables }))
  }
}
