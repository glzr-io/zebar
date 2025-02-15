use std::{
  fs::{self, DirEntry},
  path::{Path, PathBuf},
};

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

/// Returns whether the path has the given extension.
pub fn has_extension(path: &Path, extension: &str) -> bool {
  path
    .file_name()
    .and_then(|name| name.to_str())
    .map(|name| name.ends_with(extension))
    .unwrap_or(false)
}

/// Recursively copies a directory and all its contents to a new file
/// location.
///
/// Optionally replaces existing files in the destination directory if
/// `override_existing` is `true`.
pub fn copy_dir_all(
  src_dir: &Path,
  dest_dir: &Path,
  override_existing: bool,
) -> anyhow::Result<()> {
  fs::create_dir_all(dest_dir)?;

  visit_deep(src_dir, &|entry| {
    let dest_path = dest_dir.join(entry.file_name());

    if override_existing || !dest_path.exists() {
      if let Err(err) = fs::copy(entry.path(), dest_path) {
        error!("Failed to copy file: {}", err);
      }
    }
  })
}

/// Recursively visit files in a directory.
///
/// The callback is invoked for each entry in the directory.
pub fn visit_deep(
  dir: &Path,
  callback: &dyn Fn(&DirEntry),
) -> anyhow::Result<()> {
  if dir.is_dir() {
    let read_dir = std::fs::read_dir(dir).with_context(|| {
      format!("Failed to read directory {}.", dir.display())
    })?;

    for entry in read_dir {
      let entry = entry?;
      let path = entry.path();

      if path.is_dir() {
        visit_deep(&path, callback)?;
      } else {
        callback(&entry);
      }
    }
  }

  Ok(())
}
