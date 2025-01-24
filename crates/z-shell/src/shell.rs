#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};

use crate::{
  commands::{Buffer, ChildProcessReturn, CommandOptions, ProcessId},
  process::CommandChild,
};

pub struct Shell {
  children: Arc<Mutex<HashMap<ProcessId, CommandChild>>>,
}

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

  pub fn stdin_write(
    &self,
    pid: ProcessId,
    buffer: Buffer,
  ) -> crate::Result<()> {
    if let Some(child) = self.children.lock().unwrap().get_mut(&pid) {
      match buffer {
        Buffer::Text(text) => child.write(text.as_bytes())?,
        Buffer::Raw(raw) => child.write(&raw)?,
      }
    }

    Ok(())
  }

  pub fn kill(&self, pid: ProcessId) -> crate::Result<()> {
    if let Some(child) = self.children.lock().unwrap().remove(&pid) {
      child.kill()?;
    }

    Ok(())
  }
}
