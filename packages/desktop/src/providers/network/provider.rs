use std::sync::Arc;

use netdev::interface::get_interfaces;
use sysinfo::Networks;
use tokio::sync::Mutex;

use super::{
  wifi_hotspot::{default_gateway_wifi, WifiHotstop},
  InterfaceType, NetworkGateway, NetworkInterface, NetworkOutput,
  NetworkProviderConfig, NetworkTraffic,
};
use crate::{
  impl_interval_provider, providers::ProviderOutput,
};

pub struct NetworkProvider {
  config: NetworkProviderConfig,
  netinfo: Arc<Mutex<Networks>>,
}

impl NetworkProvider {
  pub fn new(
    config: NetworkProviderConfig,
    netinfo: Arc<Mutex<Networks>>,
  ) -> NetworkProvider {
    NetworkProvider { config, netinfo }
  }

  fn refresh_interval_ms(&self) -> u64 {
    self.config.refresh_interval
  }

  async fn run_interval(&self) -> anyhow::Result<ProviderOutput> {
    let mut netinfo = self.netinfo.lock().await;
    netinfo.refresh();

    let interfaces = get_interfaces();

    let default_interface = netdev::get_default_interface().ok();

    Ok(ProviderOutput::Network(NetworkOutput {
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
        received: to_bytes_per_seconds(
          get_network_down(&netinfo),
          self.config.refresh_interval,
        ),
        transmitted: to_bytes_per_seconds(
          get_network_up(&netinfo),
          self.config.refresh_interval,
        ),
      },
    }))
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

impl_interval_provider!(NetworkProvider);

/// Gets the total network (down) usage.
fn get_network_down(req_net: &sysinfo::Networks) -> u64 {
  // Get the total bytes recieved by every network interface
  let mut received_total: Vec<u64> = Vec::new();
  for (_interface_name, network) in req_net {
    received_total.push(network.received() as u64);
  }

  received_total.iter().sum()
}

/// Gets the total network (up) usage.
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
