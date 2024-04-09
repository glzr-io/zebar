use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use sysinfo::Networks;
use tokio::{sync::Mutex, task::AbortHandle};

use crate::providers::{
  interval_provider::IntervalProvider, variables::ProviderVariables,
};

use super::{NetworkActivityProviderConfig, NetworkActivityVariables};

pub struct NetworkActivityProvider {
  pub config: Arc<NetworkActivityProviderConfig>,
  abort_handle: Option<AbortHandle>,
  netinfo: Arc<Mutex<Networks>>,
}

impl NetworkActivityProvider {
  pub fn new(
    config: NetworkActivityProviderConfig,
    netinfo: Arc<Mutex<Networks>>,
  ) -> NetworkActivityProvider {
    NetworkActivityProvider {
      config: Arc::new(config),
      abort_handle: None,
      netinfo,
    }
  }
}

#[async_trait]
impl IntervalProvider for NetworkActivityProvider {
  type Config = NetworkActivityProviderConfig;
  type State = Mutex<Networks>;

  fn config(&self) -> Arc<NetworkActivityProviderConfig> {
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
    _: &NetworkActivityProviderConfig,
    netinfo: &Mutex<Networks>,
  ) -> Result<ProviderVariables> {
    let mut netinfo = netinfo.lock().await;
    netinfo.refresh();

    Ok(ProviderVariables::NetworkActivity(NetworkActivityVariables {
      received: get_network_down(&netinfo),
      transmitted: get_network_up(&netinfo),
    }))
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

// NOTE: convert bytes to a readable format.
// This could be a new provider in Rust or javascript in order to convert other byte fields like memory

//#[derive(Debug, FromPrimitive)]
//enum DataUnit {
//    B = 0,
//    K = 1,
//    M = 2,
//    G = 3,
//    T = 4,
//    P = 5,
//    E = 6,
//    Z = 7,
//    Y = 8,
//}

//impl fmt::Display for DataUnit {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "{:?}", self)
//    }
//}

//fn to_pretty_bytes(input_in_bytes: u64, timespan_in_ms: u64) -> String {
//    if input_in_bytes < 1024 {
//        return format!("0 KB");
//    }

//    let seconds = timespan_in_ms as f32 / f32::powf(10.0, 3.0);
//    let magnitude = input_in_bytes.ilog(1024);
//    let base: Option<DataUnit> = num::FromPrimitive::from_u32(magnitude);
//    let result = (input_in_bytes as f32 / seconds) / ((1 as u64) << (magnitude * 10)) as f32;

//    match base {
//        Some(DataUnit::B) => format!("{result:.2} B"),
//        Some(unit) => format!("{result:.2} {unit}B"),
//        None => format!("Unknown data unit"),
//    }
//}

//fn to_pretty_bits(input_in_bytes: u64, timespan_in_ms: u64) -> String {
//    if input_in_bytes < 1000 {
//        return format!("0 Kb");
//    }
   
//    let input = input_in_bytes * 8;

//    let seconds = timespan_in_ms as f32 / f32::powf(10.0, 3.0);
//    let magnitude = input.ilog(1000);
//    let base: Option<DataUnit> = num::FromPrimitive::from_u32(magnitude);
//    let result = (input as f32 / seconds) / f32::powf(1000.0, magnitude as f32);

//    match base {
//        Some(DataUnit::B) => format!("{result:.2} b"),
//        Some(unit) => format!("{result:.2} {unit}b"),
//        None => format!("Unknown data unit"),
//    }
//}
