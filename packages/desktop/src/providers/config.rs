use serde::Deserialize;

use super::{
  battery::BatteryProviderConfig, cpu::CpuProviderConfig,
  host::HostProviderConfig, memory::MemoryProviderConfig,
  network::NetworkProviderConfig,
};

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ProviderConfig {
  Battery(BatteryProviderConfig),
  Cpu(CpuProviderConfig),
  Host(HostProviderConfig),
  Memory(MemoryProviderConfig),
  Network(NetworkProviderConfig),
}
