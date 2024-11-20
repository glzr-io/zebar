use serde::{Deserialize, Serialize};
use sysinfo::Networks;

use super::{
  wifi_hotspot::{default_gateway_wifi, WifiHotstop},
  InterfaceType, NetworkGateway, NetworkInterface, NetworkTraffic,
  NetworkTrafficMeasure,
};
use crate::{
  common::{to_iec_bytes, to_si_bytes, SyncInterval},
  providers::{
    CommonProviderState, Provider, ProviderInputMsg, RuntimeType,
  },
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkProviderConfig {
  pub refresh_interval: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkOutput {
  pub default_interface: Option<NetworkInterface>,
  pub default_gateway: Option<NetworkGateway>,
  pub interfaces: Vec<NetworkInterface>,
  pub traffic: NetworkTraffic,
}

pub struct NetworkProvider {
  config: NetworkProviderConfig,
  common: CommonProviderState,
  netinfo: Networks,
}

impl NetworkProvider {
  pub fn new(
    config: NetworkProviderConfig,
    common: CommonProviderState,
  ) -> NetworkProvider {
    NetworkProvider {
      config,
      common,
      netinfo: Networks::new_with_refreshed_list(),
    }
  }

  fn run_interval(&mut self) -> anyhow::Result<NetworkOutput> {
    self.netinfo.refresh();

    let interfaces = netdev::get_interfaces();
    let default_interface = netdev::get_default_interface().ok();

    let (received, total_received) = Self::bytes_received(&self.netinfo);
    let received_per_sec = received / self.config.refresh_interval * 1000;

    let (transmitted, total_transmitted) =
      Self::bytes_transmitted(&self.netinfo);
    let transmitted_per_sec =
      transmitted / self.config.refresh_interval * 1000;

    Ok(NetworkOutput {
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
        total_received: Self::to_network_traffic_measure(total_received)?,
        transmitted: Self::to_network_traffic_measure(
          transmitted_per_sec,
        )?,
        total_transmitted: Self::to_network_traffic_measure(
          total_transmitted,
        )?,
      },
    })
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

  /// Gets the network (down) usage by every network interface.
  ///
  /// Returns a tuple of the bytes received since last refresh and total
  /// bytes received.
  fn bytes_received(networks: &sysinfo::Networks) -> (u64, u64) {
    let mut bytes_received: Vec<u64> = Vec::new();
    let mut total_bytes_received: Vec<u64> = Vec::new();

    for (_, network) in networks {
      bytes_received.push(network.received());
      total_bytes_received.push(network.total_received());
    }

    (
      bytes_received.iter().sum(),
      total_bytes_received.iter().sum(),
    )
  }

  /// Gets the network (up) usage by every network interface.
  ///
  /// Returns a tuple of the bytes transmitted since last refresh and total
  /// bytes transmitted.
  fn bytes_transmitted(networks: &sysinfo::Networks) -> (u64, u64) {
    let mut bytes_transmitted: Vec<u64> = Vec::new();
    let mut total_bytes_transmitted: Vec<u64> = Vec::new();

    for (_, network) in networks {
      bytes_transmitted.push(network.transmitted());
      total_bytes_transmitted.push(network.total_transmitted());
    }

    (
      bytes_transmitted.iter().sum(),
      total_bytes_transmitted.iter().sum(),
    )
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

impl Provider for NetworkProvider {
  fn runtime_type(&self) -> RuntimeType {
    RuntimeType::Sync
  }

  fn start_sync(&mut self) {
    let mut interval = SyncInterval::new(self.config.refresh_interval);

    loop {
      crossbeam::select! {
        recv(interval.tick()) -> _ => {
          let output = self.run_interval();
          self.common.emitter.emit_output(output);
        }
        recv(self.common.input.sync_rx) -> input => {
          if let Ok(ProviderInputMsg::Stop) = input {
            break;
          }
        }
      }
    }
  }
}
