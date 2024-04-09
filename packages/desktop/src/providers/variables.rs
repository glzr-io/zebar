use serde::Serialize;

#[cfg(all(windows, target_arch = "x86_64"))]
use super::komorebi::KomorebiVariables;
use super::{
  battery::BatteryVariables, cpu::CpuVariables, host::HostVariables,
  ip::IpVariables, memory::MemoryVariables, network::NetworkVariables,
  networkactivity::NetworkActivityVariables, weather::WeatherVariables,
};

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ProviderVariables {
  Battery(BatteryVariables),
  Cpu(CpuVariables),
  Host(HostVariables),
  Ip(IpVariables),
  #[cfg(all(windows, target_arch = "x86_64"))]
  Komorebi(KomorebiVariables),
  Memory(MemoryVariables),
  Network(NetworkVariables),
  NetworkActivity(NetworkActivityVariables),
  Weather(WeatherVariables),
}
