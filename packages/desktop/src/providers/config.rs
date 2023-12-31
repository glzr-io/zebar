use serde::Deserialize;

use super::{
  battery::BatteryProviderConfig, cpu::CpuProviderConfig,
  host::HostProviderConfig, ip::IpProviderConfig,
  memory::MemoryProviderConfig, network::NetworkProviderConfig,
  weather::WeatherProviderConfig,
};

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ProviderConfig {
  Battery(BatteryProviderConfig),
  Cpu(CpuProviderConfig),
  Host(HostProviderConfig),
  Ip(IpProviderConfig),
  Memory(MemoryProviderConfig),
  Network(NetworkProviderConfig),
  Weather(WeatherProviderConfig),
}
