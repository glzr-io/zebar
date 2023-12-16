use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use tokio::task::AbortHandle;

use crate::providers::{
  interval_provider::IntervalProvider, variables::ProviderVariables,
};

use super::{WeatherProviderConfig, WeatherStatus, WeatherVariables};

pub struct WeatherProvider {
  pub config: Arc<WeatherProviderConfig>,
  abort_handle: Option<AbortHandle>,
  http_client: Arc<Client>,
}

impl WeatherProvider {
  pub fn new(config: WeatherProviderConfig) -> WeatherProvider {
    WeatherProvider {
      config: Arc::new(config),
      abort_handle: None,
      http_client: Arc::new(Client::new()),
    }
  }
}

#[async_trait]
impl IntervalProvider for WeatherProvider {
  type Config = WeatherProviderConfig;
  type State = Client;

  fn config(&self) -> Arc<WeatherProviderConfig> {
    self.config.clone()
  }

  fn state(&self) -> Arc<Client> {
    self.http_client.clone()
  }

  fn abort_handle(&self) -> &Option<AbortHandle> {
    &self.abort_handle
  }

  fn set_abort_handle(&mut self, abort_handle: AbortHandle) {
    self.abort_handle = Some(abort_handle)
  }

  async fn get_refreshed_variables(
    config: &WeatherProviderConfig,
    http_client: &Client,
  ) -> Result<ProviderVariables> {
    let body = http_client
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
      .text()
      .await?;

    Ok(ProviderVariables::Weather(WeatherVariables {
      is_daytime: true,
      status: WeatherStatus::ClearDay,
      celsius_temp: 32.,
      fahrenheit_temp: 32.,
      wind_speed: 32.,
    }))
  }
}
