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
  /// JSON schema URL to validate the settings file.
  #[serde(rename = "$schema")]
  schema: Option<String>,

  /// Relative paths to widget configs to launch on startup.
  pub startup_configs: Vec<PathBuf>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetConfig {
  /// JSON schema URL to validate the widget config file.
  #[serde(rename = "$schema")]
  schema: Option<String>,

  /// Relative path to entry point HTML file.
  pub html_path: PathBuf,

  /// Whether to show the Tauri window above/below all others.
  pub z_order: ZOrder,

  /// Whether the Tauri window should be shown in the taskbar.
  pub shown_in_taskbar: bool,

  /// Whether the Tauri window should be focused when opened.
  pub focused: bool,

  /// Whether the Tauri window should have resize handles.
  pub resizable: bool,

  /// Whether the Tauri window frame should be transparent.
  pub transparent: bool,

  /// Where to place the widget.
  #[serde(alias = "defaultPlacements")]
  pub presets: Vec<WidgetPreset>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ZOrder {
  BottomMost,
  Normal,
  TopMost,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetPreset {
  #[serde(default = "default_preset_name")]
  pub name: String,

  /// Anchor-point of the widget.
  pub anchor: AnchorPoint,

  /// Offset from the anchor-point.
  pub offset_x: LengthValue,

  /// Offset from the anchor-point.
  pub offset_y: LengthValue,

  /// Width of the widget in % or physical pixels.
  pub width: LengthValue,

  /// Height of the widget in % or physical pixels.
  pub height: LengthValue,

  /// Monitor(s) to place the widget on.
  pub monitor_selection: MonitorSelection,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnchorPoint {
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
#[serde(tag = "type", content = "match", rename_all = "snake_case")]
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

  /// List of widget configs.
  pub widget_configs: Arc<Mutex<Vec<WidgetConfigEntry>>>,

  _settings_change_rx: broadcast::Receiver<SettingsConfig>,

  pub settings_change_tx: broadcast::Sender<SettingsConfig>,

  _widget_configs_change_rx: broadcast::Receiver<Vec<WidgetConfigEntry>>,

  pub widget_configs_change_tx: broadcast::Sender<Vec<WidgetConfigEntry>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetConfigEntry {
  /// Absolute path to the widget's config file.
  pub config_path: PathBuf,

  /// Absolute path to the widget's HTML file.
  pub html_path: PathBuf,

  /// Parsed widget config.
  pub config: WidgetConfig,
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
    };

    let settings = Self::read_settings_or_init(app_handle, &config_dir)?;
    let widget_configs = Self::read_widget_configs(&config_dir)?;

    let (settings_change_tx, _settings_change_rx) = broadcast::channel(16);
    let (widget_configs_change_tx, _widget_configs_change_rx) =
      broadcast::channel(16);

    Ok(Self {
      app_handle: app_handle.clone(),
      config_dir: config_dir.to_absolute()?,
      settings: Arc::new(Mutex::new(settings)),
      widget_configs: Arc::new(Mutex::new(widget_configs)),
      _settings_change_rx,
      settings_change_tx,
      _widget_configs_change_rx,
      widget_configs_change_tx,
    })
  }

  /// Re-evaluates config files within the config directory.
  pub async fn reload(&self) -> anyhow::Result<()> {
    let new_settings =
      Self::read_settings_or_init(&self.app_handle, &self.config_dir)?;
    let new_widget_configs = Self::read_widget_configs(&self.config_dir)?;

    {
      let mut settings = self.settings.lock().await;
      *settings = new_settings.clone();
    }

    {
      let mut widget_configs = self.widget_configs.lock().await;
      *widget_configs = new_widget_configs.clone();
    }

    self.settings_change_tx.send(new_settings)?;
    self.widget_configs_change_tx.send(new_widget_configs)?;

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
        Self::create_from_examples(app_handle, dir)?;

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

  /// Writes to the global settings file.
  async fn write_settings(
    &self,
    new_settings: SettingsConfig,
  ) -> anyhow::Result<()> {
    let settings_path = self.config_dir.join("settings.json");

    fs::write(
      &settings_path,
      serde_json::to_string_pretty(&new_settings)?,
    )?;

    let mut settings = self.settings.lock().await;
    *settings = new_settings.clone();

    self.settings_change_tx.send(new_settings)?;

    Ok(())
  }

  /// Aggregates all valid widget configs at the 2nd-level of the given
  /// directory (i.e. `<CONFIG_DIR>/*/*.zebar.json`).
  ///
  /// Returns a vector of `WidgetConfigEntry` instances.
  fn read_widget_configs(
    dir: &PathBuf,
  ) -> anyhow::Result<Vec<WidgetConfigEntry>> {
    let dir_entries = fs::read_dir(dir).with_context(|| {
      format!("Failed to read directory: {}", dir.display())
    })?;

    // Scan the 2nd-level of the directory for config files.
    let config_files = dir_entries
      .into_iter()
      .filter_map(|entry| {
        let path = entry.ok()?.path();
        if path.is_dir() {
          Some(fs::read_dir(path).ok()?)
        } else {
          None
        }
      })
      .flatten()
      .filter_map(|entry| {
        let path = entry.ok()?.path();
        if path.is_file() && has_extension(&path, ".zebar.json") {
          Some(path)
        } else {
          None
        }
      })
      .collect::<Vec<PathBuf>>();

    let mut configs = Vec::new();

    // Parse the found config files.
    for path in config_files {
      let parse_res =
        read_and_parse_json::<WidgetConfig>(&path).and_then(|config| {
          let config_path = path.to_absolute()?;

          let html_path = path
            .parent()
            .and_then(|parent| {
              parent.join(&config.html_path).to_absolute().ok()
            })
            .with_context(|| {
              format!(
                "HTML file not found at {} for config {}.",
                config.html_path.display(),
                config_path.display()
              )
            })?;

          Ok(WidgetConfigEntry {
            config,
            config_path,
            html_path,
          })
        });

      match parse_res {
        Ok(config) => {
          info!(
            "Found valid widget config at: {}",
            config.config_path.display()
          );

          configs.push(config);
        }
        Err(err) => {
          error!(
            "Failed to parse config at {}: {:?}",
            path.display(),
            err
          );
        }
      }
    }

    Ok(configs)
  }

  /// Initializes settings and widget configs at the given path.
  ///
  /// `settings.json` is initialized with either `starter/vanilla.json` or
  /// `starter/with-glazewm.json` as startup config. Widget configs are
  /// initialized from `examples/` directory.
  fn create_from_examples(
    app_handle: &AppHandle,
    config_dir: &PathBuf,
  ) -> anyhow::Result<()> {
    let starter_path = app_handle
      .path()
      .resolve("../../examples", BaseDirectory::Resource)
      .context("Unable to resolve starter config resource.")?;

    info!(
      "Copying starter configs from {} to {}.",
      starter_path.display(),
      config_dir.display()
    );

    copy_dir_all(&starter_path, config_dir, false)?;

    let default_startup_config = match is_app_installed("glazewm") {
      true => "starter/with-glazewm.zebar.json",
      false => "starter/vanilla.zebar.json",
    };

    let default_settings = SettingsConfig {
      schema: Some("https://github.com/glzr-io/zebar/raw/v2.1.0/resources/settings-schema.json".into()),
      startup_configs: vec![default_startup_config.into()],
    };

    let settings_path = config_dir.join("settings.json");
    fs::write(
      &settings_path,
      serde_json::to_string_pretty(&default_settings)?,
    )?;

    Ok(())
  }

  pub async fn widget_configs(&self) -> Vec<WidgetConfigEntry> {
    self.widget_configs.lock().await.clone()
  }

  /// Returns the widget configs to open on startup.
  pub async fn startup_widget_configs(
    &self,
  ) -> anyhow::Result<Vec<WidgetConfigEntry>> {
    let startup_configs =
      { self.settings.lock().await.startup_configs.clone() };

    let mut result = Vec::new();

    for config_path in startup_configs {
      let abs_config_path = self.join_config_dir(&config_path);
      let config = self
        .widget_config_by_path(&abs_config_path)
        .await
        .unwrap_or(None)
        .context(format!(
          "Failed to get widget config at {}.",
          abs_config_path.display()
        ))?;

      result.push(config);
    }

    Ok(result)
  }

  /// Adds the given config to be launched on startup.
  ///
  /// Config path must be absolute.
  pub async fn add_startup_config(
    &self,
    config_path: &PathBuf,
  ) -> anyhow::Result<()> {
    let startup_configs = self.startup_widget_configs().await?;

    // Check if the config is already set to be launched on startup.
    if startup_configs
      .iter()
      .find(|config| config.config_path == *config_path)
      .is_some()
    {
      return Ok(());
    }

    // Add the path to startup configs.
    let mut new_settings = { self.settings.lock().await.clone() };
    new_settings
      .startup_configs
      .push(self.strip_config_dir(config_path)?);

    self.write_settings(new_settings).await?;

    Ok(())
  }

  /// Removes the given config from being launched on startup.
  ///
  /// Config path must be absolute.
  pub async fn remove_startup_config(
    &self,
    config_path: &PathBuf,
  ) -> anyhow::Result<()> {
    let rel_path = self.strip_config_dir(config_path)?;

    let mut new_settings = { self.settings.lock().await.clone() };
    new_settings
      .startup_configs
      .retain(|path| path != &rel_path);

    self.write_settings(new_settings).await?;

    Ok(())
  }

  /// Joins the given path with the config directory path.
  ///
  /// Returns an absolute path.
  pub fn join_config_dir(&self, config_path: &PathBuf) -> PathBuf {
    self.config_dir.join(config_path)
  }

  /// Strips the config directory path from the given path.
  ///
  /// Returns a relative path.
  pub fn strip_config_dir(
    &self,
    config_path: &PathBuf,
  ) -> anyhow::Result<PathBuf> {
    config_path
      .strip_prefix(&self.config_dir)
      .context("Failed to strip config directory path.")
      .map(Into::into)
  }

  /// Returns the widget config at the given absolute path.
  pub async fn widget_config_by_path(
    &self,
    config_path: &PathBuf,
  ) -> anyhow::Result<Option<WidgetConfigEntry>> {
    let formatted_config_path =
      PathBuf::from(config_path).to_absolute()?;

    let widget_configs = self.widget_configs.lock().await;
    let config_entry = widget_configs
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

/// Checks if an application is installed and available in the system PATH.
///
/// Returns `true` if the application is found in PATH, `false` otherwise.
fn is_app_installed(app_name: &str) -> bool {
  #[cfg(target_os = "windows")]
  {
    std::process::Command::new("where")
      .arg(app_name)
      .output()
      .map(|output| output.status.success())
      .unwrap_or(false)
  }

  #[cfg(any(target_os = "macos", target_os = "linux"))]
  {
    std::process::Command::new("which")
      .arg(app_name)
      .output()
      .map(|output| output.status.success())
      .unwrap_or(false)
  }
}

/// Helper function for setting the default value for a
/// `WidgetPreset::name` field.
fn default_preset_name() -> String {
  "default".into()
}
