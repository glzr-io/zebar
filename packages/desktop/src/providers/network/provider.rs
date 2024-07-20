use std::sync::Arc;

use async_trait::async_trait;
use tokio::{sync::Mutex, task::AbortHandle};

use super::{
  wifi_hotspot::{default_gateway_wifi, WifiHotstop},
  InterfaceType, NetworkGateway, NetworkInterface, NetworkProviderConfig,
  NetworkVariables,
};
use crate::providers::{
  interval_provider::IntervalProvider, variables::ProviderVariables,
};

pub struct NetworkProvider {
  pub config: Arc<NetworkProviderConfig>,
  abort_handle: Option<AbortHandle>,
  _state: Arc<Mutex<()>>,
}

impl NetworkProvider {
  pub fn new(config: NetworkProviderConfig) -> NetworkProvider {
    NetworkProvider {
      config: Arc::new(config),
      abort_handle: None,
      _state: Arc::new(Mutex::new(())),
    }
  }

  fn transform_interface(
    interface: &netdev::Interface,
  ) -> NetworkInterface {
    NetworkInterface {
      name: interface.name.to_string(),
      friendly_name: interface.friendly_name.clone(),
      description: interface.description.clone(),
      interface_type: InterfaceType::from(interface.if_type),
      ipv4_addresses: interface
        .ipv4
        .iter()
        .map(|ip| ip.to_string())
        .collect(),
      ipv6_addresses: interface
        .ipv6
        .iter()
        .map(|ip| ip.to_string())
        .collect(),
      mac_address: interface.mac_addr.map(|mac| mac.to_string()),
      transmit_speed: interface.transmit_speed,
      receive_speed: interface.receive_speed,
      dns_servers: interface
        .dns_servers
        .iter()
        .map(|ip| ip.to_string())
        .collect(),
      is_default: interface.default,
    }
  }

  fn transform_gateway(
    gateway: &netdev::NetworkDevice,
    wifi_hotspot: WifiHotstop,
  ) -> NetworkGateway {
    NetworkGateway {
      mac_address: gateway.mac_addr.address(),
      ipv4_addresses: gateway
        .ipv4
        .iter()
        .map(|ip| ip.to_string())
        .collect(),
      ipv6_addresses: gateway
        .ipv6
        .iter()
        .map(|ip| ip.to_string())
        .collect(),
      ssid: wifi_hotspot.ssid,
      signal_strength: wifi_hotspot.signal_strength,
    }
  }
}

#[async_trait]
impl IntervalProvider for NetworkProvider {
  type Config = NetworkProviderConfig;
  type State = ();

  fn config(&self) -> Arc<NetworkProviderConfig> {
    self.config.clone()
  }

  fn state(&self) -> Arc<()> {
    Arc::new(())
  }

  fn abort_handle(&self) -> &Option<AbortHandle> {
    &self.abort_handle
  }

  fn set_abort_handle(&mut self, abort_handle: AbortHandle) {
    self.abort_handle = Some(abort_handle)
  }

  async fn get_refreshed_variables(
    _: &NetworkProviderConfig,
    _state: &(),
  ) -> anyhow::Result<ProviderVariables> {
    let interfaces = netdev::get_interfaces();

    let default_interface = netdev::get_default_interface().ok();

    let variables = NetworkVariables {
      default_interface: default_interface
        .as_ref()
        .map(Self::transform_interface),
      default_gateway: default_interface
        .and_then(|interface| interface.gateway)
        .and_then(|gateway| {
          default_gateway_wifi()
            .map(|wifi| Self::transform_gateway(&gateway, wifi))
            .ok()
        }),
      interfaces: interfaces
        .iter()
        .map(Self::transform_interface)
        .collect(),
    };

    Ok(ProviderVariables::Network(variables))
  }
}
