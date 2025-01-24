use std::path::PathBuf;

use serde::{Serialize, Serializer};

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error("current executable path has no parent")]
  CurrentExeHasNoParent,
  #[error("unknown program {0}")]
  UnknownProgramName(String),
  #[error("program not allowed on the configured shell scope: {0}")]
  ProgramNotAllowed(PathBuf),
  #[error("unknown encoding {0}")]
  UnknownEncoding(String),
  #[error(transparent)]
  Json(#[from] serde_json::Error),
  #[error(transparent)]
  Utf8(#[from] std::string::FromUtf8Error),
}

impl Serialize for Error {
  fn serialize<S>(
    &self,
    serializer: S,
  ) -> std::result::Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}
