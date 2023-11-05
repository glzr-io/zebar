use serde::{Deserialize, Serialize};

use super::{cpu::CpuProviderConfig, network::NetworkProviderConfig};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ProviderConfig {
  Cpu(CpuProviderConfig),
  Network(NetworkProviderConfig),
}
