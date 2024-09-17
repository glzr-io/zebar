use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryOutput {
  pub usage: f32,
  pub free_memory: u64,
  pub used_memory: u64,
  pub total_memory: u64,
  pub free_swap: u64,
  pub used_swap: u64,
  pub total_swap: u64,
}
