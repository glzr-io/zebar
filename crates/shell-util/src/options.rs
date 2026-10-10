use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Deserializer, Serialize};

use crate::encoding::Encoding;

#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CommandOptions {
  pub cwd: Option<PathBuf>,

  #[serde(deserialize_with = "deserialize_null_as_default")]
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
      clear_env: false,
      encoding: Encoding::Utf8,
    }
  }
}

fn deserialize_null_as_default<'de, D, T>(
  deserializer: D,
) -> Result<T, D::Error>
where
  D: Deserializer<'de>,
  T: Default + Deserialize<'de>,
{
  Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn null_env_deserializes_to_empty_map() {
    let options: CommandOptions =
      serde_json::from_str(r#"{"env": null}"#).unwrap();

    assert!(options.env.is_empty());
  }

  #[test]
  fn env_map_deserializes() {
    let options: CommandOptions =
      serde_json::from_str(r#"{"env": {"FOO": "bar"}}"#).unwrap();

    assert_eq!(options.env.get("FOO").map(String::as_str), Some("bar"));
  }
}
