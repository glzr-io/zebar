use std::fs;

use anyhow::{Context, Result};
use tauri::{api::path::home_dir, AppHandle};

/// Reads the config file at `/.glazer/zanbar.yaml` in the user's home
/// directory.
pub fn read_file(app_handle: AppHandle) -> Result<String> {
  let config_path = home_dir()
    .context("Unable to get home directory.")?
    .join(".glazer")
    .join("zanbar.yaml");

  let sample_path = app_handle
    .path_resolver()
    .resolve_resource("sample-config.yaml")
    .expect("Failed to resolve resource.");

  std::fs::create_dir_all(&config_path).with_context(|| {
    format!("Failed to create directory {}.", &config_path.display())
  })?;

  fs::copy(&sample_path, &config_path).with_context(|| {
    format!(
      "Failed to copy file from {} to {}.",
      &sample_path.display(),
      &config_path.display()
    )
  })?;

  let config_string = fs::read(&config_path)
    .context("Unable to read config file.")?
    .iter()
    .map(|&c| c as char)
    .collect::<String>();

  Ok(config_string)
}
