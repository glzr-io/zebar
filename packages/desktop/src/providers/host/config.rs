use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HostProviderConfig {
  pub refresh_interval: u64,
}
