use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use sysinfo::Networks;
use netdev::interface::{
  get_default_interface, get_interfaces,
};
use tokio::{sync::Mutex, task::AbortHandle};

use crate::providers::{
  interval_provider::IntervalProvider,
  network::parse_netsh::get_primary_interface_ssid_and_strength,
  variables::ProviderVariables,
};

use super::{
  Gateway, NetworkInterface, NetworkTraffic, NetworkProviderConfig, NetworkVariables
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
  ) -> Result<ProviderVariables> {
    let mut netinfo = netinfo.lock().await;
    netinfo.refresh();

    let default_interface = get_default_interface().unwrap();

    let interfaces = get_interfaces();

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
        ipv4_addresses: default_interface
          .gateway
          .as_ref()
          .unwrap()
          .ipv4
          .clone(),
        ipv6_addresses: default_interface
          .gateway
          .as_ref()
          .unwrap()
          .ipv6
          .clone(),
        ssid: default_gateway_ssid_and_strength.ssid.unwrap(),
        signal_strength: default_gateway_ssid_and_strength
          .signal
          .unwrap(),
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
