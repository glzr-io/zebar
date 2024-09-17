use serde::Deserialize;

use super::{
  battery::BatteryProviderConfig, cpu::CpuProviderConfig,
  host::HostProviderConfig, ip::IpProviderConfig,
  memory::MemoryProviderConfig, network::NetworkProviderConfig,
  weather::WeatherProviderConfig,
};
#[cfg(windows)]
use super::{
  keyboard::KeyboardProviderConfig, komorebi::KomorebiProviderConfig,
};

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ProviderConfig {
  Battery(BatteryProviderConfig),
  Cpu(CpuProviderConfig),
  Host(HostProviderConfig),
  Ip(IpProviderConfig),
  #[cfg(windows)]
  Komorebi(KomorebiProviderConfig),
  Memory(MemoryProviderConfig),
  Network(NetworkProviderConfig),
  Weather(WeatherProviderConfig),
  #[cfg(windows)]
  Keyboard(KeyboardProviderConfig),
}
