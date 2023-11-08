use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use starship_battery::{
  units::{power::watt, ratio::percent, time::second},
  Battery, Manager,
};
use sysinfo::System;
use tokio::{
  sync::{mpsc::Sender, Mutex},
  task::{self, AbortHandle},
  time,
};

use crate::providers::{provider::Provider, variables::ProviderVariables};

use super::{BatteryProviderConfig, BatteryVariables};

pub struct BatteryProvider {
  pub config: BatteryProviderConfig,
  abort_handle: Option<AbortHandle>,
  sysinfo: Arc<Mutex<System>>,
}

impl BatteryProvider {
  pub fn new(
    config: BatteryProviderConfig,
    sysinfo: Arc<Mutex<System>>,
  ) -> BatteryProvider {
    BatteryProvider {
      config,
      abort_handle: None,
      sysinfo,
    }
  }

  async fn refresh_and_emit(
    sysinfo: &Mutex<System>,
    emit_output_tx: &Sender<ProviderVariables>,
  ) {
    let manager = Manager::new().unwrap();

    for (idx, maybe_battery) in manager.batteries().unwrap().enumerate() {
      let battery = maybe_battery.unwrap();
      println!("Battery #{}:", idx);
      println!("Vendor: {:?}", battery.vendor());
      println!("Model: {:?}", battery.model());
      println!("State: {:?}", battery.state());
      println!("Time to full charge: {:?}", battery.time_to_full());
      println!("empty: {:?}", battery.time_to_empty());
      println!("cycle: {:?}", battery.cycle_count());
      println!("charge: {:?}", battery.state_of_charge());
      println!("");
    }

    _ = emit_output_tx
      .send(ProviderVariables::Battery(BatteryVariables {}))
      .await;
  }
}

#[async_trait]
impl Provider for BatteryProvider {
  async fn on_start(&mut self, emit_output_tx: Sender<ProviderVariables>) {
    let refresh_interval_ms = self.config.refresh_interval_ms;
    let sysinfo = self.sysinfo.clone();

    let forever = task::spawn(async move {
      let mut interval =
        time::interval(Duration::from_millis(refresh_interval_ms));

      Self::refresh_and_emit(&sysinfo, &emit_output_tx).await;

      loop {
        interval.tick().await;
        Self::refresh_and_emit(&sysinfo, &emit_output_tx).await;
      }
    });

    self.abort_handle = Some(forever.abort_handle());
    _ = forever.await;
  }

  async fn on_refresh(&mut self, emit_output_tx: Sender<ProviderVariables>) {
    Self::refresh_and_emit(&self.sysinfo, &emit_output_tx).await;
  }

  async fn on_stop(&mut self) {
    if let Some(handle) = &self.abort_handle {
      handle.abort();
    }
  }
}
