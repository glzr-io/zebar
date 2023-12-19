use serde::Deserialize;

use crate::impl_interval_config;

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename = "network")]
pub struct NetworkProviderConfig {
  pub refresh_interval_ms: u64,
}

impl_interval_config!(NetworkProviderConfig);
