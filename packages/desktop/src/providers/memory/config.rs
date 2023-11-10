use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename = "memory")]
pub struct MemoryProviderConfig {
  pub refresh_interval_ms: u64,
}
