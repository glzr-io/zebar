use std::{sync::Arc, time::Duration};

use anyhow::Context;
use async_trait::async_trait;
use starship_battery::{
  units::{
    electric_potential::volt, power::watt, ratio::percent,
    time::millisecond,
  },
  Manager, State,
};
use tokio::{sync::mpsc, time};

use super::{BatteryProviderConfig, BatteryVariables};
use crate::providers::{
  provider::Provider,
  provider_manager::SharedProviderState,
  provider_ref::{ProviderOutput, VariablesResult},
  variables::ProviderVariables,
};

pub struct BatteryProvider {
  config: BatteryProviderConfig,
  battery_manager: Manager,
}

impl BatteryProvider {
  pub fn new(
    config: BatteryProviderConfig,
  ) -> anyhow::Result<BatteryProvider> {
    Ok(BatteryProvider {
      config,
      battery_manager: Manager::new()?,
    })
  }

  /// Battery manager from `starship_battery` is not thread-safe, so it
  /// requires its own non-async function.
  pub fn get_variables(
    manager: &Manager,
  ) -> anyhow::Result<ProviderVariables> {
    let battery = manager
      .batteries()
      .and_then(|mut batteries| batteries.nth(0).transpose())
      .unwrap_or(None)
      .context("No battery found.")?;

    Ok(ProviderVariables::Battery(BatteryVariables {
      charge_percent: battery.state_of_charge().get::<percent>(),
      health_percent: battery.state_of_health().get::<percent>(),
      state: battery.state().to_string(),
      is_charging: battery.state() == State::Charging,
      time_till_full: battery
        .time_to_full()
        .map(|time| time.get::<millisecond>()),
      time_till_empty: battery
        .time_to_empty()
        .map(|time| time.get::<millisecond>()),
      power_consumption: battery.energy_rate().get::<watt>(),
      voltage: battery.voltage().get::<volt>(),
      cycle_count: battery.cycle_count(),
    }))
  }
}

#[async_trait]
impl Provider for BatteryProvider {
  async fn on_start(
    self: Arc<Self>,
    shared_state: SharedProviderState,
    emit_output_tx: mpsc::Sender<VariablesResult>,
  ) {
    let mut interval =
      time::interval(Duration::from_millis(self.config.refresh_interval));

    loop {
      interval.tick().await;
      emit_output_tx
        .send(Self::get_variables(&self.battery_manager).into())
        .await
        .unwrap();
    }
  }
}
