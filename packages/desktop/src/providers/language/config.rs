use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename = "inputlanguage")]
pub struct LanguageProviderConfig {}
