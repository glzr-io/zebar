use std::{path::PathBuf, process};

use clap::{Args, Parser, Subcommand};

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
  /// Open a window by its config path.
  ///
  /// Config path is relative within the Zebar config directory (e.g.
  /// `zebar open ./material/config.yaml`).
  ///
  /// Starts Zebar if it is not already running.
  Open(OpenWindowArgs),

  /// Open all default windows.
  ///
  /// Starts Zebar if it is not already running.
  OpenAll(OpenAllWindowsArgs),

  /// Output available monitors.
  Monitors(OutputMonitorsArgs),

  /// Used when Zebar is launched with no arguments.
  ///
  /// If Zebar is already running, this command will no-op, otherwise it
  /// will start Zebar and open all default windows.
  #[clap(hide = true)]
  Empty,
}

#[derive(Args, Clone, Debug, PartialEq)]
pub struct OpenWindowArgs {
  /// Relative file path to window config within the Zebar config
  /// directory.
  pub config_path: String,

  /// Absolute path to the Zebar config directory.
  ///
  /// The default path is `%userprofile%/.glzr/zebar/`
  #[clap(long, value_hint = clap::ValueHint::FilePath)]
  pub config_dir: Option<PathBuf>,
}

#[derive(Args, Clone, Debug, PartialEq)]
pub struct OpenAllWindowsArgs {
  /// Absolute path to the Zebar config directory.
  ///
  /// The default path is `%userprofile%/.glzr/zebar/`
  #[clap(long, value_hint = clap::ValueHint::FilePath)]
  pub config_dir: Option<PathBuf>,
}

#[derive(Args, Clone, Debug, PartialEq)]
pub struct OutputMonitorsArgs {
  /// Use ASCII NUL character (character code 0) instead of newlines
  /// for delimiting monitors.
  ///
  /// Useful for piping to `xargs -0`.
  #[clap(short, long)]
  pub print0: bool,
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
