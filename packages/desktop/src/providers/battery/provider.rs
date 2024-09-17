use anyhow::Context;
use starship_battery::{
  units::{
    electric_potential::volt, power::watt, ratio::percent,
    time::millisecond,
  },
  Manager, State,
};

use super::{BatteryOutput, BatteryProviderConfig};
use crate::{impl_interval_provider, providers::ProviderOutput};

pub struct BatteryProvider {
  config: BatteryProviderConfig,
}

impl BatteryProvider {
  pub fn new(config: BatteryProviderConfig) -> BatteryProvider {
    BatteryProvider { config }
  }

  fn refresh_interval_ms(&self) -> u64 {
    self.config.refresh_interval
  }

  async fn run_interval(&self) -> anyhow::Result<ProviderOutput> {
    let battery = Manager::new()?
      .batteries()
      .and_then(|mut batteries| batteries.nth(0).transpose())
      .unwrap_or(None)
      .context("No battery found.")?;

    Ok(ProviderOutput::Battery(BatteryOutput {
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

impl_interval_provider!(BatteryProvider);
