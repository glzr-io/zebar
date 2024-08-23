use std::{
  fs::{self},
  path::PathBuf,
};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use tauri::{path::BaseDirectory, AppHandle, Manager};

use crate::util::LengthValue;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsConfig {
  /// Relative paths to Zebar window configs to launch on start.
  pub launch_on_start: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowConfig {
  /// Entry point HTML file.
  pub html_path: String,

  /// Default options for when the window is opened.
  pub launch_options: WindowLaunchOptions,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowLaunchOptions {
  /// Whether to show the window above/below all others.
  pub z_order: WindowZOrder,

  /// Whether the window should be shown in the taskbar.
  pub shown_in_taskbar: bool,

  /// Whether the window should have resize handles.
  pub resizable: bool,

  /// Whether the window frame should be transparent.
  pub transparent: bool,

  /// Where to place the window.
  pub placement: Vec<WindowPlacement>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowZOrder {
  AlwaysOnBottom,
  AlwaysOnTop,
  Normal,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowPlacement {
  /// The monitor index to place the window on.
  pub monitor: u32,

  /// Anchor-point of the window.
  pub anchor: WindowAnchor,

  /// Offset from the anchor-point.
  pub offset_x: LengthValue,

  /// Offset from the anchor-point.
  pub offset_y: LengthValue,

  /// Width of the window in % or physical pixels.
  pub width: LengthValue,

  /// Height of the window in % or physical pixels.
  pub height: LengthValue,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
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

#[derive(Debug)]
pub struct Config {
  /// Directory where config files are stored.
  pub config_dir: PathBuf,

  /// Global settings.
  // pub settings: SettingsConfig,

  /// List of window configs.
  pub window_configs: Vec<WindowConfigEntry>,
}

#[derive(Debug)]
pub struct WindowConfigEntry {
  pub path: PathBuf,
  pub config: WindowConfig,
}

impl Config {
  /// Creates a new `Config` instance.
  pub fn new(app_handle: &AppHandle) -> anyhow::Result<Self> {
    let config_dir = app_handle
      .path()
      .resolve(".glzr/zebar", BaseDirectory::Home)
      .context("Unable to get home directory.")?
      .canonicalize()?;

    Ok(Self {
      config_dir,
      window_configs: Vec::new(),
    })
  }

  /// Reads the config files within the config directory.
  pub fn read(&mut self) -> anyhow::Result<()> {
    self.window_configs = Self::aggregate_configs(&self.config_dir)?;
    Ok(())
  }

  /// Recursively aggregates all valid window configs in the given
  /// directory and its subdirectories.
  ///
  /// Returns a list of `ConfigEntry` instances.
  fn aggregate_configs(
    dir: &PathBuf,
  ) -> anyhow::Result<Vec<WindowConfigEntry>> {
    let mut configs = Vec::new();

    let entries = fs::read_dir(dir).with_context(|| {
      format!("Failed to read directory: {}", dir.display())
    })?;

    for entry in entries {
      let entry = entry?;
      let path = entry.path();

      if path.is_dir() {
        // Recursively aggregate configs on subdirectories.
        configs.extend(Self::aggregate_configs(&path)?);
      } else if Self::is_json(&path) {
        if let Ok(config) = Self::process_file(&path) {
          configs.push(WindowConfigEntry {
            config,
            path: path.clone(),
          });
        }
      }
    }

    Ok(configs)
  }

  /// Returns whether the given path is a JSON file.
  fn is_json(path: &PathBuf) -> bool {
    path.extension().and_then(|ext| ext.to_str()) == Some("json")
  }

  /// Processes a single JSON file, reading its contents and parsing it.
  ///
  /// Returns the parsed `WindowConfig` if successful.
  fn process_file(path: &PathBuf) -> anyhow::Result<WindowConfig> {
    let content = fs::read_to_string(path).with_context(|| {
      format!("Failed to read file: {}", path.display())
    })?;

    let config = serde_json::from_str(&content).with_context(|| {
      format!("Failed to parse JSON from file: {}", path.display())
    })?;

    Ok(config)
  }

  /// Initialize config at the given path from the sample config resource.
  fn create_from_sample(
    &self,
    app_handle: &AppHandle,
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

  /// Opens the config directory in the OS-dependent file explorer.
  pub fn open_config_dir(&self) -> anyhow::Result<()> {
    #[cfg(target_os = "windows")]
    {
      std::process::Command::new("explorer")
        .arg(self.config_dir.clone())
        .spawn()?;
    }

    #[cfg(target_os = "macos")]
    {
      std::process::Command::new("open")
        .arg(self.config_dir.clone())
        .arg("-R")
        .spawn()?;
    }

    #[cfg(target_os = "linux")]
    {
      std::process::Command::new("xdg-open")
        .arg(self.config_dir.clone())
        .spawn()?;
    }

    Ok(())
  }
}
