use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ProviderConfig {
  #[serde(rename = "cpu")]
  Cpu { refresh_interval: i32 },
  #[serde(rename = "network")]
  Network { refresh_interval: i32 },
}

pub fn provider_scheduler() -> Result<()> {
  Ok(())
}
