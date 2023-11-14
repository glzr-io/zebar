use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use tauri::{path::BaseDirectory, AppHandle, Manager};

/// Reads the config file at `/.glazer/zebar.yaml` in the user's home
/// directory.
pub fn read_file(
  config_path_override: Option<&str>,
  app_handle: AppHandle,
) -> Result<String> {
  let default_config_path = app_handle
    .path()
    .resolve(".glazer/zebar.yaml", BaseDirectory::Home)
    .context("Unable to get home directory.")?;

  let config_path = match config_path_override {
    Some(val) => PathBuf::from(val),
    None => default_config_path,
  };

  // Create new config file from sample if it doesn't exist.
  match config_path.exists() {
    true => {
      fs::read_to_string(&config_path).context("Unable to read config file.")
    }
    false => create_from_sample(&config_path, app_handle),
  }
}

fn create_from_sample(
  config_path: &PathBuf,
  app_handle: AppHandle,
) -> Result<String> {
  let sample_config = app_handle
    .asset_resolver()
    .get("resources/sample-config.yaml".to_owned())
    .context("Failed to resolve sample config.")?;

  // Read the contents of the sample config.
  // let config_string = String::from_utf8_lossy(&sample_path.bytes);

  let parent_dir = config_path.parent().context("Invalid config directory.")?;

  // Create the containing directory for the config file.
  std::fs::create_dir_all(&parent_dir).with_context(|| {
    format!("Unable to create directory {}.", &config_path.display())
  })?;

  // fs::write(&config_path, &config_string)
  fs::write(&config_path, &sample_config.bytes)
    .context("Unable to write config file.")?;

  Ok(String::from_utf8_lossy(&sample_config.bytes).to_string())
}
