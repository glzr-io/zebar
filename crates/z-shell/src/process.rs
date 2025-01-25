use core::slice::memchr;
#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::{
  future::Future,
  io::{BufRead, BufReader, Write},
  process::{Command as StdCommand, Stdio},
  sync::{Arc, RwLock},
  thread::spawn,
};

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;
const NEWLINE_BYTE: u8 = b'\n';

use os_pipe::{pipe, PipeReader, PipeWriter};
use serde::{Deserialize, Serialize};
use shared_child::SharedChild;
use tokio::sync::mpsc;

use crate::{encoding::Encoding, options::CommandOptions};

pub type ProcessId = u32;

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

impl JSCommandEvent {
  pub fn new(event: CommandEvent, encoding: Encoding) -> Self {
    match event {
      CommandEvent::Terminated(payload) => {
        JSCommandEvent::Terminated(payload)
      }
      CommandEvent::Error(error) => JSCommandEvent::Error(error),
      CommandEvent::Stderr(line) => {
        JSCommandEvent::Stderr(encoding.decode(line))
      }
      CommandEvent::Stdout(line) => {
        JSCommandEvent::Stdout(encoding.decode(line))
      }
    }
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Buffer {
  Text(String),
  Raw(Vec<u8>),
}

#[derive(Debug, Serialize)]
pub struct ChildProcessReturn {
  pub code: Option<i32>,
  pub signal: Option<i32>,
  pub stdout: Buffer,
  pub stderr: Buffer,
}

/// Payload for the [`CommandEvent::Terminated`] command event.
#[derive(Debug, Clone, Serialize)]
pub struct TerminatedPayload {
  /// Exit code of the process.
  pub code: Option<i32>,

  /// If the process was terminated by a signal, represents that signal.
  pub signal: Option<i32>,
}

/// A event sent to the command callback.
#[derive(Debug, Clone)]
pub enum CommandEvent {
  /// If configured for raw output, all bytes written to stderr.
  /// Otherwise, bytes until a newline (\n) or carriage return (\r) is
  /// found.
  Stderr(Vec<u8>),

  /// If configured for raw output, all bytes written to stdout.
  /// Otherwise, bytes until a newline (\n) or carriage return (\r) is
  /// found.
  Stdout(Vec<u8>),

  /// An error happened waiting for the command to finish or converting
  /// the stdout/stderr bytes to a UTF-8 string.
  Error(String),

  /// Command process terminated.
  Terminated(TerminatedPayload),
}

/// The spawned child process.
#[derive(Debug)]
pub struct CommandChild {
  inner: Arc<SharedChild>,
  stdin_writer: PipeWriter,
  rx: mpsc::Receiver<CommandEvent>,
}

impl CommandChild {
  /// Writes to process stdin.
  pub fn write(&mut self, buf: &[u8]) -> crate::Result<()> {
    self.stdin_writer.write_all(buf)?;
    Ok(())
  }

  /// Sends a kill signal to the child.
  pub fn kill(self) -> crate::Result<()> {
    self.inner.kill()?;
    Ok(())
  }

  /// Returns the process pid.
  pub fn pid(&self) -> u32 {
    self.inner.id()
  }

  pub fn events(&mut self) -> &mut mpsc::Receiver<CommandEvent> {
    &mut self.rx
  }
}

/// The result of a process after it has terminated.
#[derive(Debug)]
pub struct ExitStatus {
  code: Option<i32>,
}

impl ExitStatus {
  /// Returns the exit code of the process, if any.
  pub fn code(&self) -> Option<i32> {
    self.code
  }

  /// Returns true if exit status is zero. Signal termination is not
  /// considered a success, and success is defined as a zero exit status.
  pub fn success(&self) -> bool {
    self.code == Some(0)
  }
}

/// The output of a finished process.
#[derive(Debug)]
pub struct Output {
  /// The status (exit code) of the process.
  pub status: ExitStatus,

  /// The data that the process wrote to stdout.
  pub stdout: Vec<u8>,

  /// The data that the process wrote to stderr.
  pub stderr: Vec<u8>,
}

/// The type to spawn commands.
#[derive(Debug)]
pub struct Command {
  inner: StdCommand,
  encoding: Encoding,
}

impl From<Command> for StdCommand {
  fn from(cmd: Command) -> StdCommand {
    cmd.inner
  }
}

impl Command {
  pub fn new(
    program: &str,
    args: &[&str],
    options: CommandOptions,
  ) -> Self {
    let mut command = StdCommand::new(program);

    if let Some(cwd) = options.cwd {
      command.current_dir(cwd);
    }

    if options.clear_env {
      command.env_clear();
    }

    command.stdout(Stdio::piped());
    command.stdin(Stdio::piped());
    command.stderr(Stdio::piped());
    command.args(args);
    command.envs(options.env);

    #[cfg(windows)]
    command.creation_flags(CREATE_NO_WINDOW);

    Self {
      inner: command,
      encoding: options.encoding,
    }
  }

  /// Spawns the command.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use shell::{process::CommandEvent, ShellExt};
  /// let shell = Shell::new();
  ///       let (mut rx, mut child) = shell.command("cargo")
  ///         .args(["tauri", "dev"])
  ///         .spawn()
  ///         .expect("Failed to spawn cargo");
  ///
  ///       let mut i = 0;
  ///       while let Some(event) = rx.recv().await {
  ///         if let CommandEvent::Stdout(line) = event {
  ///           println!("got: {}", String::from_utf8(line).unwrap());
  ///           i += 1;
  ///           if i == 4 {
  ///             child.write("message from Rust\n".as_bytes()).unwrap();
  ///             i = 0;
  ///           }
  ///         }
  ///       }
  ///     });
  ///     Ok(())
  /// });
  /// ```
  pub fn spawn(self) -> crate::Result<CommandChild> {
    let encoding = self.encoding.clone();
    let mut command: StdCommand = self.into();
    let (stdout_reader, stdout_writer) = pipe()?;
    let (stderr_reader, stderr_writer) = pipe()?;
    let (stdin_reader, stdin_writer) = pipe()?;
    command.stdout(stdout_writer);
    command.stderr(stderr_writer);
    command.stdin(stdin_reader);

    let shared_child = SharedChild::spawn(&mut command)?;
    let child = Arc::new(shared_child);
    let child_ = child.clone();
    let guard = Arc::new(RwLock::new(()));

    let (tx, rx) = mpsc::channel(1);

    spawn_pipe_reader(
      tx.clone(),
      guard.clone(),
      stdout_reader,
      CommandEvent::Stdout,
      encoding.clone(),
    );

    spawn_pipe_reader(
      tx.clone(),
      guard.clone(),
      stderr_reader,
      CommandEvent::Stderr,
      encoding,
    );

    spawn(move || {
      let status = child_.wait();
      let _lock = guard.write().unwrap();

      let event = match status {
        Ok(status) => CommandEvent::Terminated(TerminatedPayload {
          code: status.code(),
          #[cfg(windows)]
          signal: None,
          #[cfg(unix)]
          signal: status.signal(),
        }),
        Err(err) => CommandEvent::Error(err.to_string()),
      };

      let _ = block_on(async move { tx.send(event).await });
    });

    Ok(CommandChild {
      inner: child,
      stdin_writer,
      rx,
    })
  }

  /// Executes a command as a child process, waiting for it to finish and
  /// collecting its exit status. Stdin, stdout and stderr are ignored.
  ///
  /// # Examples
  /// ```rust,no_run
  /// use shell::ShellExt;
  /// tauri::Builder::default()
  ///   .setup(|app| {
  ///     let status = tauri::async_runtime::block_on(async move { app.shell().command("which").args(["ls"]).status().await.unwrap() });
  ///     println!("`which` finished with status: {:?}", status.code());
  ///     Ok(())
  ///   });
  /// ```
  pub async fn status(self) -> crate::Result<ExitStatus> {
    let mut child = self.spawn()?;

    while let Some(event) = child.events().recv().await {
      if let CommandEvent::Terminated(payload) = event {
        return Ok(ExitStatus { code: payload.code });
      }
    }

    Ok(ExitStatus { code: None })
  }

  /// Executes the command as a child process, waiting for it to finish and
  /// collecting all of its output. Stdin is ignored.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use shell::ShellExt;
  /// tauri::Builder::default()
  ///   .setup(|app| {
  ///     let output = tauri::async_runtime::block_on(async move { app.shell().command("echo").args(["TAURI"]).output().await.unwrap() });
  ///     assert!(output.status.success());
  ///     assert_eq!(String::from_utf8(output.stdout).unwrap(), "TAURI");
  ///     Ok(())
  ///   });
  /// ```
  pub async fn output(self) -> crate::Result<Output> {
    let mut child = self.spawn()?;

    let mut code = None;
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();

    while let Some(event) = child.events().recv().await {
      match event {
        CommandEvent::Terminated(payload) => {
          code = payload.code;
        }
        CommandEvent::Stdout(line) => {
          stdout.extend(line);
          stdout.push(NEWLINE_BYTE);
        }
        CommandEvent::Stderr(line) => {
          stderr.extend(line);
          stderr.push(NEWLINE_BYTE);
        }
        CommandEvent::Error(_) => {}
      }
    }

    Ok(Output {
      status: ExitStatus { code },
      stdout,
      stderr,
    })
  }
}

fn read_raw_bytes<
  F: Fn(Vec<u8>) -> CommandEvent + Send + Copy + 'static,
>(
  mut reader: BufReader<PipeReader>,
  tx: mpsc::Sender<CommandEvent>,
  wrapper: F,
) {
  loop {
    let result = reader.fill_buf();
    let tx_ = tx.clone();
    match result {
      Ok(buf) => {
        let length = buf.len();
        if length == 0 {
          break;
        }
        let _ =
          block_on(async move { tx_.send(wrapper(buf.to_vec())).await });
        reader.consume(length);
      }
      Err(err) => {
        let _ = block_on(async move {
          tx_.send(CommandEvent::Error(err.to_string())).await
        });
      }
    }
  }
}

fn read_line<F: Fn(Vec<u8>) -> CommandEvent + Send + Copy + 'static>(
  mut reader: BufReader<PipeReader>,
  tx: mpsc::Sender<CommandEvent>,
  wrapper: F,
) {
  loop {
    let mut buf = Vec::new();
    let tx_ = tx.clone();
    match read_line2(&mut reader, &mut buf) {
      Ok(n) => {
        if n == 0 {
          break;
        }
        let _ = block_on(async move { tx_.send(wrapper(buf)).await });
      }
      Err(err) => {
        let _ = block_on(async move {
          tx_.send(CommandEvent::Error(err.to_string())).await
        });
        break;
      }
    }
  }
}

fn spawn_pipe_reader<
  F: Fn(Vec<u8>) -> CommandEvent + Send + Copy + 'static,
>(
  tx: mpsc::Sender<CommandEvent>,
  guard: Arc<RwLock<()>>,
  pipe_reader: PipeReader,
  wrapper: F,
  encoding: Encoding,
) {
  spawn(move || {
    let _lock = guard.read().unwrap();
    let reader = BufReader::new(pipe_reader);

    if encoding == Encoding::Raw {
      read_raw_bytes(reader, tx, wrapper);
    } else {
      read_line(reader, tx, wrapper);
    }
  });
}

/// Runs a future to completion on runtime.
pub fn block_on<F: Future>(task: F) -> F::Output {
  let runtime = tokio::runtime::Handle::current();
  runtime.block_on(task)
}

/// Read all bytes until a newline (the `0xA` byte) or a carriage return
/// (`\r`) is reached, and append them to the provided buffer.
///
/// Adapted from <https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_line>.
fn read_line2<R: BufRead + ?Sized>(
  r: &mut R,
  buf: &mut Vec<u8>,
) -> std::io::Result<usize> {
  let mut read = 0;
  loop {
    let (done, used) = {
      let available = match r.fill_buf() {
        Ok(n) => n,
        Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {
          continue
        }

        Err(err) => return Err(err),
      };
      match memchr::memchr(b'\n', available) {
        Some(i) => {
          let end = i + 1;
          buf.extend_from_slice(&available[..end]);
          (true, end)
        }
        None => match memchr::memchr(b'\r', available) {
          Some(i) => {
            let end = i + 1;
            buf.extend_from_slice(&available[..end]);
            (true, end)
          }
          None => {
            buf.extend_from_slice(available);
            (false, available.len())
          }
        },
      }
    };
    r.consume(used);
    read += used;
    if done || used == 0 {
      return Ok(read);
    }
  }
}

// tests for the commands functions.
#[cfg(test)]
mod tests {
  #[cfg(not(windows))]
  use super::*;

