use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename = "weather")]
pub struct WeatherProviderConfig {
  pub refresh_interval_ms: u64,
  pub latitude: f32,
  pub longitude: f32,
}
