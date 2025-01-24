use std::{
  collections::HashMap, future::Future, path::PathBuf, pin::Pin,
  string::FromUtf8Error,
};

use encoding_rs::Encoding;
use serde::{Deserialize, Serialize};

use crate::{
  process::{CommandEvent, TerminatedPayload},
  scope::ExecuteArgs,
  Shell,
};

type ChildId = u32;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "event", content = "payload")]
#[non_exhaustive]
pub enum JSCommandEvent {
  /// Stderr bytes until a newline (\n) or carriage return (\r) is found.
  Stderr(Buffer),
  /// Stdout bytes until a newline (\n) or carriage return (\r) is found.
  Stdout(Buffer),
  /// An error happened waiting for the command to finish or converting
  /// the stdout/stderr bytes to an UTF-8 string.
  Error(String),
  /// Command process terminated.
  Terminated(TerminatedPayload),
}

fn get_event_buffer(
  line: Vec<u8>,
  encoding: EncodingWrapper,
) -> Result<Buffer, FromUtf8Error> {
  match encoding {
    EncodingWrapper::Text(character_encoding) => {
      match character_encoding {
        Some(encoding) => Ok(Buffer::Text(
          encoding.decode_with_bom_removal(&line).0.into(),
        )),
        None => String::from_utf8(line).map(Buffer::Text),
      }
    }
    EncodingWrapper::Raw => Ok(Buffer::Raw(line)),
  }
}

impl JSCommandEvent {
  pub fn new(event: CommandEvent, encoding: EncodingWrapper) -> Self {
    match event {
      CommandEvent::Terminated(payload) => {
        JSCommandEvent::Terminated(payload)
      }
      CommandEvent::Error(error) => JSCommandEvent::Error(error),
      CommandEvent::Stderr(line) => get_event_buffer(line, encoding)
        .map(JSCommandEvent::Stderr)
        .unwrap_or_else(|e| JSCommandEvent::Error(e.to_string())),
      CommandEvent::Stdout(line) => get_event_buffer(line, encoding)
        .map(JSCommandEvent::Stdout)
        .unwrap_or_else(|e| JSCommandEvent::Error(e.to_string())),
    }
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Buffer {
  Text(String),
  Raw(Vec<u8>),
}

#[derive(Debug, Copy, Clone)]
pub enum EncodingWrapper {
  Raw,
  Text(Option<&'static Encoding>),
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandOptions {
  #[serde(default)]
  sidecar: bool,
  cwd: Option<PathBuf>,
  // by default we don't add any env variables to the spawned process
  // but the env is an `Option` so when it's `None` we clear the env.
  #[serde(default = "default_env")]
  env: Option<HashMap<String, String>>,
  // Character encoding for stdout/stderr
  encoding: Option<String>,
}

#[allow(clippy::unnecessary_wraps)]
fn default_env() -> Option<HashMap<String, String>> {
  Some(HashMap::default())
}

#[inline(always)]
fn prepare_cmd(
  program: String,
  args: ExecuteArgs,
  options: CommandOptions,
) -> crate::Result<(crate::process::Command, EncodingWrapper)> {
  let scope = crate::scope::ShellScope {
    scopes: command_scope
      .allows()
      .iter()
      .chain(global_scope.allows())
      .collect(),
  };

  let mut command = if options.sidecar {
    let program = PathBuf::from(program);
    let program_as_string = program.display().to_string();
    let program_no_ext_as_string =
      program.with_extension("").display().to_string();
    let configured_sidecar = window
      .config()
      .bundle
      .external_bin
      .as_ref()
      .and_then(|bins| {
        bins.iter().find(|b| {
          b == &&program_as_string || b == &&program_no_ext_as_string
        })
      })
      .cloned();
    if let Some(sidecar) = configured_sidecar {
      scope.prepare_sidecar(&program.to_string_lossy(), &sidecar, args)?
    } else {
      return Err(crate::Error::SidecarNotAllowed(program));
    }
  } else {
    match scope.prepare(&program, args) {
      Ok(cmd) => cmd,
      Err(e) => {
        #[cfg(debug_assertions)]
        eprintln!("{e}");
        return Err(crate::Error::ProgramNotAllowed(PathBuf::from(
          program,
        )));
      }
    }
  };
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

#[derive(Serialize)]
#[serde(untagged)]
enum Output {
  String(String),
  Raw(Vec<u8>),
}

#[derive(Serialize)]
pub struct ChildProcessReturn {
  code: Option<i32>,
  signal: Option<i32>,
  stdout: Output,
  stderr: Output,
}

#[allow(clippy::too_many_arguments)]
#[tauri::command]
pub async fn execute<R: Runtime>(
  window: Window<R>,
  program: String,
  args: ExecuteArgs,
  options: CommandOptions,
  command_scope: CommandScope<crate::scope::ScopeAllowedCommand>,
  global_scope: GlobalScope<crate::scope::ScopeAllowedCommand>,
) -> crate::Result<ChildProcessReturn> {
  let (command, encoding) = prepare_cmd(
    window,
    program,
    args,
    options,
    command_scope,
    global_scope,
  )?;

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
#[tauri::command]
pub fn spawn<R: Runtime>(
  window: Window<R>,
  shell: State<'_, Shell<R>>,
  program: String,
  args: ExecuteArgs,
  on_event: Channel<JSCommandEvent>,
  options: CommandOptions,
  command_scope: CommandScope<crate::scope::ScopeAllowedCommand>,
  global_scope: GlobalScope<crate::scope::ScopeAllowedCommand>,
) -> crate::Result<ChildId> {
  let (command, encoding) = prepare_cmd(
    window,
    program,
    args,
    options,
    command_scope,
    global_scope,
  )?;

  let (mut rx, child) = command.spawn()?;

  let pid = child.pid();
  shell.children.lock().unwrap().insert(pid, child);
  let children = shell.children.clone();

  tauri::async_runtime::spawn(async move {
    while let Some(event) = rx.recv().await {
      if matches!(event, crate::process::CommandEvent::Terminated(_)) {
        children.lock().unwrap().remove(&pid);
      };
      let js_event = JSCommandEvent::new(event, encoding);

      if on_event.send(js_event.clone()).is_err() {
        fn send<'a>(
          on_event: &'a Channel<JSCommandEvent>,
          js_event: &'a JSCommandEvent,
        ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
          Box::pin(async move {
            tokio::time::sleep(std::time::Duration::from_millis(15)).await;
            if on_event.send(js_event.clone()).is_err() {
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

#[tauri::command]
pub fn stdin_write<R: Runtime>(
  _window: Window<R>,
  shell: State<'_, Shell<R>>,
  pid: ChildId,
  buffer: Buffer,
) -> crate::Result<()> {
  if let Some(child) = shell.children.lock().unwrap().get_mut(&pid) {
    match buffer {
      Buffer::Text(t) => child.write(t.as_bytes())?,
      Buffer::Raw(r) => child.write(&r)?,
    }
  }
  Ok(())
}

#[tauri::command]
pub fn kill<R: Runtime>(
  _window: Window<R>,
  shell: State<'_, Shell<R>>,
  pid: ChildId,
) -> crate::Result<()> {
  if let Some(child) = shell.children.lock().unwrap().remove(&pid) {
    child.kill()?;
  }
  Ok(())
}

#[allow(deprecated)]
#[tauri::command]
pub async fn open<R: Runtime>(
  _window: Window<R>,
  shell: State<'_, Shell<R>>,
  path: String,
  with: Option<Program>,
) -> crate::Result<()> {
  shell.open(path, with)
}
