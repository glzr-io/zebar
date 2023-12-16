use serde::Deserialize;

use crate::impl_interval_config;

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename = "memory")]
pub struct MemoryProviderConfig {
  pub refresh_interval_ms: u64,
}

impl_interval_config!(MemoryProviderConfig);
