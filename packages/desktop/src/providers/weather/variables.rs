use serde::Serialize;

#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct WeatherVariables {
  pub is_daytime: bool,
  pub status: WeatherStatus,
  pub celsius_temp: f32,
  pub fahrenheit_temp: f32,
  pub wind_speed: f32,
}
