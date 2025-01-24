#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;

use crate::{
  options::CommandOptions,
  process::{ChildProcessReturn, CommandChild},
};

pub struct Shell;

impl Shell {
  pub async fn execute(
    program: &str,
    args: &[&str],
    options: CommandOptions,
  ) -> crate::Result<ChildProcessReturn> {
    let encoding = options.encoding.clone();
    let command = crate::process::Command::new(program, args, options);

    let mut command: std::process::Command = command.into();
    let output = command.output()?;

    let stdout = encoding.decode(output.stdout);
    let stderr = encoding.decode(output.stderr);

    Ok(ChildProcessReturn {
      code: output.status.code(),
      #[cfg(windows)]
      signal: None,
      #[cfg(unix)]
      signal: output.status.signal(),
      stdout,
      stderr,
    })
  }

  pub fn spawn(
    &self,
    program: &str,
    args: &[&str],
    options: CommandOptions,
  ) -> crate::Result<CommandChild> {
    crate::process::Command::new(program, args, options).spawn()
  }
}
