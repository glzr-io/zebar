use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CpuOutput {
  pub frequency: u64,
  pub usage: f32,
  pub logical_core_count: usize,
  pub physical_core_count: usize,
  pub vendor: String,
}
