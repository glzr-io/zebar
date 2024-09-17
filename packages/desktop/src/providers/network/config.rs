use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkProviderConfig {
  pub refresh_interval: u64,
}
