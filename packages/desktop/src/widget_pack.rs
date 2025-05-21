use std::{
  collections::HashMap,
  fs::{self},
  path::{Path, PathBuf},
  sync::Arc,
};

use anyhow::Context;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, Mutex};

use crate::{
  app_settings::{AppSettings, VERSION_NUMBER},
  common::{read_and_parse_json, LengthValue, PathExt},
  marketplace_installer::{MarketplaceInstaller, MarketplacePackMetadata},
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

  /// Metadata for the pack (if it's a marketplace pack).
  pub metadata: Option<MarketplacePackMetadata>,

  /// Path to the pack config file.
  pub config_path: PathBuf,

  /// Path to the directory containing the pack config.
  ///
  /// This is the parent directory of `config_path`.
  pub directory_path: PathBuf,
}

impl WidgetPack {
  /// Returns a list of file patterns to include for all widgets in the
  /// pack.
  pub fn include_files(&self) -> Vec<String> {
    self
      .config
      .widgets
      .iter()
      .flat_map(|widget| widget.include_files.clone())
      .collect()
  }
}

/// Deserialized widget pack.
///
/// This is the type of the `zpack.json` file.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetPackConfig {
  /// JSON schema URL to validate the widget pack file.
  #[serde(rename = "$schema")]
  pub schema: Option<String>,

  /// Name of the pack.
  pub name: String,

  /// Version of the pack.
  pub version: String,

  /// Description of the pack.
  #[serde(default)]
  pub description: String,

  /// Tags of the pack.
  #[serde(default)]
  pub tags: Vec<String>,

  /// Preview images of the pack.
  #[serde(default)]
  pub preview_images: Vec<String>,

  /// URL of the repository containing the pack.
  #[serde(default)]
  pub repository_url: String,

  /// Widgets in the pack.
  #[serde(default)]
  pub widgets: Vec<WidgetConfig>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum WidgetPackType {
  Custom,
  Marketplace,
}

