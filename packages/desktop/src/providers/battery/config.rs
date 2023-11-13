use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename = "battery")]
pub struct BatteryProviderConfig {
  pub refresh_interval_ms: u64,
}
