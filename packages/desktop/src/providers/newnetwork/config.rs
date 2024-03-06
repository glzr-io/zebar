use serde::Deserialize;

use crate::impl_interval_config;

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename = "newnetwork")]
pub struct NewNetworkProviderConfig {
  pub refresh_interval: u64,
}

impl_interval_config!(NewNetworkProviderConfig);
