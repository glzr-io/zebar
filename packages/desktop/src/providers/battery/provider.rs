use std::sync::Arc;

use async_trait::async_trait;
use starship_battery::Manager;
use sysinfo::System;
use tokio::{
  sync::{mpsc::Sender, Mutex},
  task::AbortHandle,
};

use crate::providers::{
  provider::RefreshableProvider, variables::ProviderVariables,
};

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
}

#[async_trait]
impl RefreshableProvider<String> for BatteryProvider {
  fn refresh_interval_ms(&self) -> u64 {
    self.config.refresh_interval_ms
  }

  fn state(&self) -> Mutex<String> {
    self.sysinfo.clone()
  }

  fn abort_handle(&self) -> Option<AbortHandle> {
    self.abort_handle
  }

  fn set_abort_handle(&self, abort_handle: AbortHandle) {
    self.abort_handle = Some(abort_handle)
  }

  async fn refresh_and_emit(
    emit_output_tx: &Sender<ProviderVariables>,
    sysinfo: &Mutex<String>,
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
