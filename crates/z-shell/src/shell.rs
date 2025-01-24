use std::{
  collections::HashMap,
  ffi::OsStr,
  future::Future,
  pin::Pin,
  sync::{Arc, Mutex},
};

use encoding_rs::Encoding;
use tokio::sync::mpsc;

use crate::{
  commands::{
    Buffer, ChildProcessReturn, CommandOptions, EncodingWrapper,
    JSCommandEvent, Output, ProcessId,
  },
  process::{Command, CommandChild},
};

pub struct Shell {
  children: Arc<Mutex<HashMap<ProcessId, CommandChild>>>,
}

impl Shell {
  /// Creates a new `Command` for launching the given program.
  pub fn command(&self, program: impl AsRef<OsStr>) -> Command {
    Command::new(program)
  }

  pub async fn execute(
    program: &str,
    args: &[&str],
    options: CommandOptions,
  ) -> crate::Result<ChildProcessReturn> {
    let (command, encoding) = Self::prepare_cmd(program, args, options)?;

    let mut command: std::process::Command = command.into();
    let output = command.output()?;

    let (stdout, stderr) = match encoding {
      EncodingWrapper::Text(Some(encoding)) => (
        Output::String(
          encoding.decode_with_bom_removal(&output.stdout).0.into(),
        ),
        Output::String(
          encoding.decode_with_bom_removal(&output.stderr).0.into(),
        ),
      ),
      EncodingWrapper::Text(None) => (
        Output::String(String::from_utf8(output.stdout)?),
        Output::String(String::from_utf8(output.stderr)?),
      ),
      EncodingWrapper::Raw => {
        (Output::Raw(output.stdout), Output::Raw(output.stderr))
      }
    };

    #[cfg(unix)]
    use std::os::unix::process::ExitStatusExt;

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
    on_event: mpsc::Sender<JSCommandEvent>,
    options: CommandOptions,
  ) -> crate::Result<ProcessId> {
    let (command, encoding) = Self::prepare_cmd(program, args, options)?;
    let (mut rx, child) = command.spawn()?;

    let pid = child.pid();
    self.children.lock().unwrap().insert(pid, child);
    let children = self.children.clone();

    tokio::spawn(async move {
      while let Some(event) = rx.recv().await {
        if matches!(event, crate::process::CommandEvent::Terminated(_)) {
          children.lock().unwrap().remove(&pid);
        };
        let js_event = JSCommandEvent::new(event, encoding);

        if on_event.send(js_event.clone()).await.is_err() {
          fn send<'a>(
            on_event: &'a mpsc::Sender<JSCommandEvent>,
            js_event: &'a JSCommandEvent,
          ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
            Box::pin(async move {
              tokio::time::sleep(std::time::Duration::from_millis(15))
                .await;
              if on_event.send(js_event.clone()).await.is_err() {
                send(on_event, js_event).await;
              }
            })
          }
          send(&on_event, &js_event).await;
        }
      }
    });

    Ok(pid)
  }

  #[inline(always)]
  fn prepare_cmd(
    program: &str,
    args: &[&str],
    options: CommandOptions,
  ) -> crate::Result<(crate::process::Command, EncodingWrapper)> {
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
      Option::None => EncodingWrapper::Text(None),
      Some(encoding) => match encoding.as_str() {
        "raw" => {
          command = command.set_raw_out(true);
          EncodingWrapper::Raw
        }
        _ => {
          if let Some(text_encoding) =
            Encoding::for_label(encoding.as_bytes())
          {
            EncodingWrapper::Text(Some(text_encoding))
          } else {
            return Err(crate::Error::UnknownEncoding(encoding));
          }
        }
      },
    };

    Ok((command, encoding))
  }

  pub fn stdin_write(
    &self,
    pid: u32,
    buffer: Buffer,
  ) -> crate::Result<()> {
    if let Some(child) = self.children.lock().unwrap().get_mut(&pid) {
      match buffer {
        Buffer::Text(t) => child.write(t.as_bytes())?,
        Buffer::Raw(r) => child.write(&r)?,
      }
    }

    Ok(())
  }

  pub fn kill(&self, pid: u32) -> crate::Result<()> {
    if let Some(child) = self.children.lock().unwrap().remove(&pid) {
      child.kill()?;
    }

    Ok(())
  }
}
