use std::sync::Arc;

use anyhow::Context;
use netdev::interface::get_interfaces;
use sysinfo::Networks;
use tokio::sync::Mutex;

use super::{
  wifi_hotspot::{default_gateway_wifi, WifiHotstop},
  InterfaceType, NetworkGateway, NetworkInterface, NetworkOutput,
  NetworkProviderConfig, NetworkTraffic, NetworkTrafficMeasure,
};
use crate::{
  common::{to_iec_bytes, to_si_bytes},
  impl_interval_provider,
  providers::ProviderOutput,
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

    let received_per_sec = Self::total_bytes_received(&netinfo)
      / self.config.refresh_interval
      * 1000;

    let transmitted_per_sec = Self::total_bytes_transmitted(&netinfo)
      / self.config.refresh_interval
      * 1000;

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
        received: Self::to_network_traffic_measure(received_per_sec)?,
        transmitted: Self::to_network_traffic_measure(
          transmitted_per_sec,
        )?,
      },
    }))
  }

  fn to_network_traffic_measure(
    bytes: u64,
  ) -> anyhow::Result<NetworkTrafficMeasure> {
    let (si_value, si_unit) = to_si_bytes(bytes as f64);
    let (iec_value, iec_unit) = to_iec_bytes(bytes as f64);

    Ok(NetworkTrafficMeasure {
      bytes,
      si_value,
      si_unit,
      iec_value,
      iec_unit,
    })
  }

  /// Gets the total network (down) usage.
  ///
  /// Returns the total bytes received by every network interface.
  fn total_bytes_received(networks: &sysinfo::Networks) -> u64 {
    let mut received_total: Vec<u64> = Vec::new();

    for (_interface_name, network) in networks {
      received_total.push(network.received());
    }

    received_total.iter().sum()
  }

  /// Gets the total network (up) usage.
  ///
  /// Returns the total bytes transmitted by every network interface.
  fn total_bytes_transmitted(networks: &sysinfo::Networks) -> u64 {
    let mut transmitted_total: Vec<u64> = Vec::new();

    for (_interface_name, network) in networks {
      transmitted_total.push(network.transmitted());
    }

    transmitted_total.iter().sum()
  }

  /// Transforms a `netdev::Interface` into a `NetworkInterface`.
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

  /// Transforms a `netdev::NetworkDevice` into a `NetworkGateway`.
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
