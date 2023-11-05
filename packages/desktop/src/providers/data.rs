use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ProviderData {
  Cpu(CpuProviderData),
  Network(NetworkProviderData),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CpuProviderData {
  pub refresh_interval_ms: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkProviderData {
  pub list: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HostProviderData {
  pub host_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BatteryProviderData {
  pub charge_percent: f64,
  pub health_percent: f64,
  pub state: State,
  pub secs_until_full: Option<i64>,
  pub secs_until_empty: Option<i64>,
  pub power_consumption_watts: f64,
}
