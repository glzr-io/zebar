use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CpuVariables {
  pub frequency: u64,
  pub usage: f32,
  pub logical_core_count: usize,
  pub physical_core_count: usize,
  pub brand: String,
  pub vendor_id: String,
}
