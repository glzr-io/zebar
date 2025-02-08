use serde::{Serialize, Serializer};

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Failed to initialize socket: {0}")]
  SocketInitialization(#[from] std::io::Error),

  #[error("Failed to parse output: {0}")]
  OutputParse(#[from] serde_json::Error),

  #[error("Failed to read next output from socket.")]
  SocketRead,

  #[error("Invalid UTF-8 in buffer: {0}")]
  InvalidUtf8(#[from] std::string::FromUtf8Error),

  #[error("Komorebi client has already been stopped.")]
  AlreadyStopped,

  #[error("Failed to get data directory.")]
  DataDir,
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
