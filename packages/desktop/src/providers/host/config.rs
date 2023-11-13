use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename = "host")]
pub struct HostProviderConfig {
  pub refresh_interval_ms: u64,
}
