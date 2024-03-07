use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use netdev::interface::{
  get_default_interface, get_interfaces, Interface,
};
use tokio::{sync::Mutex, task::AbortHandle};

use crate::providers::{
  interval_provider::IntervalProvider,
  newnetwork::parse_netsh::get_primary_interface_ssid_and_strength,
  variables::ProviderVariables,
};

use super::{
  Gateway, NetworkInterface, NewNetworkProviderConfig, NewNetworkVariables,
};

pub struct NewNetworkProvider {
  pub config: Arc<NewNetworkProviderConfig>,
  abort_handle: Option<AbortHandle>,
  netdev_data: Arc<Mutex<NetdevData>>,
}

pub struct NetdevData {
  pub default_interface: Arc<Mutex<Interface>>,
  pub interfaces: Arc<Mutex<Vec<Interface>>>,
}

impl NewNetworkProvider {
  pub fn new(config: NewNetworkProviderConfig) -> NewNetworkProvider {
    NewNetworkProvider {
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
impl IntervalProvider for NewNetworkProvider {
  type Config = NewNetworkProviderConfig;
  type State = Mutex<NetdevData>;

  fn config(&self) -> Arc<NewNetworkProviderConfig> {
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
    _: &NewNetworkProviderConfig,
    netdev_data: &Mutex<NetdevData>,
  ) -> Result<ProviderVariables> {
    let netdev = netdev_data.lock().await;

    let default_interface = netdev.default_interface.lock().await.clone();

    let interfaces = netdev.interfaces.lock().await;

    #[cfg(target_os = "windows")]
    let default_gateway_ssid_and_strength =
      get_primary_interface_ssid_and_strength()?;

    #[cfg(not(target_os = "windows"))]
    let default_gateway_ssid_and_strength = {
      let ssid = None;
      let signal = None;
      Gateway { ssid, signal };
    };

    let variables = NewNetworkVariables {
      default_interface: NetworkInterface {
        name: default_interface.name.clone(),
        friendly_name: default_interface.friendly_name.clone(),
        description: default_interface.description.clone(),
        if_type: default_interface.if_type,
        ipv4: default_interface.ipv4.clone(),
        ipv6: default_interface.ipv6.clone(),
        mac_addr: default_interface.mac_addr.unwrap(),
        transmit_speed: default_interface.transmit_speed,
        receive_speed: default_interface.receive_speed,
        dns_servers: default_interface.dns_servers.clone(),
        default: default_interface.default,
      },
      default_gateway: Gateway {
        mac_addr: default_interface
          .gateway
          .as_ref()
          .unwrap()
          .mac_addr
          .clone(),
        ipv4: default_interface.gateway.as_ref().unwrap().ipv4.clone(),
        ipv6: default_interface.gateway.as_ref().unwrap().ipv6.clone(),
        ssid: default_gateway_ssid_and_strength.ssid.unwrap(),
        signal_strength: default_gateway_ssid_and_strength.signal.unwrap(),
      },
      interfaces: interfaces
        .iter()
        .map(|iface| NetworkInterface {
          name: iface.name.clone(),
          friendly_name: iface.friendly_name.clone(),
          description: iface.description.clone(),
          if_type: iface.if_type.clone(),
          ipv4: iface.ipv4.clone(),
          ipv6: iface.ipv6.clone(),
          mac_addr: iface.mac_addr.unwrap().clone(),
          transmit_speed: iface.transmit_speed,
          receive_speed: iface.receive_speed,
          dns_servers: iface.dns_servers.clone(),
          default: iface.default,
        })
        .collect(),
    };

    println!("variables: {:?}", variables);

    Ok(ProviderVariables::NewNetwork(variables))
  }
}
