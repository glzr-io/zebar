use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::open_meteo_res::OpenMeteoRes;
use crate::{impl_interval_provider, providers::ProviderOutput};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WeatherProviderConfig {
  pub refresh_interval: u64,
  pub latitude: f32,
  pub longitude: f32,
}

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

pub struct WeatherProvider {
  config: WeatherProviderConfig,
  http_client: Client,
}

impl WeatherProvider {
  pub fn new(config: WeatherProviderConfig) -> WeatherProvider {
    WeatherProvider {
      config,
      http_client: Client::new(),
    }
  }

  fn refresh_interval_ms(&self) -> u64 {
    self.config.refresh_interval
  }

  async fn run_interval(&self) -> anyhow::Result<ProviderOutput> {
    let res = self
      .http_client
      .get("https://api.open-meteo.com/v1/forecast")
      .query(&[
        ("temperature_unit", "celsius"),
        ("latitude", &self.config.latitude.to_string()),
        ("longitude", &self.config.longitude.to_string()),
        ("current_weather", "true"),
        ("daily", "sunset,sunrise"),
        ("timezone", "auto"),
      ])
      .send()
      .await?
      .json::<OpenMeteoRes>()
      .await?;

    let current_weather = res.current_weather;
    let is_daytime = current_weather.is_day == 1;

    Ok(ProviderOutput::Weather(WeatherOutput {
      is_daytime,
      status: Self::get_weather_status(
        current_weather.weather_code,
        is_daytime,
      ),
      celsius_temp: current_weather.temperature,
      fahrenheit_temp: Self::celsius_to_fahrenheit(
        current_weather.temperature,
      ),
      wind_speed: current_weather.wind_speed,
    }))
  }

  fn celsius_to_fahrenheit(celsius_temp: f32) -> f32 {
    return (celsius_temp * 9.) / 5. + 32.;
  }

  /// Relevant documentation: https://open-meteo.com/en/docs#weathervariables
  fn get_weather_status(code: u32, is_daytime: bool) -> WeatherStatus {
    match code {
      0 => match is_daytime {
        true => WeatherStatus::ClearDay,
        false => WeatherStatus::ClearNight,
      },
      1..=50 => match is_daytime {
        true => WeatherStatus::CloudyDay,
        false => WeatherStatus::CloudyNight,
      },
      51..=62 => match is_daytime {
        true => WeatherStatus::LightRainDay,
        false => WeatherStatus::LightRainNight,
      },
      63..=70 => match is_daytime {
        true => WeatherStatus::HeavyRainDay,
        false => WeatherStatus::HeavyRainNight,
      },
      71..=79 => match is_daytime {
        true => WeatherStatus::SnowDay,
        false => WeatherStatus::SnowNight,
      },
      80..=84 => match is_daytime {
        true => WeatherStatus::HeavyRainDay,
        false => WeatherStatus::HeavyRainNight,
      },
      85..=94 => match is_daytime {
        true => WeatherStatus::SnowDay,
        false => WeatherStatus::SnowNight,
      },
      95..=u32::MAX => match is_daytime {
        true => WeatherStatus::ThunderDay,
        false => WeatherStatus::ThunderNight,
      },
    }
  }
}

impl_interval_provider!(WeatherProvider, true);
