use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BatteryOutput {
  pub has_battery: bool,
  pub charge_percent: Option<f32>,
  pub health_percent: Option<f32>,
  pub state: String,
  pub is_charging: bool,
  pub time_till_full: Option<f32>,
  pub time_till_empty: Option<f32>,
  pub power_consumption: Option<f32>,
  pub voltage: Option<f32>,
  pub cycle_count: Option<u32>,
}
