use komorebi_client::{Monitor, Ring};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KomorebiVariables {
  pub monitors: Ring<Monitor>,
}
