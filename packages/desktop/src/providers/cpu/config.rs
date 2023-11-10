use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename = "cpu")]
pub struct CpuProviderConfig {
  pub refresh_interval_ms: u64,
}
