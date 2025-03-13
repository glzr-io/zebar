use std::{
  collections::HashMap,
  fs::{self},
  path::{Path, PathBuf},
  sync::Arc,
};

use anyhow::Context;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tokio::sync::{broadcast, Mutex};

use crate::{
  app_settings::{
    AppSettings, FrontendTemplate, TemplateResource, VERSION_NUMBER,
  },
  common::{has_extension, read_and_parse_json, LengthValue, PathExt},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetPack {
  /// Unique identifier for the pack.
  pub id: String,

  /// Name of the pack.
  pub name: String,

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

  /// List of widget configs.
  pub widget_configs: Vec<WidgetConfigEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetConfigEntry {
  /// Absolute path to the widget config file.
  pub absolute_path: PathBuf,

  /// Relative path to the widget config file.
  pub relative_path: PathBuf,

  /// Deserialized widget config.
  pub value: WidgetConfig,
}

/// Deserialized widget pack.
///
/// This is the type of the `zebar-pack.json` file.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetPackConfig {
  /// JSON schema URL to validate the widget pack file.
  #[serde(rename = "$schema")]
  schema: Option<String>,

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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum WidgetPackType {
  Local,
  Marketplace,
}

/// Deserialized widget config.
///
/// This is the type of the `zebar-widget.json` file.
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

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWidgetPackArgs {
  pub name: Option<String>,
  pub description: Option<String>,
  pub tags: Option<Vec<String>>,
  pub preview_images: Option<Vec<String>>,
  pub exclude_files: Option<String>,
  pub widget_paths: Option<Vec<PathBuf>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWidgetConfigArgs {
  pub name: String,
  pub pack_id: String,
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

  _widget_configs_change_rx: broadcast::Receiver<(String, WidgetConfig)>,

  pub widget_configs_change_tx: broadcast::Sender<(String, WidgetConfig)>,
}

impl Config {
  /// Reads the config files within the config directory.
  ///
  /// Returns a new `Config` instance.
  pub fn new(
    app_handle: &AppHandle,
    app_settings: Arc<AppSettings>,
  ) -> anyhow::Result<Self> {
    let widget_packs = Self::read_widget_packs(&app_settings)?;

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

  /// Re-evaluates widget packs within the config directory.
  pub async fn reload(&self) -> anyhow::Result<()> {
    let new_widget_packs = Self::read_widget_packs(&self.app_settings)?;

    {
      let mut widget_packs = self.widget_packs.lock().await;
      *widget_packs = new_widget_packs.clone();
    }

    self.widget_packs_change_tx.send(new_widget_packs)?;

    Ok(())
  }

  /// Finds all valid widget packs within the user's config directory.
  ///
  /// Widget packs are at the 2nd-level of the config directory
  /// (i.e. `<CONFIG_DIR>/*/*.zpack.json`).
  ///
  /// Returns a hashmap of widget pack ID's to `WidgetPack` instances.
  fn read_widget_packs(
    app_settings: &AppSettings,
  ) -> anyhow::Result<HashMap<String, WidgetPack>> {
    // Get paths to the subdirectories within the config directory.
    let dir_paths = fs::read_dir(&app_settings.config_dir)
      .with_context(|| {
        format!(
          "Failed to read config directory: {}",
          app_settings.config_dir.display()
        )
      })?
      .filter_map(|entry| Some(entry.ok()?.path()))
      .filter(|path| path.is_dir());

    // Get paths to the pack configs within the subdirectories.
    let config_paths = dir_paths
      .filter_map(|dir| fs::read_dir(dir).ok())
      .flatten()
      .filter_map(|entry| Some(entry.ok()?.path()))
      .filter(|path| path.is_file() && has_extension(path, ".zpack.json"))
      .collect::<Vec<PathBuf>>();

    let mut packs = HashMap::new();

    // Parse the found config files.
    for config_path in config_paths {
      match Self::read_widget_pack(&config_path) {
        Ok(pack) => {
          tracing::info!(
            "Found valid widget pack at: {}",
            config_path.display()
          );

          packs.insert(pack.id.clone(), pack);
        }
        Err(err) => {
          error!("{:?}", err);
        }
      }
    }

    Ok(packs)
  }

  /// Reads a widget pack from a directory. Expects the pack config
  /// file (`*.zpack.json`) to be present.
  ///
  /// Returns a `WidgetPack` instance.
  pub fn read_widget_pack(
    config_path: &Path,
  ) -> anyhow::Result<WidgetPack> {
    let (pack_config, _) = read_and_parse_json::<WidgetPackConfig>(
      config_path,
    )
    .map_err(|err| {
      anyhow::anyhow!(
        "Failed to parse widget pack at '{}': {:?}",
        config_path.display(),
        err
      )
    })?;

    let pack_dir = config_path.parent().with_context(|| {
      format!(
        "Invalid widget pack config path: {}.",
        config_path.display()
      )
    })?;

    let widget_configs =
      Self::read_widget_configs(&pack_config, pack_dir)?;

    let pack_name =
      config_path.to_unicode_string().replace(".zpack.json", "");

    let pack = WidgetPack {
      id: format!("local.{}", pack_name),
      name: pack_name,
      r#type: WidgetPackType::Local,
      config_path: config_path.to_path_buf(),
      directory_path: pack_dir.to_path_buf(),
      config: pack_config,
      widget_configs,
    };

    Ok(pack)
  }

  /// Reads widget configs for a widget pack.
  ///
  /// Returns a vector of `WidgetConfigEntry` instances.
  fn read_widget_configs(
    pack_config: &WidgetPackConfig,
    pack_dir: &Path,
  ) -> anyhow::Result<Vec<WidgetConfigEntry>> {
    let mut widget_configs = vec![];

    for widget_path in &pack_config.widget_paths {
      let absolute_path = widget_path.to_absolute(pack_dir)?;
      let relative_path = widget_path.to_relative(pack_dir)?;

      let config = read_and_parse_json::<WidgetConfig>(&absolute_path)
        .map(|(config, _)| config)
        .map_err(|err| {
          anyhow::anyhow!(
            "Failed to parse widget config at '{}': {:?}",
            absolute_path.display(),
            err
          )
        });

      match config {
        Ok(widget_config) => {
          tracing::info!(
            "Found valid widget config at: {}",
            absolute_path.display()
          );

          widget_configs.push(WidgetConfigEntry {
            absolute_path,
            relative_path,
            value: widget_config,
          });
        }
        Err(err) => {
          tracing::error!("{:?}", err);
        }
      }
    }

    Ok(widget_configs)
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

  /// Finds a local widget pack by ID.
  ///
  /// Returns an error if the widget pack is not found or is not a
  /// local pack.
  async fn find_local_widget_pack(
    &self,
    pack_id: &str,
  ) -> anyhow::Result<WidgetPack> {
    self
      .widget_pack_by_id(pack_id)
      .await
      .filter(|pack| pack.r#type == WidgetPackType::Local)
      .context(format!("Local widget pack not found: {}", pack_id))
  }

  /// Updates the widget config for the given pack and widget name.
  pub async fn update_widget_config(
    &self,
    pack_id: &str,
    widget_name: &str,
    new_config: WidgetConfig,
  ) -> anyhow::Result<WidgetConfigEntry> {
    tracing::info!("Updating widget config for {}.", widget_name);

    let config_entry = {
      let mut widget_packs = self.widget_packs.lock().await;

      let pack = widget_packs
        .get_mut(pack_id)
        .context(format!("Widget pack not found for {}.", pack_id))?;

      let config_entry = pack
        .widget_configs
        .iter_mut()
        .find(|entry| entry.value.name == widget_name)
        .context(format!(
          "Widget config not found for {}.",
          widget_name
        ))?;

      // Update the config in state.
      config_entry.value = new_config.clone();
      config_entry.clone()
    };

    // Emit the changed config.
    self
      .widget_configs_change_tx
      .send((pack_id.to_string(), new_config.clone()))?;

    // Write the updated config to file.
    fs::write(
      &config_entry.absolute_path,
      serde_json::to_string_pretty(&new_config)? + "\n",
    )?;

    Ok(config_entry)
  }

  /// Creates a new widget pack.
  pub async fn create_widget_pack(
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

    // Initialize git repository. Ignore errors (in case Git is not
    // installed).
    let _ = std::process::Command::new("git")
      .arg("init")
      .current_dir(&pack_dir)
      .output();

    let pack = Self::read_widget_pack(&pack_dir)?;

    // Add the new widget pack to state.
    {
      let mut widget_packs = self.widget_packs.lock().await;
      widget_packs.insert(pack.id.clone(), pack.clone());
    }

    self
      .widget_packs_change_tx
      .send(HashMap::from([(pack.id.clone(), pack.clone())]))?;

    Ok(pack)
  }

  /// Updates a widget pack.
  pub async fn update_widget_pack(
    &self,
    pack_id: &str,
    args: UpdateWidgetPackArgs,
  ) -> anyhow::Result<WidgetPack> {
    let mut pack = self.find_local_widget_pack(pack_id).await?;
    let pack_id = pack.id.clone();

    // Update pack config fields.
    pack.config.description =
      args.description.unwrap_or(pack.config.description);
    pack.config.tags = args.tags.unwrap_or(pack.config.tags);
    pack.config.preview_images =
      args.preview_images.unwrap_or(pack.config.preview_images);
    pack.config.exclude_files =
      args.exclude_files.unwrap_or(pack.config.exclude_files);

    // Only re-parse widget configs if widget paths have changed.
    if let Some(new_widget_paths) = args.widget_paths {
      pack.config.widget_paths = new_widget_paths;
      pack.widget_configs =
        Self::read_widget_configs(&pack.config, &pack.directory_path)?;
    }

    // Update the pack ID if a new name is provided.
    if let Some(new_name) = args.name {
      pack.id = format!("local.{}", new_name);

      let new_config_path =
        pack.directory_path.join(format!("{}.zpack.json", new_name));

      fs::rename(&pack.config_path, &new_config_path)?;
      pack.config_path = new_config_path;
      pack.name = new_name;
    }

    // Write the updated pack config to file.
    fs::write(
      &pack.config_path,
      serde_json::to_string_pretty(&pack.config)? + "\n",
    )?;

    {
      let mut widget_packs = self.widget_packs.lock().await;

      // Remove the old entry if the ID has changed.
      if pack_id != pack.id {
        widget_packs.remove(&pack_id);
      }

      widget_packs.insert(pack.id.clone(), pack.clone());
    }

    // Broadcast the change.
    self
      .widget_packs_change_tx
      .send(HashMap::from([(pack.id.clone(), pack.clone())]))?;

    Ok(pack)
  }

  /// Deletes a widget pack.
  ///
  /// Removes the pack directory and all its contents.
  pub async fn delete_widget_pack(
    &self,
    pack_id: &str,
  ) -> anyhow::Result<()> {
    let pack = self.find_local_widget_pack(pack_id).await?;

    // Remove the directory with all widget files.
    fs::remove_dir_all(&pack.directory_path)?;

    // Remove the pack from state.
    {
      let mut widget_packs = self.widget_packs.lock().await;
      widget_packs.remove(pack_id);
    }

    // TODO: Broadcast the change.
    // TODO: Kill active widget instances from the removed pack.

    Ok(())
  }

  /// Creates a new widget from a template.
  ///
  /// Adds a new entry to the pack config and copies the appropriate
  /// frontend template (e.g. React, Solid) to the widget's sub-directory.
  pub async fn create_widget_config(
    &self,
    args: CreateWidgetConfigArgs,
  ) -> anyhow::Result<WidgetConfigEntry> {
    let pack = self.find_local_widget_pack(&args.pack_id).await?;
    let widget_dir = pack.directory_path.join(&args.name);

    self.app_settings.init_template(
      TemplateResource::Widget(args.template),
      &widget_dir,
      &HashMap::from([
        ("WIDGET_NAME", args.name.clone()),
        ("ZEBAR_VERSION", VERSION_NUMBER.to_string()),
      ]),
    )?;

    // Add widget to pack config.
    let mut widget_paths = pack.config.widget_paths.clone();
    widget_paths.push(format!("{}/zebar-widget.json", args.name).into());

    self
      .update_widget_pack(
        &args.pack_id,
        UpdateWidgetPackArgs {
          widget_paths: Some(widget_paths),
          ..Default::default()
        },
      )
      .await?;

    let widget_config_path = widget_dir.join("zebar-widget.json");
    let (widget_config, _) =
      read_and_parse_json::<WidgetConfig>(&widget_config_path)?;

    Ok(WidgetConfigEntry {
      absolute_path: widget_config_path.clone(),
      relative_path: widget_config_path
        .to_relative(&pack.directory_path)?,
      value: widget_config,
    })
  }

  /// Deletes a widget from a pack.
  ///
  /// Removes the entry from the pack config and deletes the widget's
  /// sub-directory.
  pub async fn delete_widget_config(
    &self,
    pack_id: &str,
    widget_name: &str,
  ) -> anyhow::Result<()> {
    let pack = self.find_local_widget_pack(pack_id).await?;

    // Remove directory with the same name as the widget.
    let widget_dir = pack.directory_path.join(widget_name);
    let _ = fs::remove_dir_all(&widget_dir);

    // Remove widget from pack config.
    let mut widget_paths = pack.config.widget_paths.clone();
    widget_paths.retain(|path| path != &widget_dir);

    self
      .update_widget_pack(
        pack_id,
        UpdateWidgetPackArgs {
          widget_paths: Some(widget_paths),
          ..Default::default()
        },
      )
      .await?;

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
