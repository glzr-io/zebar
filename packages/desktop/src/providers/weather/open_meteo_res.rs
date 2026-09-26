use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct OpenMeteoRes {
  pub current_weather: OpenMeteoWeather,
}

#[derive(Deserialize, Debug)]
pub struct OpenMeteoWeather {
  pub temperature: f32,
  #[serde(rename = "windspeed")]
  pub wind_speed: f32,
  #[serde(rename = "winddirection")]
  // TODO: Surface wind_direction in widget data (#279)
  #[allow(dead_code)]
  pub wind_direction: f32,
  #[serde(rename = "weathercode")]
  pub weather_code: u32,
  pub is_day: u32,
}
