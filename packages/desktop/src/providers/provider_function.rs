use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProviderFunction {
  Media(MediaFunction),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaFunction {
  PlayPause,
  Next,
  Previous,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProviderFunctionResult {
  Media(MediaFunctionResult),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaFunctionResult {
  PlayPause(bool),
  Next(bool),
  Previous(bool),
}
