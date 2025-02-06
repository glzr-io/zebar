use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "function", rename_all = "snake_case")]
pub enum ProviderFunction {
  Audio(AudioFunction),
  Media(MediaFunction),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "name", content = "args", rename_all = "snake_case")]
pub enum AudioFunction {
  SetVolume(SetVolumeArgs),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetVolumeArgs {
  pub volume: f32,
  pub device_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "name", content = "args", rename_all = "snake_case")]
pub enum MediaFunction {
  Play(MediaControlArgs),
  Pause(MediaControlArgs),
  TogglePlayPause(MediaControlArgs),
  Next(MediaControlArgs),
  Previous(MediaControlArgs),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaControlArgs {
  pub session_id: Option<String>,
}

pub type ProviderFunctionResult = Result<ProviderFunctionResponse, String>;

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ProviderFunctionResponse {
  Null,
}
