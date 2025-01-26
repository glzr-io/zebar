#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::{
  ffi::OsStr,
  io::Write,
  process::{Command, Stdio},
  sync::{Arc, RwLock},
  thread::spawn,
};

use os_pipe::{pipe, PipeWriter};
use serde::{Deserialize, Serialize};
use shared_child::SharedChild;
use tokio::sync::mpsc;

use crate::{encoding::Encoding, options::CommandOptions, StdoutReader};

pub type ProcessId = u32;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Buffer {
  Text(String),
  Raw(Vec<u8>),
}

impl Buffer {
  /// Creates a new buffer.
  pub fn new(is_raw: bool) -> Buffer {
    if is_raw {
      Buffer::Raw(Vec::new())
    } else {
      Buffer::Text(String::new())
    }
  }

  /// Pushes a buffer of the same type into the current buffer.
  ///
  /// # Examples
  /// ```
  /// use crate::shell::Buffer;
  /// let mut buffer = Buffer::new(false);
  /// buffer.push(Buffer::Text("Hello".to_string())).unwrap();
  /// assert_eq!(buffer, Buffer::Text("Hello".to_string()));
  /// ```
  pub fn push(&mut self, buffer: Buffer) -> crate::Result<()> {
    match self {
      Buffer::Text(string) => {
        let incoming_string =
          buffer.as_str().ok_or(crate::Error::InvalidBuffer)?;

        string.push_str(incoming_string);
      }
      Buffer::Raw(bytes) => bytes.extend_from_slice(buffer.as_bytes()),
    }

    Ok(())
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

/// A event sent to the command callback.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
  Terminated(ExitStatus),
}

/// The child process spawned by a shell command.
#[derive(Debug)]
pub struct ChildProcess {
  inner: Arc<SharedChild>,
  stdin_writer: PipeWriter,
  rx: mpsc::Receiver<CommandEvent>,
}

impl ChildProcess {
  /// Writes to the child process' stdin.
  pub fn write(&mut self, buffer: &[u8]) -> crate::Result<()> {
    self.stdin_writer.write_all(buffer)?;
    Ok(())
  }

  /// Sends a kill signal to the child process.
  pub fn kill(self) -> crate::Result<()> {
    self.inner.kill()?;
    Ok(())
  }

  /// Returns the child process' pid.
  pub fn pid(&self) -> u32 {
    self.inner.id()
  }

  /// Returns a channel of events from the child process.
  pub fn events(&mut self) -> &mut mpsc::Receiver<CommandEvent> {
    &mut self.rx
  }
}

/// The result of a process after it has terminated.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExitStatus {
  /// Exit code of the process.
  pub code: Option<i32>,

  /// If the process was terminated by a signal, represents that signal.
  pub signal: Option<i32>,
}

impl ExitStatus {
  /// Returns true if exit status is zero. Signal termination is not
  /// considered a success, and success is defined as a zero exit status.
  pub fn success(&self) -> bool {
    self.code == Some(0)
  }
}

