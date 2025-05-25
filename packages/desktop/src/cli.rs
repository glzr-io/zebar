use std::{path::PathBuf, process};

use clap::{Args, Parser, Subcommand, ValueEnum};
use tracing::Level;

use crate::{
  app_settings::VERSION_NUMBER, common::LengthValue,
  widget_pack::AnchorPoint,
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
  #[clap(long = "pack")]
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
  #[clap(long = "pack")]
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

  /// Logging verbosity.
  #[clap(flatten)]
  pub verbosity: Verbosity,
}

/// Verbosity flags to be used with `#[command(flatten)]`.
#[derive(Args, Clone, Debug, PartialEq)]
#[clap(about = None, long_about = None)]
pub struct Verbosity {
  /// Enables verbose logging.
  #[clap(short = 'v', long, action)]
  verbose: bool,

  /// Disables logging.
  #[clap(short = 'q', long, action, conflicts_with = "verbose")]
  quiet: bool,

  /// Set log level directly (overrides verbose/quiet flags).
  ///
  /// Can also be set via `LOG_LEVEL` environment variable.
  #[clap(long, env = "LOG_LEVEL", value_enum)]
  log_level: Option<LogLevel>,
}

impl Verbosity {
  /// Gets the log level based on the verbosity flags.
  #[must_use]
  pub fn level(&self) -> Level {
    // If log_level is explicitly set (via CLI or env), use that.
    if let Some(level) = &self.log_level {
      return level.clone().into();
    }

    // Otherwise fall back to verbose/quiet flags.
    match (self.verbose, self.quiet) {
      (true, _) => Level::DEBUG,
      (_, true) => Level::ERROR,
      _ => Level::INFO,
    }
  }
}

#[derive(Clone, Debug, PartialEq, ValueEnum)]
pub enum LogLevel {
  Debug,
  Info,
  Warn,
  Error,
}

impl From<LogLevel> for Level {
  fn from(log_level: LogLevel) -> Self {
    match log_level {
      LogLevel::Debug => Level::DEBUG,
      LogLevel::Info => Level::INFO,
      LogLevel::Warn => Level::WARN,
      LogLevel::Error => Level::ERROR,
    }
  }
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
  /// The default path is `./zpack.json`.
  #[clap(long, value_hint = clap::ValueHint::FilePath, default_value = "./zpack.json")]
  pub pack_config: PathBuf,

  /// API token for authentication.
  ///
  /// The widget pack gets published under the account that this token
  /// belongs to.
  #[clap(long, env = "ZEBAR_PUBLISH_TOKEN")]
  pub token: String,

  /// Override the version number (e.g. `1.0.0`) in the pack config
  /// (optional).
  ///
  /// Must be a valid semver string.
  #[clap(long)]
  pub version_override: Option<String>,

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
  #[clap(long, default_value = "https://api.glzr.io", hide = true)]
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
