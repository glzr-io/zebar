use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use tokio::{sync::Mutex, task::AbortHandle};

use crate::providers::{
  interval_provider::IntervalProvider, variables::ProviderVariables,
};

use super::WeatherProviderConfig;

pub struct WeatherProvider {
  pub config: WeatherProviderConfig,
  abort_handle: Option<AbortHandle>,
  weather_manager: Client,
}

impl WeatherProvider {
  pub fn new(config: WeatherProviderConfig) -> WeatherProvider {
    WeatherProvider {
      config,
      abort_handle: None,
      // weather_manager: Arc::new(Mutex::new(reqwest::Client::new())),
      weather_manager: reqwest::Client::new(),
    }
  }
}

#[async_trait]
impl IntervalProvider for WeatherProvider {
  type State = Client;

  fn refresh_interval_ms(&self) -> u64 {
    self.config.refresh_interval_ms
  }

  // fn state(&self) -> Arc<Mutex<Client>> {
  fn state(&self) -> Client {
    self.weather_manager.clone()
  }

  fn abort_handle(&self) -> &Option<AbortHandle> {
    &self.abort_handle
  }

  fn set_abort_handle(&mut self, abort_handle: AbortHandle) {
    self.abort_handle = Some(abort_handle)
  }

  async fn get_refreshed_variables(
    weather_manager: &Mutex<Client>,
  ) -> Result<ProviderVariables> {
    Ok(ProviderVariables::Weather(Self::get_variables(
      &*weather_manager.lock().await,
    )?))
  }
}
