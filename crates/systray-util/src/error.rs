use serde::{Serialize, Serializer};

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),

  #[error(transparent)]
  Windows(#[from] windows::core::Error),

  #[error("Creation of message window failed.")]
  MessageWindowCreationFailed,

  #[error("Failed to forward message to the real tray window: {0}")]
  ForwardMessageFailed(String),

  #[error("Invalid `COPYDATASTRUCT`.")]
  CopyDataInvalid,
}

impl Serialize for Error {
  fn serialize<S>(
    &self,
    serializer: S,
  ) -> std::result::Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&self.to_string())
  }
}
