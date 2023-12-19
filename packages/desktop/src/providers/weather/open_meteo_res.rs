use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct OpenMeteoRes {
  pub latitude: i32,
  pub longitude: i32,
  pub generationtime_ms: i32,
  pub utc_offset_seconds: i32,
  pub timezone: String,
  pub timezone_abbreviation: String,
  pub elevation: i32,
  pub current_weather: OpenMeteoWeather,
  pub daily_units: OpenMeteoUnits,
  pub daily: OpenMeteoDaily,
}

#[derive(Deserialize, Debug)]
pub struct OpenMeteoWeather {
  pub temperature: f32,
  pub windspeed: f32,
  pub winddirection: i32,
  pub weathercode: u32,
  pub is_day: i32,
  pub time: String,
}

#[derive(Deserialize, Debug)]
pub struct OpenMeteoUnits {
  pub time: String,
  pub sunset: String,
  pub sunrise: String,
}

#[derive(Deserialize, Debug)]
pub struct OpenMeteoDaily {
  pub time: Vec<String>,
  pub sunset: Vec<String>,
  pub sunrise: Vec<String>,
}
