use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct MemoryVariables {
  pub free_memory: u64,
  pub used_memory: u64,
  pub total_memory: u64,
  pub free_swap: u64,
  pub used_swap: u64,
  pub total_swap: u64,
}
