use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::encoding::Encoding;

#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CommandOptions {
  pub cwd: Option<PathBuf>,

  pub env: HashMap<String, String>,

  /// Clear the environment variables of the spawned process.
  pub clear_env: bool,

  /// Character encoding for stdout/stderr.
  pub encoding: Encoding,
}

impl Default for CommandOptions {
  fn default() -> Self {
    Self {
      cwd: None,
      env: HashMap::default(),
      clear_env: true,
      encoding: Encoding::Utf8,
    }
  }
}
