use std::{fs, path::PathBuf};

use anyhow::Context;
use serde::de::DeserializeOwned;

/// Reads a JSON file and parses it into the specified type.
///
/// Returns the parsed type `T` if successful.
pub fn read_and_parse_json<T: DeserializeOwned>(
  path: &PathBuf,
) -> anyhow::Result<T> {
  let content = fs::read_to_string(path)
    .with_context(|| format!("Failed to read file: {}", path.display()))?;

  let parsed = serde_json::from_str(&content).with_context(|| {
    format!("Failed to parse JSON from file: {}", path.display())
  })?;

  Ok(parsed)
}

/// Recursively copies a directory and all its contents to a new file
/// location.
pub fn copy_dir_all(src: &PathBuf, dest: &PathBuf) -> anyhow::Result<()> {
  fs::create_dir_all(&dest)?;

  for entry in fs::read_dir(src)? {
    let entry = entry?;

    if entry.file_type()?.is_dir() {
      copy_dir_all(&entry.path(), &dest.join(entry.file_name()))?;
    } else {
      fs::copy(entry.path(), dest.join(entry.file_name()))?;
    }
  }

  Ok(())
}
