use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename = "host")]
pub struct HostProviderConfig {
  pub refresh_interval_ms: u64,
}
