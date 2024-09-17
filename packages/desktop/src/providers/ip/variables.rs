use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IpOutput {
  pub address: String,
  pub approx_city: String,
  pub approx_country: String,
  pub approx_latitude: f32,
  pub approx_longitude: f32,
}
