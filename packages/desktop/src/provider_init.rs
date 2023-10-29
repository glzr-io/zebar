use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ProviderConfig {
  Cpu { refresh_interval: i32 },
  Network { refresh_interval: i32 },
}

pub fn provider_init() {}
