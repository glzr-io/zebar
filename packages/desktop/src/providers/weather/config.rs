use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WeatherProviderConfig {
  pub refresh_interval: u64,
  pub latitude: f32,
  pub longitude: f32,
}
