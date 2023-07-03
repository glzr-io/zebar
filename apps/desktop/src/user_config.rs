use std::{fs, io::Read, path::PathBuf};

use anyhow::{Context, Result};
use tauri::{api::path::home_dir, AppHandle};

/// Reads the config file at `/.glazer/zanbar.yaml` in the user's home
/// directory.
pub fn read_file(
  config_path_override: Option<&str>,
  app_handle: AppHandle,
) -> Result<String> {
  let default_config_path = home_dir()
    .context("Unable to get home directory.")?
    .join(".glazer")
    .join("zanbar.yaml");

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
  let sample_path = app_handle
    .path_resolver()
    .resolve_resource("resources/sample-config.yaml")
    .context("Failed to resolve sample config.")?;

  let mut sample_file =
    fs::File::open(&sample_path).context("Unable to read sample config.")?;

  // Read the contents of the sample config.
  let mut config_string = String::new();
  sample_file.read_to_string(&mut config_string)?;

  let parent_dir = config_path.parent().context("Invalid config directory.")?;

  // Create the containing directory for the config file.
  std::fs::create_dir_all(&parent_dir).with_context(|| {
    format!("Unable to create directory {}.", &config_path.display())
  })?;

  fs::write(&config_path, &config_string)
    .context("Unable to write config file.")?;

  Ok(config_string)
}
