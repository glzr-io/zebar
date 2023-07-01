use std::fs;

use anyhow::{Context, Result};
use tauri::api::path::home_dir;

/// Reads the config file at `/.glazer/zanbar.yaml` in the user's home
/// directory.
pub fn read_file() -> Result<String> {
  let path = home_dir()
    .context("Unable to get home directory.")?
    .join(".glazer")
    .join("zanbar.yaml");

  let config_string = fs::read(path)
    .context("Unable to read config file.")?
    .iter()
    .map(|&c| c as char)
    .collect::<String>();

  Ok(config_string)
}
