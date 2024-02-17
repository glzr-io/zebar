use serde::Deserialize;

use crate::impl_interval_config;

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename = "host")]
pub struct HostProviderConfig {
  pub refresh_interval: u64,
}

impl_interval_config!(HostProviderConfig);
