use std::path::PathBuf;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[path = "common/length_value.rs"]
pub(crate) mod length_value;
use length_value::LengthValue;

/// # Zebar settings
///
/// General settings for Zebar.
#[derive(Clone, Debug, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
#[schemars(example = serde_json::json!({
  "startupConfigs": [{
    "pack": "glzr-io.starter",
    "widget": "vanilla",
    "preset": "default"
  }]
}))]
pub struct AppSettingsValue {
  /// JSON schema URL to validate the settings file.
  #[serde(rename = "$schema")]
  pub schema: Option<String>,

  /// Widget presets to launch automatically when Zebar starts. Each entry
  /// identifies an installed pack, a widget, and one of its presets.
  pub startup_configs: Vec<StartupConfig>,
}

/// An installed widget preset that Zebar launches on startup.
#[derive(
  Clone, Debug, Deserialize, PartialEq, Serialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub struct StartupConfig {
  /// ID of the installed widget pack. Marketplace packs use the pack_id
  /// from their metadata (for example, glzr-io.starter); local packs use
  /// the name from zpack.json.
  pub pack: String,

  /// Widget name from the pack's zpack.json widgets array.
  pub widget: String,

  /// Preset name from the selected widget's presets array.
  pub preset: String,
}

/// # Zebar widget pack
///
/// Deserialized widget pack.
///
/// This is the type of the `zpack.json` file.
#[derive(Clone, Debug, Deserialize, Serialize, schemars::JsonSchema)]
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
#[derive(
  Clone, Debug, Deserialize, PartialEq, Serialize, schemars::JsonSchema,
)]
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

#[derive(
  Clone, Debug, Deserialize, Eq, PartialEq, Serialize, schemars::JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ZOrder {
  BottomMost,
  Normal,
  TopMost,
}

#[derive(
  Clone, Debug, Deserialize, Eq, PartialEq, Serialize, schemars::JsonSchema,
)]
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

#[derive(
  Clone, Debug, Deserialize, Eq, PartialEq, Serialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub struct WidgetCachingRule {
  /// URL regex pattern to match.
  pub url_regex: String,

  /// Duration to cache the matched requests for (in seconds).
  pub duration: u32,
}

#[derive(
  Clone, Debug, Deserialize, PartialEq, Serialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub struct WidgetPreset {
  #[serde(default = "default_preset_name")]
  pub name: String,

  #[serde(flatten)]
  pub placement: WidgetPlacement,
}

#[derive(
  Clone, Debug, Deserialize, PartialEq, Serialize, schemars::JsonSchema,
)]
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
  Clone,
  Copy,
  Debug,
  Deserialize,
  Eq,
  PartialEq,
  Serialize,
  ValueEnum,
  schemars::JsonSchema,
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

#[derive(
  Clone, Debug, Deserialize, Eq, PartialEq, Serialize, schemars::JsonSchema,
)]
#[serde(tag = "type", content = "match", rename_all = "snake_case")]
pub enum MonitorSelection {
  All,
  Primary,
  Secondary,
  Index(usize),
  Name(String),
}

#[derive(
  Clone,
  Debug,
  Default,
  Deserialize,
  Eq,
  PartialEq,
  Serialize,
  schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub struct WidgetPrivileges {
  /// Shell commands that the widget is allowed to run.
  pub shell_commands: Vec<ShellPrivilege>,
}

#[derive(
  Clone,
  Debug,
  Default,
  Deserialize,
  Eq,
  PartialEq,
  Serialize,
  schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub struct ShellPrivilege {
  /// Program name (if in PATH) or full path to the program.
  pub program: String,

  /// Arguments to pass to the program.
  pub args_regex: String,
}

#[derive(
  Clone,
  Debug,
  Default,
  Deserialize,
  PartialEq,
  Serialize,
  schemars::JsonSchema,
)]
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

#[derive(
  Clone,
  Copy,
  Debug,
  Deserialize,
  Eq,
  PartialEq,
  Serialize,
  schemars::JsonSchema,
)]
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

/// Helper function for setting a default value for a boolean field.
const fn default_bool<const V: bool>() -> bool {
  V
}

/// Helper function for setting the default value for a
/// `WidgetPreset::name` field.
fn default_preset_name() -> String {
  "default".into()
}
