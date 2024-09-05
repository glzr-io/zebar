use serde::Deserialize;

// use crate::impl_interval_config;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CpuProviderConfig {
  pub refresh_interval: u64,
}

// impl_interval_config!(CpuProviderConfig);
