use core::slice::memchr;
#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::{
  io::{BufRead, BufReader, Write},
  process::{Command, Stdio},
  sync::{Arc, RwLock},
  thread::spawn,
};

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

use os_pipe::{pipe, PipeReader, PipeWriter};
use serde::Serialize;
use shared_child::SharedChild;
use tokio::sync::mpsc;

use crate::{encoding::Encoding, options::CommandOptions};

pub type ProcessId = u32;

#[derive(Clone, Debug, Serialize)]
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
#[derive(Clone, Debug, Serialize)]
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
pub struct CommandChild {
  inner: Arc<SharedChild>,
  stdin_writer: PipeWriter,
  rx: mpsc::Receiver<CommandEvent>,
}

impl CommandChild {
  /// Writes to the child process' stdin.
  pub fn write(&mut self, buf: &[u8]) -> crate::Result<()> {
    self.stdin_writer.write_all(buf)?;
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
#[derive(Clone, Debug, Default, Serialize)]
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
#[derive(Clone, Debug, Serialize)]
pub struct Output {
  /// The exit code and termination signal of the process.
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
  pub async fn execute(
    program: &str,
    args: &[&str],
    options: &CommandOptions,
  ) -> crate::Result<Output> {
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

    Ok(Output {
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
  pub async fn status(
    &self,
    program: &str,
    args: &[&str],
    options: &CommandOptions,
  ) -> crate::Result<ExitStatus> {
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
  pub fn spawn(
    program: &str,
    args: &[&str],
    options: &CommandOptions,
  ) -> crate::Result<CommandChild> {
    let mut command = Self::create_command(program, args, options);
    Self::spawn_child(&mut command, options)
  }

  /// Spawns the command as a child process.
  fn spawn_child(
    command: &mut Command,
    options: &CommandOptions,
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
    options: &CommandOptions,
  ) -> Command {
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
        let _ = tx_.blocking_send(wrapper(buffer));
        reader.consume(length);
      }
      Err(err) => {
        let _ = tx_.blocking_send(CommandEvent::Error(err.to_string()));
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
        let _ = tx_.blocking_send(wrapper(buffer));
      }
      Err(err) => {
        let _ = tx_.blocking_send(CommandEvent::Error(err.to_string()));
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
    let mut options = CommandOptions::default();
    options.encoding = Encoding::Raw;

    let mut child = Shell::spawn(
      if cfg!(windows) { "cmd" } else { "sh" },
      &[if cfg!(windows) { "/C" } else { "-c" }, "echo test"],
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
