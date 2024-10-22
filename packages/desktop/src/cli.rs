use std::{path::PathBuf, process};

use clap::{Args, Parser, Subcommand};

use crate::config::WidgetPlacement;

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
  /// Config path is relative within the Zebar config directory (e.g.
  /// `zebar open-widget-default ./material/config.yaml`).
  ///
  /// Starts Zebar if it is not already running.
  StartWidget(StartWidgetArgs),

  /// Opens a widget by its config path and a preset name.
  ///
  /// Config path is relative within the Zebar config directory (e.g.
  /// `zebar open-widget-default ./material/config.yaml`).
  ///
  /// Starts Zebar if it is not already running.
  StartPreset(StartPresetArgs),

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

  #[clap(long, flatten)]
  pub placement: WidgetPlacement,

  /// Absolute or relative path to the Zebar config directory.
  ///
  /// The default path is `%userprofile%/.glzr/zebar/`
  #[clap(long, value_hint = clap::ValueHint::FilePath)]
  pub config_dir: Option<PathBuf>,
}

#[derive(Args, Clone, Debug, PartialEq)]
pub struct StartPresetArgs {
  /// Relative file path to widget config within the Zebar config
  /// directory.
  #[clap(long = "path", value_hint = clap::ValueHint::FilePath)]
  pub config_path: PathBuf,

  /// Name of the preset within the target widget config.
  #[clap(long = "preset")]
  pub preset_name: String,

  /// Absolute or relative path to the Zebar config directory.
  ///
  /// The default path is `%userprofile%/.glzr/zebar/`
  #[clap(long, value_hint = clap::ValueHint::FilePath)]
  pub config_dir: Option<PathBuf>,
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
