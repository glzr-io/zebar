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

use os_pipe::{pipe, PipeReader, PipeWriter};
use serde::{Deserialize, Serialize};
use shared_child::SharedChild;
use tokio::sync::mpsc;

use crate::{encoding::Encoding, options::CommandOptions};

pub type ProcessId = u32;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Buffer {
  Text(String),
  Raw(Vec<u8>),
}

impl Buffer {
  /// Joins multiple buffers of the same type into a single buffer.
  ///
  /// # Examples
  /// ```
  /// use crate::process::Buffer;
  /// let joined = Buffer::join(&[
  ///   Buffer::Text("Hello".to_string()),
  ///   Buffer::Text("World!".to_string())
  /// ]).unwrap();
  /// assert_eq!(joined, Buffer::Text("Hello\nWorld!".to_string()));
  /// ```
  pub fn join(buffers: &[Buffer]) -> crate::Result<Buffer> {
    let start = buffers.first().ok_or(crate::Error::InvalidBuffer)?;

    match start {
      Buffer::Text(_) => {
        let strings = buffers
          .iter()
          .map(|bytes| bytes.as_str().ok_or(crate::Error::InvalidBuffer))
          .collect::<Result<Vec<_>, _>>()?;

        Ok(Buffer::Text(strings.join("\n")))
      }
      Buffer::Raw(_) => Ok(Buffer::Raw(
        buffers
          .iter()
          .flat_map(|bytes| bytes.as_bytes())
          .copied()
          .collect(),
      )),
    }
  }

  /// Returns the buffer contents as a string slice if it contains text
  /// data. Returns `None` if the buffer contains raw bytes.
  pub fn as_str(&self) -> Option<&str> {
    match self {
      Buffer::Text(string) => Some(string),
      Buffer::Raw(_) => None,
    }
  }

  /// Returns the buffer contents as a byte slice.
  pub fn as_bytes(&self) -> &[u8] {
    match self {
      Buffer::Text(string) => string.as_bytes(),
      Buffer::Raw(bytes) => bytes,
    }
  }
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
  Stderr(Buffer),

  /// If configured for raw output, all bytes written to stdout.
  /// Otherwise, bytes until a newline (\n) or carriage return (\r) is
  /// found.
  Stdout(Buffer),

  /// An error happened waiting for the command to finish or converting
  /// the stdout/stderr bytes to a UTF-8 string.
  Error(String),

  /// Command process terminated.
  Terminated(TerminatedPayload),
}

/// The child process spawned by a shell command.
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

  /// Returns a channel of events from the child process.
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
  pub stdout: Buffer,

  /// The data that the process wrote to stderr.
  pub stderr: Buffer,
}

/// The type to spawn commands.
#[derive(Debug)]
pub struct Shell;

impl Shell {
  /// Executes a command as a child process, waiting for it to finish and
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
  pub async fn execute(
    program: &str,
    args: &[&str],
    options: CommandOptions,
  ) -> crate::Result<Output> {
    let (mut command, options) =
      Self::create_command(program, args, options);

    let mut child = Self::spawn_child(&mut command, options)?;

    let mut code = None;
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();

    while let Some(event) = child.events().recv().await {
      match event {
        CommandEvent::Terminated(payload) => {
          code = payload.code;
        }
        CommandEvent::Stdout(line) => {
          stdout.push(line);
        }
        CommandEvent::Stderr(line) => {
          stderr.push(line);
        }
        CommandEvent::Error(_) => {}
      }
    }

    Ok(Output {
      status: ExitStatus { code },
      stdout: Buffer::join(&stdout)?,
      stderr: Buffer::join(&stderr)?,
    })
  }

  /// Spawns the command as a child process.
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
  pub fn spawn(
    program: &str,
    args: &[&str],
    options: CommandOptions,
  ) -> crate::Result<CommandChild> {
    let (mut command, options) =
      Self::create_command(program, args, options);

    Self::spawn_child(&mut command, options)
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
  pub async fn status(
    &self,
    program: &str,
    args: &[&str],
    options: CommandOptions,
  ) -> crate::Result<ExitStatus> {
    let (mut command, options) =
      Self::create_command(program, args, options);

    let mut child = Self::spawn_child(&mut command, options)?;

    while let Some(event) = child.events().recv().await {
      if let CommandEvent::Terminated(payload) = event {
        return Ok(ExitStatus { code: payload.code });
      }
    }

    Ok(ExitStatus { code: None })
  }

  fn spawn_child(
    command: &mut StdCommand,
    options: CommandOptions,
  ) -> crate::Result<CommandChild> {
    let (stdout_reader, stdout_writer) = pipe()?;
    let (stderr_reader, stderr_writer) = pipe()?;
    let (stdin_reader, stdin_writer) = pipe()?;

    command.stdout(stdout_writer);
    command.stderr(stderr_writer);
    command.stdin(stdin_reader);

    let shared_child = SharedChild::spawn(command)?;
    let child = Arc::new(shared_child);
    let child_ = child.clone();
    let guard = Arc::new(RwLock::new(()));

    let (tx, rx) = mpsc::channel(1);

    spawn_pipe_reader(
      tx.clone(),
      guard.clone(),
      stdout_reader,
      CommandEvent::Stdout,
      options.encoding.clone(),
    );

    spawn_pipe_reader(
      tx.clone(),
      guard.clone(),
      stderr_reader,
      CommandEvent::Stderr,
      options.encoding,
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

  /// Creates a `Command` instance.
  fn create_command(
    program: &str,
    args: &[&str],
    options: CommandOptions,
  ) -> (StdCommand, CommandOptions) {
    let mut command = StdCommand::new(program);

    if let Some(cwd) = &options.cwd {
      command.current_dir(cwd);
    }

    if options.clear_env {
      command.env_clear();
    }

    command.stdout(Stdio::piped());
    command.stdin(Stdio::piped());
    command.stderr(Stdio::piped());
    command.args(args);
    command.envs(&options.env);

    #[cfg(windows)]
    command.creation_flags(CREATE_NO_WINDOW);

    (command, options)
  }
}

fn read_raw_bytes<
  F: Fn(Buffer) -> CommandEvent + Send + Copy + 'static,
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
        let buffer = Buffer::Raw(buf.to_vec());
        let _ = block_on(async move { tx_.send(wrapper(buffer)).await });
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

fn read_line<F: Fn(Buffer) -> CommandEvent + Send + Copy + 'static>(
  mut reader: BufReader<PipeReader>,
  tx: mpsc::Sender<CommandEvent>,
  wrapper: F,
  encoding: Encoding,
) {
  loop {
    let mut buf = Vec::new();
    let tx_ = tx.clone();
    match read_line2(&mut reader, &mut buf) {
      Ok(n) => {
        if n == 0 {
          break;
        }
        let buffer = encoding.decode(buf);
        let _ = block_on(async move { tx_.send(wrapper(buffer)).await });
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
  F: Fn(Buffer) -> CommandEvent + Send + Copy + 'static,
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
      read_line(reader, tx, wrapper, encoding);
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
