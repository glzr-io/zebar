use std::sync::Arc;

use anyhow::{Context, Result};
use async_trait::async_trait;
use starship_battery::{
  units::{
    electric_potential::volt, power::watt, ratio::percent, time::second,
  },
  Manager,
};
use tokio::{sync::Mutex, task::AbortHandle};

use crate::providers::{
  interval_provider::IntervalProvider, variables::ProviderVariables,
};

use super::{BatteryProviderConfig, BatteryVariables};

pub struct BatteryProvider {
  pub config: BatteryProviderConfig,
  abort_handle: Option<AbortHandle>,
  battery_manager: Arc<Mutex<Manager>>,
}

impl BatteryProvider {
  pub fn new(config: BatteryProviderConfig) -> Result<BatteryProvider> {
    let manager = Manager::new()?;

    Ok(BatteryProvider {
      config,
      abort_handle: None,
      battery_manager: Arc::new(Mutex::new(manager)),
    })
  }

  /// Battery manager from `starship_battery` is not thread-safe, so it
  /// requires its own non-async function.
  fn get_variables(manager: &Manager) -> Result<BatteryVariables> {
    let first_battery = manager
      .batteries()
      .and_then(|mut batteries| batteries.nth(0).transpose())
      .unwrap_or(None)
      .context("No battery found.");

    first_battery.map(|battery| BatteryVariables {
      charge_percent: battery.state_of_charge().get::<percent>(),
      health_percent: battery.state_of_health().get::<percent>(),
      state: battery.state().to_string(),
      time_till_full: battery.time_to_full().map(|time| time.get::<second>()),
      time_till_empty: battery.time_to_empty().map(|time| time.get::<second>()),
      power_consumption: battery.energy_rate().get::<watt>(),
      voltage: battery.voltage().get::<volt>(),
      cycle_count: battery.cycle_count(),
    })
  }
}

#[async_trait]
impl IntervalProvider for BatteryProvider {
  type State = Manager;

  fn refresh_interval_ms(&self) -> u64 {
    self.config.refresh_interval_ms
  }

  fn state(&self) -> Arc<Mutex<Manager>> {
    self.battery_manager.clone()
  }

  fn abort_handle(&self) -> &Option<AbortHandle> {
    &self.abort_handle
  }

  fn set_abort_handle(&mut self, abort_handle: AbortHandle) {
    self.abort_handle = Some(abort_handle)
  }

  async fn get_refreshed_variables(
    battery_manager: &Mutex<Manager>,
  ) -> Result<ProviderVariables> {
    Ok(ProviderVariables::Battery(Self::get_variables(
      &*battery_manager.lock().await,
    )?))
  }
}
