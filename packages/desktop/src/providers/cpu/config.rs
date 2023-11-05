use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename = "cpu")]
pub struct CpuProviderConfig {
  pub refresh_interval_ms: u64,
}
