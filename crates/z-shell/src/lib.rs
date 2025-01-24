use std::ffi::OsStr;

use crate::process::Command;

mod commands;
pub mod error;
mod process;

pub use error::Error;
type Result<T> = std::result::Result<T, Error>;

pub struct Shell;

impl Shell {
  /// Creates a new Command for launching the given program.
  pub fn command(&self, program: impl AsRef<OsStr>) -> Command {
    Command::new(program)
  }
}
