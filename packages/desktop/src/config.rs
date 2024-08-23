use std::{
  fs::{self},
  path::PathBuf,
};

use anyhow::Context;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tauri::{path::BaseDirectory, AppHandle, Manager};
use tracing::{info, warn};

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
  /// Handle to the Tauri application.
  app_handle: AppHandle,

  /// Directory where config files are stored.
  pub config_dir: PathBuf,

  /// Global settings.
  pub settings: SettingsConfig,

  /// List of window configs.
  pub window_configs: Vec<WindowConfigEntry>,
}

#[derive(Debug)]
pub struct WindowConfigEntry {
  pub path: PathBuf,
  pub config: WindowConfig,
}

impl Config {
  /// Reads the config files within the config directory and creates a new
  /// `Config` instance.
  pub fn new(app_handle: &AppHandle) -> anyhow::Result<Self> {
    let config_dir = app_handle
      .path()
      .resolve(".glzr/zebar", BaseDirectory::Home)
      .context("Unable to get home directory.")?
      .canonicalize()?;

    let settings = Self::read_settings_or_init(app_handle, &config_dir)?;
    let window_configs = Self::read_window_configs(&config_dir)?;

    Ok(Self {
      app_handle: app_handle.clone(),
      config_dir,
      settings,
      window_configs,
    })
  }

  /// Re-evaluates config files within the config directory.
  pub fn reload(&mut self) -> anyhow::Result<()> {
    self.settings =
      Self::read_settings_or_init(&self.app_handle, &self.config_dir)?;

    self.window_configs = Self::read_window_configs(&self.config_dir)?;

    Ok(())
  }

  /// Reads the global settings file or initializes it with the starter.
  ///
  /// Returns the parsed `SettingsConfig`.
  fn read_settings_or_init(
    app_handle: &AppHandle,
    dir: &PathBuf,
  ) -> anyhow::Result<SettingsConfig> {
    let settings = Self::read_settings(&dir)?;

    match settings {
      Some(settings) => Ok(settings),
      None => {
        Self::create_from_starter(app_handle, dir)?;
        Ok(Self::read_settings(&dir)?.unwrap())
      }
    }
  }

  /// Reads the global settings file.
  ///
  /// Returns the parsed `SettingsConfig` if found.
  fn read_settings(
    dir: &PathBuf,
  ) -> anyhow::Result<Option<SettingsConfig>> {
    let settings_path = dir.join("settings.json");

    match settings_path.exists() {
      false => Ok(None),
      true => Self::read_and_parse_json(&settings_path),
    }
  }

  /// Recursively aggregates all valid window configs in the given
  /// directory.
  ///
  /// Returns a list of `ConfigEntry` instances.
  fn read_window_configs(
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
        // Recursively aggregate configs in subdirectories.
        configs.extend(Self::read_window_configs(&path)?);
      } else if Self::is_json(&path) {
        if let Ok(config) = Self::read_and_parse_json(&path) {
          info!("Found valid window config at: {}", path.display());

          configs.push(WindowConfigEntry {
            config,
            path: path.clone(),
          });
        } else {
          // TODO: Show error dialog.
          warn!("Failed to process file: {}", path.display());
        }
      }
    }

    Ok(configs)
  }

  /// Returns whether the given path is a JSON file.
  fn is_json(path: &PathBuf) -> bool {
    path.extension().and_then(|ext| ext.to_str()) == Some("json")
  }

  /// Reads a JSON file and parses it into the specified type.
  ///
  /// Returns the parsed type `T` if successful.
  fn read_and_parse_json<T: DeserializeOwned>(
    path: &PathBuf,
  ) -> anyhow::Result<T> {
    let content = fs::read_to_string(path).with_context(|| {
      format!("Failed to read file: {}", path.display())
    })?;

    let parsed = serde_json::from_str(&content).with_context(|| {
      format!("Failed to parse JSON from file: {}", path.display())
    })?;

    Ok(parsed)
  }

  /// Initialize config at the given path from the starter resource.
  fn create_from_starter(
    app_handle: &AppHandle,
    config_dir: &PathBuf,
  ) -> anyhow::Result<()> {
    let starter_path = app_handle
      .path()
      .resolve("resources/config", BaseDirectory::Resource)
      .context("Unable to resolve sample config resource.")?;

    // Create the destination directory.
    fs::create_dir_all(&config_dir).with_context(|| {
      format!("Unable to create directory {}.", &config_dir.display())
    })?;

    Self::copy_dir_all(&starter_path, config_dir)?;

    Ok(())
  }

  fn copy_dir_all(src: &PathBuf, dest: &PathBuf) -> anyhow::Result<()> {
    fs::create_dir_all(&dest)?;

    for entry in fs::read_dir(src)? {
      let entry = entry?;

      if entry.file_type()?.is_dir() {
        Self::copy_dir_all(&entry.path(), &dest.join(entry.file_name()))?;
      } else {
        fs::copy(entry.path(), dest.join(entry.file_name()))?;
      }
    }

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