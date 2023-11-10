use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct NetworkVariables {
  pub interfaces: Vec<NetworkInterface>,
}

#[derive(Serialize, Debug, Clone)]
pub struct NetworkInterface {
  pub name: String,
  pub mac_address: String,
  pub transmitted: u64,
  pub total_transmitted: u64,
  pub received: u64,
  pub total_received: u64,
}
