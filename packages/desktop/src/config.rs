use std::{
  collections::HashMap,
  fs::{self},
  path::PathBuf,
  sync::Arc,
};

use anyhow::Context;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tokio::sync::{broadcast, Mutex};
use tracing::{error, info};

use crate::{
  app_settings::{AppSettings, FrontendTemplate, TemplateResource},
  common::{has_extension, read_and_parse_json, LengthValue, PathExt},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetPackConfig {
  /// JSON schema URL to validate the widget pack file.
  #[serde(rename = "$schema")]
  schema: Option<String>,

  /// Name of the pack.
  pub name: String,

  /// Description of the pack.
  pub description: String,

  /// Tags of the pack.
  pub tags: Vec<String>,

  /// Preview images of the pack.
  pub preview_images: Vec<String>,

  /// Files to exclude from the pack during publishing.
  pub exclude_files: String,

  /// Paths to widgets in the pack.
  pub widget_paths: Vec<PathBuf>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetPack {
  pub id: String,
  pub r#type: WidgetPackType,
  #[serde(flatten)]
  pub config: WidgetPackConfig,
  pub directory_path: PathBuf,
  pub widget_configs: Vec<WidgetConfig>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum WidgetPackType {
  Local,
  Marketplace,
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

  /// How network requests should be cached.
  #[serde(default)]
  pub caching: WidgetCaching,

  /// Privileges for the widget.
  #[serde(default)]
  pub privileges: WidgetPrivileges,

  /// Where to place the widget. Add alias for `defaultPlacements` for
  /// compatibility with v2.3.0 and earlier.
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
#[serde(default, rename_all = "camelCase")]
pub struct WidgetCaching {
  /// Default duration to cache network resources for (in seconds).
  pub default_duration: u32,

  /// Custom cache rules.
  pub rules: Vec<WidgetCachingRule>,
}

impl Default for WidgetCaching {
  fn default() -> Self {
    Self {
      default_duration: 604800,
      rules: Vec::new(),
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetCachingRule {
  /// URL regex pattern to match.
  pub url_regex: String,

  /// Duration to cache the matched requests for (in seconds).
  pub duration: u32,
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

  /// How to reserve space for the widget.
  #[serde(default)]
  pub dock_to_edge: DockConfig,
}

#[derive(
  Clone, Copy, Debug, Deserialize, PartialEq, Serialize, ValueEnum,
)]
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct WidgetPrivileges {
  /// Shell commands that the widget is allowed to run.
  pub shell_commands: Vec<ShellPrivilege>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ShellPrivilege {
  /// Program name (if in PATH) or full path to the program.
  pub program: String,

  /// Arguments to pass to the program.
  pub args_regex: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DockConfig {
  /// Whether to dock the widget to the monitor edge and reserve screen
  /// space for it.
  #[serde(default = "default_bool::<false>")]
  pub enabled: bool,

  /// Edge to dock the widget to.
  pub edge: Option<DockEdge>,

  /// Margin to reserve after the widget window. Can be positive or
  /// negative.
  #[serde(default)]
  pub window_margin: LengthValue,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DockEdge {
  Top,
  Bottom,
  Left,
  Right,
}

impl DockEdge {
  pub fn is_horizontal(&self) -> bool {
    matches!(self, Self::Top | Self::Bottom)
  }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWidgetPackArgs {
  pub name: String,
  pub description: String,
  pub tags: Vec<String>,
  pub preview_images: Vec<String>,
  pub exclude_files: String,
  pub widgets: Vec<CreateWidgetArgs>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWidgetArgs {
  pub name: String,
  pub pack_name: String,
  pub template: FrontendTemplate,
}

#[derive(Debug)]
pub struct Config {
  /// Handle to the Tauri application.
  app_handle: AppHandle,

  /// Reference to `AppSettings`.
  app_settings: Arc<AppSettings>,

  /// List of widget configs.
  pub widget_configs: Arc<Mutex<HashMap<PathBuf, WidgetConfig>>>,

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
    app_settings: Arc<AppSettings>,
  ) -> anyhow::Result<Self> {
    let widget_configs =
      Self::read_widget_configs(&app_settings.config_dir)?;

    let (widget_configs_change_tx, _widget_configs_change_rx) =
      broadcast::channel(16);

    Ok(Self {
      app_handle: app_handle.clone(),
      app_settings,
      widget_configs: Arc::new(Mutex::new(widget_configs)),
      _widget_configs_change_rx,
      widget_configs_change_tx,
    })
  }

  /// Re-evaluates config files within the config directory.
  pub async fn reload(&self) -> anyhow::Result<()> {
    let new_widget_configs =
      Self::read_widget_configs(&self.app_settings.config_dir)?;

    {
      let mut widget_configs = self.widget_configs.lock().await;
      *widget_configs = new_widget_configs.clone();
    }

    self.widget_configs_change_tx.send(new_widget_configs)?;

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
      match Self::parse_widget_config(&path) {
        Ok((config_path, config)) => {
          info!("Found valid widget config at: {}", config_path.display());
          configs.insert(config_path, config);
        }
        Err(err) => {
          error!("{:?}", err);
        }
      }
    }

    Ok(configs)
  }

  fn parse_widget_config(
    config_path: &PathBuf,
  ) -> anyhow::Result<(PathBuf, WidgetConfig)> {
    let abs_path = config_path.to_absolute().with_context(|| {
      format!("Invalid widget config path '{}'.", config_path.display())
    })?;

    let config =
      read_and_parse_json::<WidgetConfig>(&abs_path).map_err(|err| {
        anyhow::anyhow!(
          "Failed to parse widget config at '{}': {:?}",
          abs_path.display(),
          err
        )
      })?;

    Ok((abs_path, config))
  }

  pub async fn widget_configs(&self) -> HashMap<PathBuf, WidgetConfig> {
    self.widget_configs.lock().await.clone()
  }

  /// Updates the widget config at the given path.
  ///
  /// Config path can be either absolute or relative.
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
    }

    // Emit the changed config.
    self
      .widget_configs_change_tx
      .send(HashMap::from([(config_path.clone(), new_config.clone())]))?;

    // Write the updated config to file.
    fs::write(
      &config_path,
      serde_json::to_string_pretty(&new_config)? + "\n",
    )?;

    Ok(())
  }

  /// Formats a widget's config path for display.
  ///
  /// Returns relative path without the `.zebar.json` suffix (e.g.
  /// `starter/vanilla`).
  pub fn formatted_widget_path(&self, config_path: &PathBuf) -> String {
    let path = self
      .app_settings
      .to_relative_path(config_path)
      .to_unicode_string();

    // Ensure path delimiters are forward slashes on Windows.
    #[cfg(windows)]
    let path = path.replace('\\', "/");

    path.strip_suffix(".zebar.json").unwrap_or(&path).into()
  }

  /// Returns the widget config at the given path.
  ///
  /// Config path can be either absolute or relative.
  pub async fn widget_config_by_path(
    &self,
    config_path: &PathBuf,
  ) -> Option<(PathBuf, WidgetConfig)> {
    let abs_path = self.app_settings.to_absolute_path(config_path).ok()?;

    let widget_configs = self.widget_configs.lock().await;
    let config = widget_configs.get(&abs_path)?;

    Some((abs_path, config.clone()))
  }

  /// Creates a new widget pack.
  pub fn create_widget_pack(
    &self,
    args: CreateWidgetPackArgs,
  ) -> anyhow::Result<WidgetPack> {
    let pack_dir = self.app_settings.config_dir.join(&args.name);

    self.app_settings.init_template(
      TemplateResource::Pack,
      &pack_dir,
      &HashMap::from([
        ("PACK_NAME", args.name),
        ("PACK_DESCRIPTION", args.description),
        ("PACK_TAGS", args.tags.join(",")),
        ("PACK_PREVIEW_IMAGES", args.preview_images.join(",")),
      ]),
    )?;

    let mut widget_configs = Vec::new();

    for widget in args.widgets {
      let widget_config = self.create_widget(widget)?;
      widget_configs.push(widget_config);
    }

    // Initialize git repository. Ignore errors.
    let _ = std::process::Command::new("git")
      .arg("init")
      .current_dir(&pack_dir)
      .output();

    let pack_config_path = pack_dir.join("zebar-pack.json");
    let pack_config =
      read_and_parse_json::<WidgetPackConfig>(&pack_config_path)?;

    let pack = WidgetPack {
      id: format!("local.{}", pack_config.name),
      r#type: WidgetPackType::Local,
      directory_path: pack_dir,
      config: pack_config,
      widget_configs,
    };

    Ok(pack)
  }

  /// Creates a new widget from a template.
  ///
  /// Adds a new entry to the pack config and copies the appropriate
  /// frontend template (e.g. React, Solid) to the widget's sub-directory.
  pub fn create_widget(
    &self,
    args: CreateWidgetArgs,
  ) -> anyhow::Result<WidgetConfig> {
    let pack_dir = self.app_settings.config_dir.join(&args.pack_name);
    let widget_dir = pack_dir.join(&args.name);

    self.app_settings.init_template(
      TemplateResource::Widget(args.template),
      &widget_dir,
      &HashMap::new(),
    )?;

    let pack_config_path = pack_dir.join("zebar-pack.json");
    let mut pack_config =
      read_and_parse_json::<WidgetPackConfig>(&pack_config_path)?;

    // Add widget to pack config.
    pack_config.widget_paths.push(widget_dir.clone());

    // Write the updated pack config to file.
    fs::write(
      pack_config_path,
      serde_json::to_string_pretty(&pack_config)? + "\n",
    )?;

    let widget_config_path = widget_dir.join("zebar-widget.json");
    let widget_config =
      read_and_parse_json::<WidgetConfig>(&widget_config_path)?;

    Ok(widget_config)
  }

  /// Deletes a widget from a pack.
  ///
  /// Removes an entry from the pack config and deletes the widget's
  /// sub-directory.
  pub fn delete_widget(
    &self,
    pack_name: &str,
    widget_name: &str,
  ) -> anyhow::Result<()> {
    let pack_dir = self.app_settings.config_dir.join(pack_name);
    let widget_dir = pack_dir.join(widget_name);

    fs::remove_dir_all(&widget_dir)?;

    let pack_config_path = pack_dir.join("zebar-pack.json");
    let mut pack_config =
      read_and_parse_json::<WidgetPackConfig>(&pack_config_path)?;

    // Remove widget from pack config.
    pack_config.widget_paths.retain(|path| path != &widget_dir);

    // Write the updated pack config to file.
    fs::write(
      pack_config_path,
      serde_json::to_string_pretty(&pack_config)? + "\n",
    )?;

    Ok(())
  }
}

/// Helper function for setting a default value for a boolean field.
const fn default_bool<const V: bool>() -> bool {
  V
}

/// Helper function for setting the default value for a
/// `WidgetPreset::name` field.
fn default_preset_name() -> String {
  "default".into()
}
