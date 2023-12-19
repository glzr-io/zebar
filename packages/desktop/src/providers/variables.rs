use serde::Serialize;

use super::{
  battery::BatteryVariables, cpu::CpuVariables, host::HostVariables,
  ip::IpVariables, memory::MemoryVariables, network::NetworkVariables,
  weather::WeatherVariables,
};

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ProviderVariables {
  Battery(BatteryVariables),
  Cpu(CpuVariables),
  Host(HostVariables),
  Ip(IpVariables),
  Memory(MemoryVariables),
  Network(NetworkVariables),
  Weather(WeatherVariables),
}
