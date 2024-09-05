use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MemoryProviderConfig {
  pub refresh_interval: u64,
}
