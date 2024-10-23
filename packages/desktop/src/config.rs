use std::{
  collections::HashMap,
  fs::{self},
  path::PathBuf,
  sync::Arc,
};

use anyhow::Context;
use clap::ValueEnum;
use serde::{Deserialize, Deserializer, Serialize};
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

  /// Widget configs to be launched on startup.
  pub startup_configs: Vec<StartupConfig>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartupConfig {
  /// Relative path to widget configs to launch on startup.
  pub path: PathBuf,

  /// Preset name within the widget config.
  pub preset: String,
}

// Deserializer that handles `StartupConfig` objects and string format from
// v2.3.0 and earlier.
impl<'de> Deserialize<'de> for StartupConfig {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrObject {
      String(String),
      Object(StartupConfig),
    }

    let value = StringOrObject::deserialize(deserializer)?;

    Ok(match value {
      StringOrObject::String(s) => StartupConfig {
        path: PathBuf::from(s),
        preset: "default".to_string(),
      },
      StringOrObject::Object(obj) => obj,
    })
  }
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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

  #[serde(flatten)]
  pub placement: WidgetPlacement,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetPlacement {
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, ValueEnum)]
#[clap(rename_all = "snake_case")]
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
  pub widget_configs: Arc<Mutex<HashMap<PathBuf, WidgetConfig>>>,

  _settings_change_rx: broadcast::Receiver<SettingsConfig>,

  pub settings_change_tx: broadcast::Sender<SettingsConfig>,

  _widget_configs_change_rx:
    broadcast::Receiver<HashMap<PathBuf, WidgetConfig>>,

  pub widget_configs_change_tx:
    broadcast::Sender<HashMap<PathBuf, WidgetConfig>>,
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

        Self::read_settings(&dir)?
          .context("Failed to create settings config.")
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
      serde_json::to_string_pretty(&new_settings)? + "\n",
    )?;

    let mut settings = self.settings.lock().await;
    *settings = new_settings.clone();

    self.settings_change_tx.send(new_settings)?;

    Ok(())
  }

  /// Aggregates all valid widget configs at the 2nd-level of the given
  /// directory (i.e. `<CONFIG_DIR>/*/*.zebar.json`).
  ///
  /// Returns a hashmap of config paths to their `WidgetConfig` instances.
  fn read_widget_configs(
    dir: &PathBuf,
  ) -> anyhow::Result<HashMap<PathBuf, WidgetConfig>> {
    let dir_paths = fs::read_dir(dir)
      .with_context(|| {
        format!("Failed to read directory: {}", dir.display())
      })?
      .filter_map(|entry| Some(entry.ok()?.path()));

    // Scan the 2nd-level of the config directory.
    let subdir_paths = dir_paths
      .filter(|path| path.is_dir())
      .filter_map(|dir| fs::read_dir(dir).ok())
      .flatten()
      .filter_map(|entry| Some(entry.ok()?.path()));

    // Collect the found config files.
    let config_paths = subdir_paths
      .filter(|path| path.is_file() && has_extension(&path, ".zebar.json"))
      .collect::<Vec<PathBuf>>();

    let mut configs = HashMap::new();

    // Parse the found config files.
    for path in config_paths {
      let parse_res = read_and_parse_json::<WidgetConfig>(&path)
        .and_then(|config| Ok((config, path.to_absolute()?)));

      match parse_res {
        Ok((config, config_path)) => {
          info!("Found valid widget config at: {}", config_path.display());

          configs.insert(config_path, config);
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
  /// `settings.json` is initialized with either `starter/vanilla` or
  /// `starter/with-glazewm` as startup config. Widget configs are
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

    let default_settings = SettingsConfig {
      schema: Some("https://github.com/glzr-io/zebar/raw/v2.1.0/resources/settings-schema.json".into()),
      startup_configs: vec![StartupConfig {
        path: match is_app_installed("glazewm") {
          true => "starter/with-glazewm.zebar.json".into(),
          false => "starter/vanilla.zebar.json".into(),
        },
        preset: "default".into(),
      }],
    };

    let settings_path = config_dir.join("settings.json");
    fs::write(
      &settings_path,
      serde_json::to_string_pretty(&default_settings)? + "\n",
    )?;

    Ok(())
  }

  pub async fn widget_configs(&self) -> HashMap<PathBuf, WidgetConfig> {
    self.widget_configs.lock().await.clone()
  }

  /// Returns the widget configs to open on startup.
  pub async fn startup_configs(&self) -> Vec<StartupConfig> {
    self.settings.lock().await.startup_configs.clone()
  }

  /// Returns the widget configs to open on startup.
  pub async fn startup_configs_by_path(
    &self,
  ) -> HashMap<PathBuf, StartupConfig> {
    self
      .startup_configs()
      .await
      .into_iter()
      .map(|config| {
        let abs_path = self.join_config_dir(&config.path);
        (abs_path, config)
      })
      .collect()
  }

  /// Updates the widget config at the given path.
  ///
  /// Config path must be absolute.
  pub async fn update_widget_config(
    &self,
    config_path: &PathBuf,
    new_config: WidgetConfig,
  ) -> anyhow::Result<()> {
    info!("Updating widget config at {}.", config_path.display());

    {
      let mut widget_configs = self.widget_configs.lock().await;

      let config_entry = widget_configs.get_mut(config_path).context(
        format!("Widget config not found at {}.", config_path.display()),
      )?;

      // Update the config in state.
      *config_entry = new_config.clone();

      self.widget_configs_change_tx.send(widget_configs.clone())?;
    }

    // Write the updated config to file.
    fs::write(
      &config_path,
      serde_json::to_string_pretty(&new_config)? + "\n",
    )?;

    Ok(())
  }

  /// Adds the given config to be launched on startup.
  ///
  /// Config path must be absolute.
  pub async fn add_startup_config(
    &self,
    config_path: &PathBuf,
    preset_name: &str,
  ) -> anyhow::Result<()> {
    let mut new_settings = { self.settings.lock().await.clone() };

    let startup_config = StartupConfig {
      path: self.strip_config_dir(config_path)?,
      preset: preset_name.to_string(),
    };

    if new_settings.startup_configs.contains(&startup_config) {
      return Ok(());
    }

    new_settings.startup_configs.push(startup_config);
    self.write_settings(new_settings).await
  }

  /// Removes the given config from being launched on startup.
  ///
  /// Config path must be absolute.
  pub async fn remove_startup_config(
    &self,
    config_path: &PathBuf,
    preset_name: &str,
  ) -> anyhow::Result<()> {
    let rel_path = self.strip_config_dir(config_path)?;
    let mut new_settings = { self.settings.lock().await.clone() };

    new_settings.startup_configs.retain(|config| {
      config.path != rel_path || config.preset != preset_name
    });

    self.write_settings(new_settings).await
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
  ) -> anyhow::Result<Option<(PathBuf, WidgetConfig)>> {
    let abs_config_path = PathBuf::from(config_path).to_absolute()?;

    let widget_configs = self.widget_configs.lock().await;
    let config = widget_configs.get(&abs_config_path);

    Ok(config.map(|config| (abs_config_path, config.clone())))
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
