use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherOutput {
  pub is_daytime: bool,
  pub status: WeatherStatus,
  pub celsius_temp: f32,
  pub fahrenheit_temp: f32,
  pub wind_speed: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WeatherStatus {
  ClearDay,
  ClearNight,
  CloudyDay,
  CloudyNight,
  LightRainDay,
  LightRainNight,
  HeavyRainDay,
  HeavyRainNight,
  SnowDay,
  SnowNight,
  ThunderDay,
  ThunderNight,
}
