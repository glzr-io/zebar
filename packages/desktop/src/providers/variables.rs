use serde::{Deserialize, Serialize};

use super::{
  battery::BatteryVariables, cpu::CpuVariables, host::HostVariables,
  memory::MemoryVariables, network::NetworkVariables,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ProviderVariables {
  Battery(BatteryVariables),
  Cpu(CpuVariables),
  Host(HostVariables),
  Memory(MemoryVariables),
  Network(NetworkVariables),
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct BatteryProviderData {
//   pub charge_percent: f64,
//   pub health_percent: f64,
//   pub state: State,
//   pub secs_until_full: Option<i64>,
//   pub secs_until_empty: Option<i64>,
//   pub power_consumption_watts: f64,
// }
