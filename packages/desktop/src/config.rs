use std::{
  fs::{self},
  path::PathBuf,
  sync::Arc,
};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use tauri::{path::BaseDirectory, AppHandle, Manager};
use tokio::sync::{broadcast, Mutex};
use tracing::{error, info};

use crate::common::{
  copy_dir_all, has_extension, read_and_parse_json, LengthValue, PathExt,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsConfig {
  /// Relative paths to window configs to launch on startup.
  pub startup_configs: Vec<String>,
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

  /// Whether the window should be focused when opened.
  pub focused: bool,

  /// Whether the window should have resize handles.
  pub resizable: bool,

  /// Whether the window frame should be transparent.
  pub transparent: bool,

  /// Where to place the window.
  pub placements: Vec<WindowPlacement>,
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

  /// Monitor(s) to place the window on.
  pub monitor_selection: MonitorSelection,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowAnchor {
  TopLeft,
  TopCenter,
  TopRight,
  CenterLeft,
  Center,
  CenterRight,
  BottomLeft,
  BottomCenter,
  BottomRight,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MonitorSelection {
  All,
  Primary,
  Secondary,
  Index(usize),
  Name(String),
}

#[derive(Debug)]
pub struct Config {
  /// Handle to the Tauri application.
  app_handle: AppHandle,

  /// Directory where config files are stored.
  pub config_dir: PathBuf,

  /// Global settings.
  pub settings: Arc<Mutex<SettingsConfig>>,

  /// List of window configs.
  pub window_configs: Arc<Mutex<Vec<WindowConfigEntry>>>,

  _changes_rx:
    broadcast::Receiver<(SettingsConfig, Vec<WindowConfigEntry>)>,

  pub changes_tx:
    broadcast::Sender<(SettingsConfig, Vec<WindowConfigEntry>)>,
}

#[derive(Clone, Debug)]
pub struct WindowConfigEntry {
  /// Canonicalized absolute path to the window config file.
  pub config_path: String,

  /// Canonicalized absolute path to the window's HTML file.
  pub html_path: String,

  /// Parsed window config.
  pub config: WindowConfig,
}

impl Config {
  /// Reads the config files within the config directory.
  ///
  /// Returns a new `Config` instance.
  pub fn new(
    app_handle: &AppHandle,
    config_dir_override: Option<PathBuf>,
  ) -> anyhow::Result<Self> {
    let config_dir = match config_dir_override {
      Some(dir) => dir,
      None => app_handle
        .path()
        .resolve(".glzr/zebar", BaseDirectory::Home)
        .context("Unable to get home directory.")?,
    }
    .canonicalize_pretty()?
    .into();

    let settings = Self::read_settings_or_init(app_handle, &config_dir)?;
    let window_configs = Self::read_window_configs(&config_dir)?;
    let (changes_tx, _changes_rx) = broadcast::channel(16);

    Ok(Self {
      app_handle: app_handle.clone(),
      config_dir,
      settings: Arc::new(Mutex::new(settings)),
      window_configs: Arc::new(Mutex::new(window_configs)),
      _changes_rx,
      changes_tx,
    })
  }

  /// Re-evaluates config files within the config directory.
  pub async fn reload(&self) -> anyhow::Result<()> {
    let new_settings =
      Self::read_settings_or_init(&self.app_handle, &self.config_dir)?;
    let new_window_configs = Self::read_window_configs(&self.config_dir)?;

    {
      let mut settings = self.settings.lock().await;
      *settings = new_settings.clone();
    }

    {
      let mut window_configs = self.window_configs.lock().await;
      *window_configs = new_window_configs.clone();
    }

    self.changes_tx.send((new_settings, new_window_configs))?;

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

        Ok(
          Self::read_settings(&dir)?
            .context("Failed to create settings config.")?,
        )
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
      true => read_and_parse_json(&settings_path),
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
      } else if has_extension(&path, ".zebar.json") {
        if let Ok(config) = read_and_parse_json::<WindowConfig>(&path) {
          let config_path = path.canonicalize_pretty()?;

          let html_path = path
            .parent()
            .context("Invalid parent directory.")?
            .join(&config.html_path)
            .canonicalize_pretty()?;

          info!("Found valid window config at: {}", config_path);

          configs.push(WindowConfigEntry {
            config,
            config_path,
            html_path,
          });
        } else {
          error!("Failed to parse config: {}", path.display());
        }
      }
    }

    Ok(configs)
  }

  /// Initializes config at the given path from the starter resource.
  fn create_from_starter(
    app_handle: &AppHandle,
    config_dir: &PathBuf,
  ) -> anyhow::Result<()> {
    let starter_path = app_handle
      .path()
      .resolve("resources/config", BaseDirectory::Resource)
      .context("Unable to resolve starter config resource.")?;

    copy_dir_all(&starter_path, config_dir, false)?;

    Ok(())
  }

  pub async fn window_configs(&self) -> Vec<WindowConfigEntry> {
    self.window_configs.lock().await.clone()
  }

  /// Returns the window configs to open on startup.
  pub async fn startup_window_configs(
    &self,
  ) -> anyhow::Result<Vec<WindowConfigEntry>> {
    let startup_configs =
      { self.settings.lock().await.startup_configs.clone() };

    let mut result = Vec::new();

    for config_path in startup_configs {
      let config = self
        .window_config_by_path(&self.join_config_dir(&config_path))
        .await
        .unwrap_or(None)
        .context("Failed to get window config.")?;

      result.push(config);
    }

    Ok(result)
  }

  /// Joins the given path with the config directory path.
  ///
  /// Returns an absolute path.
  pub fn join_config_dir(&self, config_path: &str) -> String {
    self.config_dir.join(config_path).to_unicode_string()
  }

  /// Strips the config directory path from the given path.
  ///
  /// Returns a relative path.
  pub fn strip_config_dir<'a>(
    &self,
    config_path: &'a str,
  ) -> anyhow::Result<&'a str> {
    config_path
      .strip_prefix(&self.config_dir.to_unicode_string())
      .context("Failed to strip config directory path.")
  }

  /// Returns the window config at the given absolute path.
  pub async fn window_config_by_path(
    &self,
    config_path: &str,
  ) -> anyhow::Result<Option<WindowConfigEntry>> {
    let formatted_config_path =
      PathBuf::from(config_path).canonicalize_pretty()?;

    let window_configs = self.window_configs.lock().await;
    let config_entry = window_configs
      .iter()
      .find(|entry| entry.config_path == formatted_config_path);

    Ok(config_entry.cloned())
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
