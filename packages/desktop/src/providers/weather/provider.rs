use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use reqwest::Client;
use tokio::{sync::mpsc, time};

use super::{
  open_meteo_res::OpenMeteoRes, WeatherOutput, WeatherProviderConfig,
  WeatherStatus,
};
use crate::providers::{
  provider::Provider, provider_ref::ProviderResult,
  variables::ProviderOutput,
};

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

  async fn interval_output(
    config: &WeatherProviderConfig,
    http_client: &Client,
  ) -> anyhow::Result<ProviderOutput> {
    let res = http_client
      .get("https://api.open-meteo.com/v1/forecast")
      .query(&[
        ("temperature_unit", "celsius"),
        ("latitude", &config.latitude.to_string()),
        ("longitude", &config.longitude.to_string()),
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

#[async_trait]
impl Provider for WeatherProvider {
  async fn on_start(&self, emit_result_tx: mpsc::Sender<ProviderResult>) {
    let mut interval =
      time::interval(Duration::from_millis(self.config.refresh_interval));

    loop {
      interval.tick().await;

      emit_result_tx
        .send(
          Self::interval_output(&self.config, &self.http_client)
            .await
            .into(),
        )
        .await;
    }
  }
}
