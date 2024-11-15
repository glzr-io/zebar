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
