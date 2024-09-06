use std::{sync::Arc, time::Duration};

use anyhow::Context;
use async_trait::async_trait;
use reqwest::Client;
use tokio::{sync::mpsc, time};

use super::{ipinfo_res::IpinfoRes, IpOutput, IpProviderConfig};
use crate::providers::{
  provider::Provider, provider_ref::ProviderResult,
  variables::ProviderOutput,
};

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

  async fn interval_output(
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

#[async_trait]
impl Provider for IpProvider {
  async fn on_start(&self, emit_result_tx: mpsc::Sender<ProviderResult>) {
    let mut interval =
      time::interval(Duration::from_millis(self.config.refresh_interval));

    loop {
      interval.tick().await;

      emit_result_tx
        .send(Self::interval_output(&self.http_client).await.into())
        .await;
    }
  }
}
