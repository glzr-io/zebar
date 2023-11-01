use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ProviderConfig {
  Cpu(CpuProviderConfig),
  Network(NetworkProviderConfig),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename = "cpu")]
pub struct CpuProviderConfig {
  pub refresh_interval_ms: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename = "network")]
pub struct NetworkProviderConfig {
  pub refresh_interval_ms: u64,
}
