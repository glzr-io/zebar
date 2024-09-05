use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BatteryProviderConfig {
  pub refresh_interval: u64,
}
