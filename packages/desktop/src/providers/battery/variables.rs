use serde::Serialize;

#[derive(Serialize, Debug, Clone, Default)]
pub struct BatteryVariables {
  pub charge_percent: f32,
  pub health_percent: f32,
  pub state: String,
  pub time_till_full: Option<f32>,
  pub time_till_empty: Option<f32>,
  pub power_consumption: f32,
  pub voltage: f32,
  pub cycle_count: Option<u32>,
}
