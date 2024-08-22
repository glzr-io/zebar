use std::{fs, path::PathBuf};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use tauri::{path::BaseDirectory, AppHandle, Manager};

use crate::util::LengthValue;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WindowConfig {
  /// Entry point HTML file.
  html_path: String,

  /// Default options for when the window is opened.
  launch_options: WindowLaunchOptions,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WindowLaunchOptions {
  /// Whether to show the window above/below all others.
  z_order: WindowZOrder,

  /// Whether the window should be shown in the taskbar.
  shown_in_taskbar: bool,

  /// Whether the window should have resize handles.
  resizable: bool,

  /// Whether the window frame should be transparent.
  transparent: bool,

  /// Where to place the window.
  placement: WindowPlacement,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum WindowZOrder {
  AlwaysOnBottom,
  AlwaysOnTop,
  Normal,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WindowPlacement {
  /// The monitor index to place the window on.
  monitor: u32,

  /// Anchor-point of the window.
  anchor: WindowAnchor,

  /// Offset from the anchor-point.
  offset_x: LengthValue,

  /// Offset from the anchor-point.
  offset_y: LengthValue,

  /// Width of the window in % or physical pixels.
  width: LengthValue,

  /// Height of the window in % or physical pixels.
  height: LengthValue,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum WindowAnchor {
  TopLeft,
  TopCenter,
  TopRight,
  CenterLeft,
  Center,
  CenterCenter,
  CenterRight,
  BottomLeft,
  BottomCenter,
  BottomRight,
}

/// Reads the config file at `~/.glzr/zebar/config.yaml`.
pub fn read_file(
  config_path_override: Option<&str>,
  app_handle: &AppHandle,
) -> anyhow::Result<WindowConfig> {
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

  let config_str = fs::read_to_string(&config_path)
    .context("Unable to read config file.")?;

  // TODO: Improve error formatting of serde_yaml errors. Something
  // similar to https://github.com/AlexanderThaller/format_serde_error
  let config_value = serde_yaml::from_str(&config_str)?;

  Ok(config_value)
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
