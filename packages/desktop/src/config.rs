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
  app_settings::{
    AppSettings, FrontendTemplate, TemplateResource, VERSION_NUMBER,
  },
  common::{read_and_parse_json, LengthValue, PathExt},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetPack {
  /// Unique identifier for the pack.
  pub id: String,

  /// Whether the pack is a local or marketplace pack.
  pub r#type: WidgetPackType,

  /// Deserialized pack config.
  #[serde(flatten)]
  pub config: WidgetPackConfig,

  /// Path to the pack config file.
  pub config_path: PathBuf,

  /// Path to the directory containing the pack config.
  ///
  /// This is the parent directory of `config_path`.
  pub directory_path: PathBuf,

  /// Widget configs and their absolute paths by widget name.
  pub widget_configs: HashMap<String, (PathBuf, WidgetConfig)>,
}

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

  /// Name of the widget.
  pub name: String,

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
  pub exclude_files: String,
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

  /// Map of widget packs by their ID's.
  pub widget_packs: Arc<Mutex<HashMap<String, WidgetPack>>>,

  _widget_packs_change_rx:
    broadcast::Receiver<HashMap<String, WidgetPack>>,

  pub widget_packs_change_tx:
    broadcast::Sender<HashMap<String, WidgetPack>>,

  _widget_configs_change_rx:
    broadcast::Receiver<HashMap<String, WidgetConfig>>,

  pub widget_configs_change_tx:
    broadcast::Sender<HashMap<String, WidgetConfig>>,
}

impl Config {
  /// Reads the config files within the config directory.
  ///
  /// Returns a new `Config` instance.
  pub fn new(
    app_handle: &AppHandle,
    app_settings: Arc<AppSettings>,
  ) -> anyhow::Result<Self> {
    let widget_packs = Self::read_widget_packs(&app_settings.config_dir)?;

    let (widget_packs_change_tx, _widget_packs_change_rx) =
      broadcast::channel(16);

    let (widget_configs_change_tx, _widget_configs_change_rx) =
      broadcast::channel(16);

    Ok(Self {
      app_handle: app_handle.clone(),
      app_settings,
      widget_packs: Arc::new(Mutex::new(widget_packs)),
      _widget_packs_change_rx,
      widget_packs_change_tx,
      _widget_configs_change_rx,
      widget_configs_change_tx,
    })
  }

  /// Re-evaluates config files within the config directory.
  pub async fn reload(&self) -> anyhow::Result<()> {
    let new_widget_packs =
      Self::read_widget_packs(&self.app_settings.config_dir)?;

    {
      let mut widget_packs = self.widget_packs.lock().await;
      *widget_packs = new_widget_packs.clone();
    }

    self.widget_packs_change_tx.send(new_widget_packs)?;

    Ok(())
  }

  /// Aggregates all valid widget packs at the 2nd-level of the given
  /// directory (i.e. `<CONFIG_DIR>/*/zebar-pack.json`).
  ///
  /// Returns a hashmap of widget pack ID's to their `WidgetPack`
  /// instances.
  fn read_widget_packs(
    dir: &PathBuf,
  ) -> anyhow::Result<HashMap<String, WidgetPack>> {
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
      .filter(|path| {
        path.is_file()
          && path
            .file_name()
            .and_then(|name| name.to_str())
            .map(|name| name == "zebar-pack.json")
            .unwrap_or(false)
      })
      .collect::<Vec<PathBuf>>();

    let mut packs = HashMap::new();

    // Parse the found config files.
    for path in config_paths {
      match Self::parse_widget_pack(&path) {
        Ok(pack) => {
          tracing::info!("Found valid widget pack at: {}", path.display());
          packs.insert(pack.id.clone(), pack);
        }
        Err(err) => {
          error!("{:?}", err);
        }
      }
    }

    Ok(packs)
  }

  fn parse_widget_pack(
    config_path: &PathBuf,
  ) -> anyhow::Result<WidgetPack> {
    let abs_path = config_path.to_absolute().with_context(|| {
      format!("Invalid widget pack path '{}'.", config_path.display())
    })?;

    let pack_config = read_and_parse_json::<WidgetPackConfig>(&abs_path)
      .map_err(|err| {
      anyhow::anyhow!(
        "Failed to parse widget pack at '{}': {:?}",
        abs_path.display(),
        err
      )
    })?;

    let mut widget_configs = HashMap::new();

    // Parse the found widget config files.
    for path in &pack_config.widget_paths {
      match Self::parse_widget_config(&path) {
        Ok((config_path, widget_config)) => {
          info!("Found valid widget config at: {}", config_path.display());

          widget_configs.insert(
            widget_config.name.clone(),
            (config_path, widget_config),
          );
        }
        Err(err) => {
          error!("{:?}", err);
        }
      }
    }

    let pack = WidgetPack {
      id: format!("local.{}", pack_config.name),
      r#type: WidgetPackType::Local,
      config_path: abs_path,
      directory_path: config_path
        .parent()
        .context(format!(
          "Failed to get parent directory of '{}'.",
          config_path.display()
        ))?
        .to_path_buf(),
      config: pack_config,
      widget_configs,
    };

    Ok(pack)
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

  /// Returns all widget packs as a hashmap.
  pub async fn widget_packs(&self) -> HashMap<String, WidgetPack> {
    self.widget_packs.lock().await.clone()
  }

  /// Finds a widget pack by ID.
  pub async fn widget_pack_by_id(
    &self,
    pack_id: &str,
  ) -> Option<WidgetPack> {
    let widget_packs = self.widget_packs.lock().await;
    widget_packs.get(pack_id).cloned()
  }

  /// Updates the widget config for the given pack and widget name.
  pub async fn update_widget_config(
    &self,
    pack_id: &str,
    widget_name: &str,
    new_config: WidgetConfig,
  ) -> anyhow::Result<()> {
    tracing::info!("Updating widget config for {}.", widget_name);

    let config_path = {
      let mut widget_packs = self.widget_packs.lock().await;

      let pack_entry = widget_packs
        .get_mut(pack_id)
        .context(format!("Widget pack not found for {}.", pack_id))?;

      let widget_configs = &mut pack_entry.widget_configs;

      let config_entry = widget_configs.get_mut(widget_name).context(
        format!("Widget config not found for {}.", widget_name),
      )?;

      // Store the config path before updating the entry.
      let config_path = config_entry.0.clone();

      // Update the config in state.
      *config_entry = (config_path.clone(), new_config.clone());

      config_path
    };

    // Emit the changed config.
    self.widget_configs_change_tx.send(HashMap::from([(
      widget_name.to_string(),
      new_config.clone(),
    )]))?;

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
        ("ZEBAR_VERSION", VERSION_NUMBER.to_string()),
      ]),
    )?;

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
      config_path: pack_config_path,
      directory_path: pack_dir,
      config: pack_config,
      widget_configs: HashMap::new(),
    };

    Ok(pack)
  }

  /// Updates a widget pack.
  pub fn update_widget_pack(
    &self,
    pack_id: String,
    args: CreateWidgetPackArgs,
  ) -> anyhow::Result<WidgetPack> {
    // TODO: Implement.
    anyhow::bail!("Not implemented")
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
      &HashMap::from([
        ("WIDGET_NAME", args.name),
        ("ZEBAR_VERSION", VERSION_NUMBER.to_string()),
      ]),
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
