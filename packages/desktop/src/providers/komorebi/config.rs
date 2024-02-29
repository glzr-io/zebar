use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename = "komorebi")]
pub struct KomorebiProviderConfig {}
