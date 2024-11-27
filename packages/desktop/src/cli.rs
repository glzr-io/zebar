use std::{path::PathBuf, process};

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::{common::LengthValue, config::AnchorPoint};

const VERSION: &'static str = env!("VERSION_NUMBER");

#[derive(Clone, Debug, Parser)]
#[clap(author, version = VERSION, about, long_about = None)]
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
  /// Opens a widget by its config path and chosen placement.
  ///
  /// Config path is relative within the Zebar config directory, e.g.
  /// `zebar start-widget --path starter/vanilla`.
  ///
  /// Starts Zebar if it is not already running.
  StartWidget(StartWidgetArgs),

  /// Opens a widget by its config path and a preset name.
  ///
  /// Config path is relative within the Zebar config directory, e.g.
  /// `zebar start-widget-preset --path starter/vanilla --preset default`.
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

  /// Used when Zebar is launched with no arguments.
  ///
  /// If Zebar is already running, this command will no-op, otherwise it
  /// will behave as `CliCommand::Startup`.
  #[clap(hide = true)]
  Empty,
}

#[derive(Args, Clone, Debug, PartialEq)]
pub struct StartWidgetArgs {
  /// Relative file path to widget config within the Zebar config
  /// directory.
  #[clap(long = "path", value_hint = clap::ValueHint::FilePath)]
  pub config_path: PathBuf,

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
  /// Relative file path to widget config within the Zebar config
  /// directory.
  #[clap(long = "path", value_hint = clap::ValueHint::FilePath)]
  pub config_path: PathBuf,

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
