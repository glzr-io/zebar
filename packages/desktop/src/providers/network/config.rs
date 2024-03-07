use serde::Deserialize;

use crate::impl_interval_config;

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename = "newnetwork")]
pub struct NetworkProviderConfig {
  pub refresh_interval: u64,
}

impl_interval_config!(NetworkProviderConfig);
