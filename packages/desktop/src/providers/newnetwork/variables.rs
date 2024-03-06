use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewNetworkVariables {
  pub name: String,
  pub mac_address: String,
  pub strength: String,
}
