use serde::Serialize;

#[cfg(windows)]
use super::komorebi::KomorebiOutput;
use super::{
  battery::BatteryOutput, cpu::CpuOutput, host::HostOutput, ip::IpOutput,
  language::LanguageVariables, memory::MemoryOutput,
  network::NetworkOutput, weather::WeatherOutput,
};

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ProviderOutput {
  Battery(BatteryOutput),
  Cpu(CpuOutput),
  Host(HostOutput),
  Ip(IpOutput),
  #[cfg(windows)]
  Komorebi(KomorebiOutput),
  Memory(MemoryOutput),
  Network(NetworkOutput),
  Weather(WeatherOutput),
  #[cfg(windows)]
  Language(LanguageVariables),
}
