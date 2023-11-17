use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: CliCommand,
}

#[derive(Subcommand, Debug)]
pub enum CliCommand {
  /// Open a window by its id (eg. `zebar open window/bar`).
  Open {
    /// ID of the window to open (eg. `window/bar`).
    window_id: String,

    /// Arguments to pass to the window.
    ///
    /// These become available via the `self` provider.
    #[clap(short, long, num_args = 1..)]
    args: Option<Vec<String>>,
  },
  Monitors,
}
