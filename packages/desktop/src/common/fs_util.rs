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

/// Returns whether the given path is a JSON file.
pub fn is_json(path: &PathBuf) -> bool {
  path.extension().and_then(|ext| ext.to_str()) == Some("json")
}

/// Recursively copies a directory and all its contents to a new file
/// location.
///
/// Optionally replaces existing files in the destination directory if
/// `override_existing` is `true`.
pub fn copy_dir_all(
  src_dir: &PathBuf,
  dest_dir: &PathBuf,
  override_existing: bool,
) -> anyhow::Result<()> {
  fs::create_dir_all(&dest_dir)?;

  for entry in fs::read_dir(src_dir)? {
    let entry = entry?;
    let path = entry.path();

    if entry.file_type()?.is_dir() {
      copy_dir_all(
        &path,
        &dest_dir.join(entry.file_name()),
        override_existing,
      )?;
    } else if override_existing || !path.exists() {
      fs::copy(path, dest_dir.join(entry.file_name()))?;
    }
  }

  Ok(())
}
