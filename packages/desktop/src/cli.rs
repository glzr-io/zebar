use clap::{Parser, Subcommand};

/// Parses arguments passed to `open` CLI command into a string tuple.
fn parse_open_args(input: &str) -> Result<(String, String), String> {
  let mut parts = input.split('=');

  match (parts.next(), parts.next()) {
    (Some(key), Some(value)) => Ok((key.into(), value.into())),
    _ => Err("Arguments must be of format KEY1=VAL1".into()),
  }
}

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
    #[clap(short, long, num_args = 1.., value_parser=parse_open_args)]
    args: Option<Vec<(String, String)>>,
  },
  Monitors,
}
