use std::{fs, path::PathBuf};

use anyhow::Context;
use tauri::{path::BaseDirectory, AppHandle, Manager};

/// Reads the config file at `~/.glzr/zebar/config.yaml`.
pub fn read_file(
  config_path_override: Option<&str>,
  app_handle: AppHandle,
) -> anyhow::Result<String> {
  let default_config_path = app_handle
    .path()
    .resolve(".glzr/zebar/config.yaml", BaseDirectory::Home)
    .context("Unable to get home directory.")?;

  let config_path = match config_path_override {
    Some(val) => PathBuf::from(val),
    None => default_config_path,
  };

  // Create new config file from sample if it doesn't exist.
  if !config_path.exists() {
    create_from_sample(&config_path, app_handle)?;
  }

  fs::read_to_string(&config_path).context("Unable to read config file.")
}

/// Initialize config at the given path from the sample config resource.
fn create_from_sample(
  config_path: &PathBuf,
  app_handle: AppHandle,
) -> anyhow::Result<()> {
  let sample_path = app_handle
    .path()
    .resolve("resources/sample-config.yaml", BaseDirectory::Resource)
    .context("Unable to resolve sample config resource.")?;

  let sample_script = app_handle
    .path()
    .resolve("resources/script.js", BaseDirectory::Resource)
    .context("Unable to resolve sample script resource.")?;

  let dest_dir =
    config_path.parent().context("Invalid config directory.")?;

  // Create the destination directory.
  std::fs::create_dir_all(&dest_dir).with_context(|| {
    format!("Unable to create directory {}.", &config_path.display())
  })?;

  // Copy over sample config.
  let config_path = dest_dir.join("config.yaml");
  fs::copy(&sample_path, &config_path).with_context(|| {
    format!("Unable to write to {}.", config_path.display())
  })?;

  // Copy over sample script.
  let script_path = dest_dir.join("script.js");
  fs::copy(&sample_script, &script_path).with_context(|| {
    format!("Unable to write to {}.", script_path.display())
  })?;

  Ok(())
}

pub fn open_config_dir(app_handle: &AppHandle) -> anyhow::Result<()> {
  let dir_path = app_handle
    .path()
    .resolve(".glzr/zebar", BaseDirectory::Home)
    .context("Unable to get home directory.")?
    .canonicalize()?;

  #[cfg(target_os = "windows")]
  {
    std::process::Command::new("explorer")
      .arg(dir_path)
      .spawn()?;
  }

  #[cfg(target_os = "macos")]
  {
    std::process::Command::new("open")
      .arg(dir_path)
      .arg("-R")
      .spawn()?;
  }

  #[cfg(target_os = "linux")]
  {
    std::process::Command::new("xdg-open")
      .arg(dir_path)
      .spawn()?;
  }

  Ok(())
}
