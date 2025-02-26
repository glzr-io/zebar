use std::{path::PathBuf, process};

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::{
  app_settings::VERSION_NUMBER, common::LengthValue, config::AnchorPoint,
};

#[derive(Clone, Debug, Parser)]
#[clap(author, version = VERSION_NUMBER, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  command: Option<CliCommand>,
}

impl Cli {
  pub fn command(&self) -> CliCommand {
    self.command.clone().unwrap_or(CliCommand::Empty)
  }
}

#[derive(Clone, Debug, PartialEq, Subcommand)]
pub enum CliCommand {
  /// Opens a widget by its name and chosen placement.
  ///
  /// Starts Zebar if it is not already running.
  StartWidget(StartWidgetArgs),

  /// Opens a widget by its name and a preset name.
  ///
  /// Starts Zebar if it is not already running.
  StartWidgetPreset(StartWidgetPresetArgs),

  /// Opens all widgets that are set to launch on startup.
  ///
  /// Starts Zebar if it is not already running.
  Startup(StartupArgs),

  /// Retrieves and outputs a specific part of the state.
  ///
  /// Requires an already running instance of Zebar.
  #[clap(subcommand)]
  Query(QueryArgs),

  /// Publishes a widget pack to the Zebar marketplace.
  Publish(PublishArgs),

  /// Used when Zebar is launched with no arguments.
  ///
  /// If Zebar is already running, this command will no-op, otherwise it
  /// will behave as `CliCommand::Startup`.
  #[clap(hide = true)]
  Empty,
}

#[derive(Args, Clone, Debug, PartialEq)]
pub struct StartWidgetArgs {
  /// Widget pack ID.
  #[clap(long)]
  pub pack_id: String,

  /// Widget name.
  #[clap(long)]
  pub widget_name: String,

  /// Anchor-point of the widget.
  #[clap(long)]
  pub anchor: AnchorPoint,

  /// Offset from the anchor-point.
  #[clap(long)]
  pub offset_x: LengthValue,

  /// Offset from the anchor-point.
  #[clap(long)]
  pub offset_y: LengthValue,

  /// Width of the widget in % or physical pixels.
  #[clap(long)]
  pub width: LengthValue,

  /// Height of the widget in % or physical pixels.
  #[clap(long)]
  pub height: LengthValue,

  /// Monitor(s) to place the widget on.
  #[clap(long)]
  pub monitor_type: MonitorType,
}

/// TODO: Add support for `Index` and `Name` types.
#[derive(Clone, Debug, PartialEq, ValueEnum)]
#[clap(rename_all = "snake_case")]
pub enum MonitorType {
  All,
  Primary,
  Secondary,
}

#[derive(Args, Clone, Debug, PartialEq)]
pub struct StartWidgetPresetArgs {
  /// Widget pack ID.
  #[clap(long)]
  pub pack_id: String,

  /// Widget name.
  #[clap(long)]
  pub widget_name: String,

  /// Name of the preset within the target widget config.
  #[clap(long = "preset")]
  pub preset_name: String,
}

#[derive(Args, Clone, Debug, PartialEq)]
pub struct StartupArgs {
  /// Absolute or relative path to the Zebar config directory.
  ///
  /// The default path is `%userprofile%/.glzr/zebar/`
  #[clap(long, value_hint = clap::ValueHint::FilePath)]
  pub config_dir: Option<PathBuf>,
}

#[derive(Clone, Debug, Parser, PartialEq)]
pub enum QueryArgs {
  /// Outputs available monitors.
  Monitors,
}

#[derive(Args, Clone, Debug, PartialEq)]
pub struct PublishArgs {
  /// Path to the pack config file.
  ///
  /// The default path is `./zebar-pack.json`.
  #[clap(long, value_hint = clap::ValueHint::FilePath, default_value = "./zebar-pack.json")]
  pub pack_config: PathBuf,

  /// API token for authentication.
  ///
  /// The widget pack gets published under the account that this token
  /// belongs to.
  #[clap(long, env = "ZEBAR_PUBLISH_TOKEN")]
  pub token: String,

  /// Version number to publish (e.g. `1.0.0`).
  ///
  /// Must be a valid semver string.
  #[clap(long)]
  pub version: String,

  /// Commit SHA associated with this release (optional).
  ///
  /// Will be shown on the Zebar marketplace page.
  #[clap(long)]
  pub commit_sha: Option<String>,

  /// Release notes for this version (optional).
  ///
  /// Will be shown on the Zebar marketplace page.
  #[clap(long)]
  pub release_notes: Option<String>,

  /// URL to the release page (optional).
  ///
  /// Will be shown on the Zebar marketplace page.
  #[clap(long)]
  pub release_url: Option<String>,

  /// API URL to send requests to.
  ///
  /// This is internally used for development and testing.
  #[clap(
    long,
    env = "GLZR_API_URL",
    default_value = "https://api.glzr.io",
    hide = true
  )]
  pub api_url: String,
}

/// Prints to stdout/stderror and exits the process.
pub fn print_and_exit(output: anyhow::Result<String>) {
  match output {
    Ok(output) => {
      print!("{}", output);
      process::exit(0);
    }
    Err(err) => {
      eprintln!("Error: {}", err);
      process::exit(1);
    }
  }
}
