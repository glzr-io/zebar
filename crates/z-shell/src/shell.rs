#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
use std::{
  collections::HashMap,
  future::Future,
  pin::Pin,
  sync::{Arc, Mutex},
};

use tokio::sync::mpsc;

use crate::{
  commands::{
    Buffer, ChildProcessReturn, CommandOptions, Encoding, JSCommandEvent,
    ProcessId,
  },
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
    let (command, encoding) = Self::prepare_cmd(program, args, options)?;

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

  #[allow(clippy::too_many_arguments)]
  pub fn spawn(
    &self,
    program: &str,
    args: &[&str],
    options: CommandOptions,
  ) -> crate::Result<CommandChild> {
    let (command, encoding) = Self::prepare_cmd(program, args, options)?;
    let mut child = command.spawn()?;

    let pid = child.pid();
    let children = self.children.clone();

    // Remove the child process from state on kill.
    child.set_termination_handler(move || {
      children.lock().unwrap().remove(&pid);
    });

    self.children.lock().unwrap().insert(pid, child);

    Ok(child)
  }

  #[inline(always)]
  fn prepare_cmd(
    program: &str,
    args: &[&str],
    options: CommandOptions,
  ) -> crate::Result<(crate::process::Command, Encoding)> {
    let mut command = crate::process::Command::new(program);
    command = command.args(args);

    if let Some(cwd) = options.cwd {
      command = command.current_dir(cwd);
    }

    if let Some(env) = options.env {
      command = command.envs(env);
    } else {
      command = command.env_clear();
    }

    let encoding = match options.encoding {
      None => Encoding::Utf8,
      Some(encoding) => encoding,
    };

    if encoding == Encoding::Raw {
      command = command.set_raw_out(true);
    }

    Ok((command, encoding))
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
