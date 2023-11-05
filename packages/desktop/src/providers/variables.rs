use serde::{Deserialize, Serialize};

use super::{cpu::CpuVariables, network::NetworkVariables};

#[derive(Serialize, Deserialize, Debug)]
pub enum ProviderVariables {
  Cpu(CpuVariables),
  Network(NetworkVariables),
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct HostProviderData {
//   pub host_name: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct BatteryProviderData {
//   pub charge_percent: f64,
//   pub health_percent: f64,
//   pub state: State,
//   pub secs_until_full: Option<i64>,
//   pub secs_until_empty: Option<i64>,
//   pub power_consumption_watts: f64,
// }
