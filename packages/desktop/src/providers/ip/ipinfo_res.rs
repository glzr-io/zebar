use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct IpinfoRes {
  pub ip: String,
  pub city: String,
  pub country: String,
  pub loc: String,
}
