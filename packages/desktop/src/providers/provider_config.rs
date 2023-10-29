use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ProviderConfig {
  Cpu(CpuProviderConfig),
  Network(NetworkProviderConfig),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename = "cpu")]
pub struct CpuProviderConfig {
  refresh_interval: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename = "network")]
pub struct NetworkProviderConfig {
  refresh_interval: i32,
}