  #[cfg(not(windows))]
  #[test]
  fn test_cmd_spawn_output() {
    let cmd = Command::new("cat").args(["test/test.txt"]);
    let (mut rx, _) = cmd.spawn().unwrap();

    tauri::async_runtime::block_on(async move {
      while let Some(event) = rx.recv().await {
        match event {
          CommandEvent::Terminated(payload) => {
            assert_eq!(payload.code, Some(0));
          }
          CommandEvent::Stdout(line) => {
            assert_eq!(
              String::from_utf8(line).unwrap(),
              "This is a test doc!"
            );
          }
          _ => {}
        }
      }
    });
  }

  #[cfg(not(windows))]
  #[test]
  fn test_cmd_spawn_raw_output() {
    let cmd = Command::new("cat").args(["test/test.txt"]);
    let (mut rx, _) = cmd.spawn().unwrap();

    tauri::async_runtime::block_on(async move {
      while let Some(event) = rx.recv().await {
        match event {
          CommandEvent::Terminated(payload) => {
            assert_eq!(payload.code, Some(0));
          }
          CommandEvent::Stdout(line) => {
            assert_eq!(
              String::from_utf8(line).unwrap(),
              "This is a test doc!"
            );
          }
          _ => {}
        }
      }
    });
  }

  #[cfg(not(windows))]
  #[test]
  // test the failure case
  fn test_cmd_spawn_fail() {
    let cmd = Command::new("cat").args(["test/"]);
    let (mut rx, _) = cmd.spawn().unwrap();

    tauri::async_runtime::block_on(async move {
      while let Some(event) = rx.recv().await {
        match event {
          CommandEvent::Terminated(payload) => {
            assert_eq!(payload.code, Some(1));
          }
          CommandEvent::Stderr(line) => {
            assert_eq!(
              String::from_utf8(line).unwrap(),
              "cat: test/: Is a directory\n"
            );
          }
          _ => {}
        }
      }
    });
  }

