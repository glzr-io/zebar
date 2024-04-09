use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetworkActivityVariables {
  pub received: u64,
  pub transmitted: u64,
}
