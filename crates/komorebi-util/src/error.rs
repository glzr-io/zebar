use serde::{Serialize, Serializer};

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Failed to initialize socket: {0}")]
  SocketInitialization(#[from] std::io::Error),

  #[error("Failed to parse notification: {0}")]
  NotificationParse(#[from] serde_json::Error),

  #[error("Failed to read stream.")]
  StreamRead,

  #[error("Invalid UTF-8 in buffer: {0}")]
  InvalidUtf8(#[from] std::string::FromUtf8Error),
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
