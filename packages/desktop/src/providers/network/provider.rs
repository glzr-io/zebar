use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use netdev::interface::{
  get_default_interface, get_interfaces, Interface,
};
use tokio::{sync::Mutex, task::AbortHandle};

use crate::providers::{
  interval_provider::IntervalProvider,
  network::parse_netsh::get_primary_interface_ssid_and_strength,
  variables::ProviderVariables,
};

use super::{
  Gateway, NetworkInterface, NetworkProviderConfig, NetworkVariables,
};

pub struct NetworkProvider {
  pub config: Arc<NetworkProviderConfig>,
  abort_handle: Option<AbortHandle>,
  netdev_data: Arc<Mutex<NetdevData>>,
}

pub struct NetdevData {
  pub default_interface: Arc<Mutex<Interface>>,
  pub interfaces: Arc<Mutex<Vec<Interface>>>,
}

impl NetworkProvider {
  pub fn new(config: NetworkProviderConfig) -> NetworkProvider {
    NetworkProvider {
      config: Arc::new(config),
      abort_handle: None,
      netdev_data: Arc::new(Mutex::new(NetdevData {
        default_interface: Arc::new(Mutex::new(
          get_default_interface().unwrap(),
        )),
        interfaces: Arc::new(Mutex::new(get_interfaces())),
      })),
    }
  }
}

#[async_trait]
impl IntervalProvider for NetworkProvider {
  type Config = NetworkProviderConfig;
  type State = Mutex<NetdevData>;

  fn config(&self) -> Arc<NetworkProviderConfig> {
    self.config.clone()
  }

  fn state(&self) -> Arc<Mutex<NetdevData>> {
    self.netdev_data.clone()
  }

  fn abort_handle(&self) -> &Option<AbortHandle> {
    &self.abort_handle
  }

  fn set_abort_handle(&mut self, abort_handle: AbortHandle) {
    self.abort_handle = Some(abort_handle)
  }

  async fn get_refreshed_variables(
    _: &NetworkProviderConfig,
    netdev_data: &Mutex<NetdevData>,
  ) -> Result<ProviderVariables> {
    let netdev = netdev_data.lock().await;

    let default_interface = netdev.default_interface.lock().await.clone();

    let interfaces = netdev.interfaces.lock().await;

    let default_gateway_ssid_and_strength =
      get_primary_interface_ssid_and_strength()?; // Returns ssid = None, signal = None, connected = false if not on Windows for now

    let variables = NetworkVariables {
      default_interface: NetworkInterface {
        name: default_interface.name.clone(),
        friendly_name: default_interface.friendly_name.clone(),
        description: default_interface.description.clone(),
        interface_type: default_interface.if_type,
        ipv4: default_interface.ipv4.clone(),
        ipv6: default_interface.ipv6.clone(),
        mac_address: default_interface.mac_addr.unwrap(),
        transmit_speed: default_interface.transmit_speed,
        receive_speed: default_interface.receive_speed,
        dns_servers: default_interface.dns_servers.clone(),
        is_default: default_interface.default,
      },
      default_gateway: Gateway {
        mac_address: default_interface
          .gateway
          .as_ref()
          .unwrap()
          .mac_addr
          .clone(),
        ipv4_addresses: default_interface.gateway.as_ref().unwrap().ipv4.clone(),
        ipv6_addresses: default_interface.gateway.as_ref().unwrap().ipv6.clone(),
        ssid: default_gateway_ssid_and_strength.ssid.unwrap(),
        signal_strength_percent: default_gateway_ssid_and_strength.signal.unwrap(),
        is_connected: default_gateway_ssid_and_strength.connected,
      },
      interfaces: interfaces
        .iter()
        .map(|iface| NetworkInterface {
          name: iface.name.clone(),
          friendly_name: iface.friendly_name.clone(),
          description: iface.description.clone(),
          interface_type: iface.if_type.clone(),
          ipv4: iface.ipv4.clone(),
          ipv6: iface.ipv6.clone(),
          mac_address: iface.mac_addr.unwrap().clone(),
          transmit_speed: iface.transmit_speed,
          receive_speed: iface.receive_speed,
          dns_servers: iface.dns_servers.clone(),
          is_default: iface.default,
        })
        .collect(),
    };

    Ok(ProviderVariables::Network(variables))
  }
}