/// Deserialized widget config.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetConfig {
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

  /// Files to include as part of the widget.
  #[serde(default)]
  pub include_files: Vec<String>,

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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ZOrder {
  BottomMost,
  Normal,
  TopMost,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetCachingRule {
  /// URL regex pattern to match.
  pub url_regex: String,

  /// Duration to cache the matched requests for (in seconds).
  pub duration: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
  Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ValueEnum,
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type", content = "match", rename_all = "snake_case")]
pub enum MonitorSelection {
  All,
  Primary,
  Secondary,
  Index(usize),
  Name(String),
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetPrivileges {
  /// Shell commands that the widget is allowed to run.
  pub shell_commands: Vec<ShellPrivilege>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellPrivilege {
  /// Program name (if in PATH) or full path to the program.
  pub program: String,

  /// Arguments to pass to the program.
  pub args_regex: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
  pub version: String,
  pub description: String,
  pub tags: Vec<String>,
  pub repository_url: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWidgetPackArgs {
  pub name: Option<String>,
  pub version: Option<String>,
  pub description: Option<String>,
  pub tags: Option<Vec<String>>,
  pub preview_images: Option<Vec<String>>,
  pub repository_url: Option<String>,
  pub widgets: Option<Vec<WidgetConfig>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWidgetConfigArgs {
  pub name: String,
  pub pack_id: String,
  pub template: FrontendTemplate,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FrontendTemplate {
  ReactBuildless,
  SolidTypescript,
}

#[derive(Debug)]
pub struct WidgetPackManager {
  /// Reference to `AppSettings`.
  app_settings: Arc<AppSettings>,

  /// Reference to `MarketplaceInstaller`.
  marketplace_installer: Arc<MarketplaceInstaller>,

  /// Map of widget packs by their ID's.
  pub widget_packs: Arc<Mutex<HashMap<String, WidgetPack>>>,

  _widget_packs_change_rx:
    broadcast::Receiver<HashMap<String, WidgetPack>>,

  pub widget_packs_change_tx:
    broadcast::Sender<HashMap<String, WidgetPack>>,

  _widget_configs_change_rx: broadcast::Receiver<(String, WidgetConfig)>,

  pub widget_configs_change_tx: broadcast::Sender<(String, WidgetConfig)>,
}

impl WidgetPackManager {
  /// Reads the pack config files within the config directory.
  ///
  /// Returns a new `WidgetPackManager` instance.
  pub fn new(
    app_settings: Arc<AppSettings>,
    marketplace_installer: Arc<MarketplaceInstaller>,
  ) -> anyhow::Result<Self> {
    let widget_packs =
      Self::read_widget_packs(&app_settings, &marketplace_installer)?;

    let (widget_packs_change_tx, _widget_packs_change_rx) =
      broadcast::channel(16);

    let (widget_configs_change_tx, _widget_configs_change_rx) =
      broadcast::channel(16);

    Ok(Self {
      app_settings,
      marketplace_installer,
      widget_packs: Arc::new(Mutex::new(widget_packs)),
      _widget_packs_change_rx,
      widget_packs_change_tx,
      _widget_configs_change_rx,
      widget_configs_change_tx,
    })
  }

  /// Re-evaluates widget packs within the config directory.
  pub async fn reload(&self) -> anyhow::Result<()> {
    let new_widget_packs = Self::read_widget_packs(
      &self.app_settings,
      &self.marketplace_installer,
    )?;

    {
      let mut widget_packs = self.widget_packs.lock().await;
      *widget_packs = new_widget_packs.clone();
    }

    self.widget_packs_change_tx.send(new_widget_packs)?;

    Ok(())
  }

  /// Reads all widget packs from:
  ///  - The user's config directory.
  ///  - The marketplace directory.
  ///
  /// Returns a hashmap of widget pack ID's to `WidgetPack` instances.
  fn read_widget_packs(
    app_settings: &AppSettings,
    marketplace_installer: &MarketplaceInstaller,
  ) -> anyhow::Result<HashMap<String, WidgetPack>> {
    let mut packs = HashMap::new();

    packs
      .extend(Self::read_custom_widget_packs(&app_settings.config_dir)?);

    for metadata in marketplace_installer.installed_packs_metadata()? {
      if let Ok(pack) = Self::read_widget_pack(
        &app_settings
          .marketplace_pack_download_dir(
            &metadata.pack_id,
            &metadata.version,
          )
          .join("zpack.json"),
        Some(&metadata),
      ) {
        packs.insert(metadata.pack_id.clone(), pack);
      } else {
        tracing::warn!(
          "Skipping marketplace pack at '{}' because it is invalid.",
          metadata.pack_id
        );
      }
    }

    Ok(packs)
  }

  /// Finds all valid widget packs within the user's config directory.
  ///
  /// Widget packs are at the 2nd-level of the config directory
  /// (i.e. `<CONFIG_DIR>/*/zpack.json`).
  ///
  /// Returns a hashmap of widget pack ID's to `WidgetPack` instances.
  fn read_custom_widget_packs(
    config_dir: &Path,
  ) -> anyhow::Result<HashMap<String, WidgetPack>> {
    // Get paths to the subdirectories within the config directory.
    let pack_dirs = fs::read_dir(config_dir)
      .with_context(|| {
        format!(
          "Failed to read config directory: {}",
          config_dir.display()
        )
      })?
      .filter_map(|entry| Some(entry.ok()?.path()))
      .filter(|path| path.is_dir());

    let mut packs = HashMap::new();

    // Parse the found config files.
    for pack_dir in pack_dirs {
      let pack_config_path = pack_dir.join("zpack.json");

      if !pack_config_path.exists() {
        warn!(
          "Skipping subdirectory at '{}' because it has no `zpack.json` file.",
          pack_dir.display()
        );

        continue;
      }

      match Self::read_widget_pack(&pack_config_path, None) {
        Ok(pack) => {
          tracing::info!(
            "Found valid widget pack at: {}",
            pack_dir.display()
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
  /// file (`zpack.json`) to be present.
  ///
  /// Returns a `WidgetPack` instance.
  pub fn read_widget_pack(
    config_path: &Path,
    metadata: Option<&MarketplacePackMetadata>,
  ) -> anyhow::Result<WidgetPack> {
    let pack_config = read_and_parse_json::<WidgetPackConfig>(config_path)
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

    let pack = WidgetPack {
      id: match metadata {
        Some(metadata) => metadata.pack_id.clone(),
        None => pack_config.name.to_string(),
      },
      r#type: match metadata {
        Some(_) => WidgetPackType::Marketplace,
        None => WidgetPackType::Custom,
      },
      config_path: config_path.canonicalize_pretty()?,
      directory_path: pack_dir.canonicalize_pretty()?,
      config: pack_config,
      metadata: metadata.cloned(),
    };

    Ok(pack)
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

  /// Finds a custom widget pack by ID.
  ///
  /// Returns an error if the widget pack is not found or is not a
  /// custom pack.
  async fn find_custom_widget_pack(
    &self,
    pack_id: &str,
  ) -> anyhow::Result<WidgetPack> {
    self
      .widget_pack_by_id(pack_id)
      .await
      .filter(|pack| pack.r#type == WidgetPackType::Custom)
      .context(format!("Custom widget pack not found: {}", pack_id))
  }

  /// Updates the widget config for the given pack and widget name.
  pub async fn update_widget_config(
    &self,
    pack_id: &str,
    widget_name: &str,
    new_config: WidgetConfig,
  ) -> anyhow::Result<WidgetConfig> {
    tracing::info!("Updating widget config for {}.", widget_name);

    let pack = self.find_custom_widget_pack(pack_id).await?;

    let mut widgets = pack.config.widgets.clone();
    let widget_index = widgets
      .iter()
      .position(|w| w.name == widget_name)
      .context(format!("Widget config not found for {}.", widget_name))?;

    widgets[widget_index] = new_config.clone();

    // Update the pack config to persist changes to disk.
    self
      .update_widget_pack(
        pack_id,
        UpdateWidgetPackArgs {
          widgets: Some(widgets),
          ..Default::default()
        },
      )
      .await?;

    // Emit the changed config.
    self
      .widget_configs_change_tx
      .send((pack_id.to_string(), new_config.clone()))?;

    Ok(new_config)
  }

  /// Creates a new widget pack.
  pub async fn create_widget_pack(
    &self,
    args: CreateWidgetPackArgs,
  ) -> anyhow::Result<WidgetPack> {
    let pack_dir = self.app_settings.config_dir.join(&args.name);

    let mut context = tera::Context::new();
    context.insert("PACK_NAME", &args.name);
    context.insert("PACK_VERSION", &args.version);
    context.insert("PACK_DESCRIPTION", &args.description);
    context.insert("PACK_TAGS", &args.tags);
    context.insert("REPOSITORY_URL", &args.repository_url);
    context.insert("ZEBAR_VERSION", &VERSION_NUMBER.to_string());

    self.app_settings.init_template(
      Path::new("pack-template"),
      &pack_dir,
      &context,
    )?;

    // Initialize git repository. Ignore errors (in case Git is not
    // installed).
    let _ = std::process::Command::new("git")
      .arg("init")
      .current_dir(&pack_dir)
      .output();

    let pack = Self::read_widget_pack(&pack_dir.join("zpack.json"), None)?;

    // Add the new widget pack to state.
    {
      let mut widget_packs = self.widget_packs.lock().await;
      widget_packs.insert(pack.id.clone(), pack.clone());

      // Broadcast the change.
      let _ = self.widget_packs_change_tx.send(widget_packs.clone());
    }

    Ok(pack)
  }

  /// Updates a widget pack.
  pub async fn update_widget_pack(
    &self,
    pack_id: &str,
    args: UpdateWidgetPackArgs,
  ) -> anyhow::Result<WidgetPack> {
    let mut pack = self.find_custom_widget_pack(pack_id).await?;
    let pack_id = pack.id.clone();

    // Update pack config fields.
    pack.config.name = args.name.clone().unwrap_or(pack.config.name);
    pack.config.version = args.version.unwrap_or(pack.config.version);
    pack.config.description =
      args.description.unwrap_or(pack.config.description);
    pack.config.tags = args.tags.unwrap_or(pack.config.tags);
    pack.config.preview_images =
      args.preview_images.unwrap_or(pack.config.preview_images);
    pack.config.repository_url =
      args.repository_url.unwrap_or(pack.config.repository_url);
    pack.config.widgets = args.widgets.unwrap_or(pack.config.widgets);

    // Write the updated pack config to file.
    fs::write(
      &pack.config_path,
      serde_json::to_string_pretty(&pack.config)? + "\n",
    )?;

    let mut widget_packs = self.widget_packs.lock().await;

    // Update the pack ID and remove the old entry if a new name is
    // provided.
    if let Some(new_name) = args.name {
      pack.id = new_name;
      widget_packs.remove(&pack_id);
    }

    // Broadcast the change.
    widget_packs.insert(pack.id.clone(), pack.clone());
    let _ = self.widget_packs_change_tx.send(widget_packs.clone());

    Ok(pack)
  }

  /// Deletes a widget pack.
  ///
  /// Removes the pack directory and all its contents.
  pub async fn delete_widget_pack(
    &self,
    pack_id: &str,
  ) -> anyhow::Result<()> {
    let pack = self
      .widget_pack_by_id(pack_id)
      .await
      .with_context(|| format!("Widget pack not found: {}", pack_id))?;

    match pack.r#type {
      WidgetPackType::Custom => {
        // Remove the directory with all widget files.
        fs::remove_dir_all(&pack.directory_path)?;
      }
      WidgetPackType::Marketplace => {
        self.marketplace_installer.delete_metadata(pack_id)?;
      }
    }

    // Remove the pack from state.
    {
      let mut widget_packs = self.widget_packs.lock().await;
      widget_packs.remove(pack_id);

      // Broadcast the change.
      let _ = self.widget_packs_change_tx.send(widget_packs.clone());
    }

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
  ) -> anyhow::Result<WidgetConfig> {
    let pack = self.find_custom_widget_pack(&args.pack_id).await?;
    let widget_dir = pack.directory_path.join(&args.name);

    let template_path = match args.template {
      FrontendTemplate::ReactBuildless => {
        "widget-templates/react-buildless"
      }
      FrontendTemplate::SolidTypescript => "widget-templates/solid-ts",
    };

    let mut context = tera::Context::new();
    context.insert("WIDGET_NAME", &args.name);
    context.insert("ZEBAR_VERSION", &VERSION_NUMBER.to_string());

    self.app_settings.init_template(
      &Path::new(template_path),
      &widget_dir,
      &context,
    )?;

    let widget_config = WidgetConfig {
      name: args.name.clone(),
      html_path: match args.template {
        FrontendTemplate::ReactBuildless => {
          format!("{}/index.html", args.name).into()
        }
        FrontendTemplate::SolidTypescript => {
          format!("{}/dist/index.html", args.name).into()
        }
      },
      z_order: ZOrder::Normal,
      shown_in_taskbar: false,
      focused: false,
      resizable: false,
      transparent: false,
      include_files: match args.template {
        FrontendTemplate::ReactBuildless => {
          vec![format!("{}/**", args.name)]
        }
        FrontendTemplate::SolidTypescript => {
          vec![format!("{}/dist/**", args.name)]
        }
      },
      caching: WidgetCaching::default(),
      privileges: WidgetPrivileges::default(),
      presets: vec![WidgetPreset {
        name: "default".to_string(),
        placement: WidgetPlacement {
          anchor: AnchorPoint::TopLeft,
          offset_x: "0px".parse()?,
          offset_y: "0px".parse()?,
          width: "100%".parse()?,
          height: "40px".parse()?,
          monitor_selection: MonitorSelection::All,
          dock_to_edge: DockConfig::default(),
        },
      }],
    };

    // Add widget to pack config.
    let mut widgets = pack.config.widgets.clone();
    widgets.push(widget_config.clone());

    self
      .update_widget_pack(
        &args.pack_id,
        UpdateWidgetPackArgs {
          widgets: Some(widgets),
          ..Default::default()
        },
      )
      .await?;

    Ok(widget_config)
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
    let pack = self.find_custom_widget_pack(pack_id).await?;

    // Remove widget from pack config.
    let mut widgets = pack.config.widgets.clone();
    widgets.retain(|widget| widget.name != widget_name);

    self
      .update_widget_pack(
        pack_id,
        UpdateWidgetPackArgs {
          widgets: Some(widgets),
          ..Default::default()
        },
      )
      .await?;

    Ok(())
  }

  /// Registers a newly installed widget pack.
  pub async fn register_widget_pack(&self, pack: WidgetPack) {
    let mut widget_packs = self.widget_packs.lock().await;
    widget_packs.insert(pack.id.clone(), pack);

    // Broadcast the change.
    let _ = self.widget_packs_change_tx.send(widget_packs.clone());
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
