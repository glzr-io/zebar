use anyhow::Context;
use reqwest::Client;

use super::{ipinfo_res::IpinfoRes, IpOutput, IpProviderConfig};
use crate::{impl_interval_provider, providers::ProviderOutput};

pub struct IpProvider {
  config: IpProviderConfig,
  http_client: Client,
}

impl IpProvider {
  pub fn new(config: IpProviderConfig) -> IpProvider {
    IpProvider {
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

impl_interval_provider!(IpProvider, false);
