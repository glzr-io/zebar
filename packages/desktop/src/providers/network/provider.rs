use std::sync::Arc;

use async_trait::async_trait;
use sysinfo::Networks;
use netdev::interface::{
   get_interfaces,
};
use tokio::{sync::Mutex, task::AbortHandle};

use super::{
  wifi_hotspot::{default_gateway_wifi, WifiHotstop},
  InterfaceType, NetworkGateway, NetworkInterface, NetworkProviderConfig,
  NetworkVariables,
  NetworkTraffic,
};
use crate::providers::{
  interval_provider::IntervalProvider, variables::ProviderVariables,
};

pub struct NetworkProvider {
  pub config: Arc<NetworkProviderConfig>,
  abort_handle: Option<AbortHandle>,
  _state: Arc<Mutex<()>>,
  netinfo: Arc<Mutex<Networks>>,
}

impl NetworkProvider {
  pub fn new(config: NetworkProviderConfig, netinfo: Arc<Mutex<Networks>>,) -> NetworkProvider {
    NetworkProvider {
      config: Arc::new(config),
      abort_handle: None,
      _state: Arc::new(Mutex::new(())),
      netinfo,
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
  type State = Mutex<Networks>;

  fn config(&self) -> Arc<NetworkProviderConfig> {
    self.config.clone()
  }

  fn state(&self) -> Arc<Mutex<Networks>> {
    self.netinfo.clone()
  }

  fn abort_handle(&self) -> &Option<AbortHandle> {
    &self.abort_handle
  }

  fn set_abort_handle(&mut self, abort_handle: AbortHandle) {
    self.abort_handle = Some(abort_handle)
  }

  async fn get_refreshed_variables(
    config: &NetworkProviderConfig,
    netinfo: &Mutex<Networks>,
  ) -> anyhow::Result<ProviderVariables> {
    let mut netinfo = netinfo.lock().await;
    netinfo.refresh();

    let interfaces = get_interfaces();

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
      traffic: NetworkTraffic {
        received: to_bytes_per_seconds(get_network_down(&netinfo), config.refresh_interval),
        transmitted: to_bytes_per_seconds(get_network_up(&netinfo), config.refresh_interval),
      },     
    };

    Ok(ProviderVariables::Network(variables))
  }
}

// Get the total network (down) usage
fn get_network_down(req_net: &sysinfo::Networks) -> u64 {
    // Get the total bytes recieved by every network interface
    let mut received_total: Vec<u64> = Vec::new();
    for (_interface_name, network) in req_net {
        received_total.push(network.received() as u64);
    }

    received_total.iter().sum()
}

// Get the total network (up) usage
fn get_network_up(req_net: &sysinfo::Networks) -> u64 {
    // Get the total bytes recieved by every network interface
    let mut transmitted_total: Vec<u64> = Vec::new();
    for (_interface_name, network) in req_net {
        transmitted_total.push(network.transmitted() as u64);
    }

    transmitted_total.iter().sum()
}

fn to_bytes_per_seconds(input_in_bytes: u64, timespan_in_ms: u64) -> u64 {
    input_in_bytes / (timespan_in_ms / 1000)
}