  #[cfg(not(windows))]
  #[test]
  // test the failure case (raw encoding)
  fn test_cmd_spawn_raw_fail() {
    let cmd = Command::new("cat").args(["test/"]);
    let (mut rx, _) = cmd.spawn().unwrap();

    tauri::async_runtime::block_on(async move {
      while let Some(event) = rx.recv().await {
        match event {
          CommandEvent::Terminated(payload) => {
            assert_eq!(payload.code, Some(1));
          }
          CommandEvent::Stderr(line) => {
            assert_eq!(
              String::from_utf8(line).unwrap(),
              "cat: test/: Is a directory\n"
            );
          }
          _ => {}
        }
      }
    });
  }

  #[cfg(not(windows))]
  #[test]
  fn test_cmd_output_output() {
    let cmd = Command::new("cat").args(["test/test.txt"]);
    let output = tauri::async_runtime::block_on(cmd.output()).unwrap();

    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
    assert_eq!(
      String::from_utf8(output.stdout).unwrap(),
      "This is a test doc!\n"
    );
  }

  #[cfg(not(windows))]
  #[test]
  fn test_cmd_output_output_fail() {
    let cmd = Command::new("cat").args(["test/"]);
    let output = tauri::async_runtime::block_on(cmd.output()).unwrap();

    assert_eq!(String::from_utf8(output.stdout).unwrap(), "");
    assert_eq!(
      String::from_utf8(output.stderr).unwrap(),
      "cat: test/: Is a directory\n\n"
    );
  }
}
