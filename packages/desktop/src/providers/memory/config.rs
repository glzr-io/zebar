use serde::Deserialize;

use crate::impl_interval_config;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MemoryProviderConfig {
  pub refresh_interval: u64,
}

impl_interval_config!(MemoryProviderConfig);