/// The output of a finished process.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ShellExecuteOutput {
  /// The exit code and termination signal of the process.
  #[serde(flatten)]
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
  /// use shell::{CommandOptions, Shell};
  /// let output =
  ///     Shell::execute("echo", &["Hello!"], &CommandOptions::default())
  ///       .await
  ///       .unwrap();
  /// assert!(output.status.success());
  /// assert_eq!(output.stdout.as_str().unwrap(), "Hello!");
  /// ```
  pub async fn execute<I, S>(
    program: &str,
    args: I,
    options: &CommandOptions,
  ) -> crate::Result<ShellExecuteOutput>
  where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
  {
    let mut child = Self::spawn(program, args, options)?;

    let mut status = ExitStatus::default();
    let mut stdout = Buffer::new(options.encoding == Encoding::Raw);
    let mut stderr = Buffer::new(options.encoding == Encoding::Raw);

    while let Some(event) = child.events().recv().await {
      match event {
        CommandEvent::Terminated(exit_status) => {
          status = exit_status;
        }
        CommandEvent::Stdout(line) => {
          stdout.push(line)?;
        }
        CommandEvent::Stderr(line) => {
          stderr.push(line)?;
        }
        CommandEvent::Error(_) => {}
      }
    }

    Ok(ShellExecuteOutput {
      status,
      stdout,
      stderr,
    })
  }

  /// Executes a command as a child process, waiting for it to finish and
  /// collecting its exit status. Stdin, stdout and stderr are ignored.
  ///
  /// # Examples
  /// ```rust,no_run
  /// use shell::{CommandOptions, Shell};
  /// let status =
  ///     Shell::status("echo", ["Hello!"], CommandOptions::default())
  ///       .await
  ///       .unwrap();
  /// assert!(status.success());
  /// ```
  pub async fn status<I, S>(
    &self,
    program: &str,
    args: I,
    options: &CommandOptions,
  ) -> crate::Result<ExitStatus>
  where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
  {
    let mut child = Self::spawn(program, args, options)?;

    while let Some(event) = child.events().recv().await {
      if let CommandEvent::Terminated(status) = event {
        return Ok(status);
      }
    }

    Ok(ExitStatus::default())
  }

  /// Spawns the command as a child process.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use shell::{CommandEvent, Shell};
  /// let child = Shell::spawn("yes", [], CommandOptions::default())
  ///   .expect("Failed to spawn yes.");
  ///
  /// while let Some(event) = child.events().recv().await {
  ///   if let CommandEvent::Stdout(buffer) = event {
  ///     println!("stdout: {}", buffer.as_str().unwrap());
  ///   }
  /// }
  /// ```
  pub fn spawn<I, S>(
    program: &str,
    args: I,
    options: &CommandOptions,
  ) -> crate::Result<ChildProcess>
  where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
  {
    let mut command = Self::create_command(program, args, options);
    Self::spawn_child(&mut command, options)
  }

  /// Spawns the command as a child process.
  fn spawn_child(
    command: &mut Command,
    options: &CommandOptions,
  ) -> crate::Result<ChildProcess> {
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

    Self::spawn_pipe_reader(
      tx.clone(),
      guard.clone(),
      stdout_reader,
      CommandEvent::Stdout,
      options.encoding.clone(),
    );

    Self::spawn_pipe_reader(
      tx.clone(),
      guard.clone(),
      stderr_reader,
      CommandEvent::Stderr,
      options.encoding.clone(),
    );

    spawn(move || {
      let status = child_.wait();
      let _lock = guard.write().unwrap();

      let event = match status {
        Ok(status) => CommandEvent::Terminated(ExitStatus {
          code: status.code(),
          #[cfg(windows)]
          signal: None,
          #[cfg(unix)]
          signal: status.signal(),
        }),
        Err(err) => CommandEvent::Error(err.to_string()),
      };

      let _ = tx.blocking_send(event);
    });

    Ok(ChildProcess {
      inner: child,
      stdin_writer,
      rx,
    })
  }

  /// Creates a `Command` instance.
  fn create_command<I, S>(
    program: &str,
    args: I,
    options: &CommandOptions,
  ) -> Command
  where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
  {
    let mut command = Command::new(program);

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

    command
  }

  /// Spawns a thread to read from stdout/stderr and emit the output
  /// through a channel.
  fn spawn_pipe_reader<F>(
    tx: mpsc::Sender<CommandEvent>,
    guard: Arc<RwLock<()>>,
    pipe: os_pipe::PipeReader,
    wrapper: F,
    encoding: Encoding,
  ) where
    F: Fn(Buffer) -> CommandEvent + Send + Copy + 'static,
  {
    spawn(move || {
      let _lock = guard.read().unwrap();
      let mut reader = StdoutReader::new(pipe, encoding);

      while let Ok(Some(buffer)) = reader.read_next() {
        if tx.blocking_send(wrapper(buffer)).is_err() {
          break;
        }
      }
    });
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_echo_command() {
    let output = Shell::execute(
      if cfg!(windows) { "cmd" } else { "sh" },
      &[if cfg!(windows) { "/C" } else { "-c" }, "echo hello world"],
      &CommandOptions::default(),
    )
    .await
    .unwrap();

    assert!(output.status.success());
    assert!(output.stderr.as_str().unwrap().is_empty());
    assert!(output.stdout.as_str().unwrap().contains("hello world"));
  }

  #[tokio::test]
  async fn test_command_failure() {
    let output = Shell::execute(
      if cfg!(windows) { "cmd" } else { "sh" },
      &[
        if cfg!(windows) { "/C" } else { "-c" },
        "nonexistent_command",
      ],
      &CommandOptions::default(),
    )
    .await
    .unwrap();

    assert!(!output.status.success());
    assert!(!output.stderr.as_str().unwrap().is_empty());
  }

  #[tokio::test]
  async fn test_raw_output() {
    let options = CommandOptions {
      encoding: Encoding::Raw,
      ..Default::default()
    };

    let mut child = Shell::spawn(
      if cfg!(windows) { "cmd" } else { "sh" },
      [if cfg!(windows) { "/C" } else { "-c" }, "echo test"],
      &options,
    )
    .unwrap();

    let mut saw_stdout = false;
    while let Some(event) = child.events().recv().await {
      match event {
        CommandEvent::Stdout(Buffer::Raw(bytes)) => {
          assert!(!bytes.is_empty());
          saw_stdout = true;
        }
        CommandEvent::Terminated(status) => {
          assert!(status.success());
        }
        _ => {}
      }
    }
    assert!(saw_stdout);
  }
}
