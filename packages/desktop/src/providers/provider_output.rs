use serde::Serialize;

use super::{
  battery::BatteryOutput, cpu::CpuOutput, host::HostOutput, ip::IpOutput,
  media::MediaOutput, memory::MemoryOutput, network::NetworkOutput,
  weather::WeatherOutput,
};
#[cfg(windows)]
use super::{keyboard::KeyboardOutput, komorebi::KomorebiOutput};

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ProviderOutput {
  Battery(BatteryOutput),
  Cpu(CpuOutput),
  Host(HostOutput),
  Ip(IpOutput),
  #[cfg(windows)]
  Media(MediaOutput),
  #[cfg(windows)]
  Komorebi(KomorebiOutput),
  Memory(MemoryOutput),
  Network(NetworkOutput),
  Weather(WeatherOutput),
  #[cfg(windows)]
  Keyboard(KeyboardOutput),
}
