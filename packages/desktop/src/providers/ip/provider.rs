use std::sync::Arc;

use anyhow::Context;
use async_trait::async_trait;
use reqwest::Client;
use tokio::task::AbortHandle;

use super::{ipinfo_res::IpinfoRes, IpProviderConfig, IpOutput};
use crate::providers::{
  provider::IntervalProvider, variables::ProviderOutput,
};

pub struct IpProvider {
  pub config: Arc<IpProviderConfig>,
  abort_handle: Option<AbortHandle>,
  http_client: Arc<Client>,
}

impl IpProvider {
  pub fn new(config: IpProviderConfig) -> IpProvider {
    IpProvider {
      config: Arc::new(config),
      abort_handle: None,
      http_client: Arc::new(Client::new()),
    }
  }
}

#[async_trait]
impl IntervalProvider for IpProvider {
  type Config = IpProviderConfig;
  type State = Client;

  fn config(&self) -> Arc<IpProviderConfig> {
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
    _: &IpProviderConfig,
    http_client: &Client,
  ) -> anyhow::Result<ProviderOutput> {
    let res = http_client
      .get("https://ipinfo.io/json")
      .send()
      .await?
      .json::<IpinfoRes>()
      .await?;

    let mut loc_parts = res.loc.split(',');

    Ok(ProviderOutput::Ip(IpOutput {
      address: res.ip,
      approx_city: res.city,
      approx_country: res.country,
      approx_latitude: loc_parts
        .next()
        .and_then(|lat| lat.parse::<f32>().ok())
        .context("Failed to parse latitude from IPinfo.")?,
      approx_longitude: loc_parts
        .next()
        .and_then(|long| long.parse::<f32>().ok())
        .context("Failed to parse longitude from IPinfo.")?,
    }))
  }
}
