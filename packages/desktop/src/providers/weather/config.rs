use serde::Deserialize;

use crate::impl_interval_config;

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename = "weather")]
pub struct WeatherProviderConfig {
  pub refresh_interval_ms: u64,
  pub latitude: f32,
  pub longitude: f32,
}

impl_interval_config!(WeatherProviderConfig);
