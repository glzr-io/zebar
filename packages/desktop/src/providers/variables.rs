use serde::Serialize;

#[cfg(windows)]
use super::komorebi::KomorebiVariables;
use super::{
  battery::BatteryVariables, cpu::CpuVariables, host::HostVariables,
  ip::IpVariables, language::LanguageVariables, memory::MemoryVariables,
  network::NetworkVariables, weather::WeatherVariables,
};

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ProviderVariables {
  Battery(BatteryVariables),
  Cpu(CpuVariables),
  Host(HostVariables),
  Ip(IpVariables),
  #[cfg(windows)]
  Komorebi(KomorebiVariables),
  Memory(MemoryVariables),
  Network(NetworkVariables),
  Weather(WeatherVariables),
  #[cfg(windows)]
  Language(LanguageVariables),
}
