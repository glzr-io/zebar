use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};

use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};
use shell::{
  Buffer, ChildProcessEvent, CommandOptions, ProcessId, Shell,
  ShellExecOutput,
};
use tauri::{AppHandle, Emitter};
use tokio::sync::{mpsc, oneshot};

use crate::widget_factory::WidgetFactory;

/// Handle for managing a spawned child process.
#[derive(Debug)]
pub struct ProcessHandle {
  write_tx: mpsc::UnboundedSender<Buffer>,
  kill_tx: oneshot::Sender<()>,
  _event_task: tokio::task::JoinHandle<()>,
}

/// Payload for events emitted by spawned child processes.
///
/// Sent to the client via the `shell-emit` event.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellEmission {
  pid: ProcessId,
  event: ChildProcessEvent,
}

/// Arguments for a shell command.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ShellCommandArgs {
  String(String),
  Array(Vec<String>),
}

impl From<ShellCommandArgs> for Vec<String> {
  fn from(val: ShellCommandArgs) -> Self {
    match val {
      ShellCommandArgs::String(args) => {
        args.split(' ').map(String::from).collect()
      }
      ShellCommandArgs::Array(args) => args,
    }
  }
}

impl From<ShellCommandArgs> for String {
  fn from(val: ShellCommandArgs) -> Self {
    match val {
      ShellCommandArgs::String(args) => args,
      ShellCommandArgs::Array(args) => args.join(" "),
    }
  }
}

/// Manages the state and lifecycle of shell processes.
#[derive(Debug)]
pub struct ShellState {
  app_handle: AppHandle,
  children: Arc<Mutex<HashMap<ProcessId, ProcessHandle>>>,
  widget_factory: Arc<WidgetFactory>,
}

impl ShellState {
  /// Creates a new `ShellState` instance.
  pub fn new(
    app_handle: &AppHandle,
    widget_factory: Arc<WidgetFactory>,
  ) -> Self {
    Self {
      children: Arc::new(Mutex::new(HashMap::new())),
      app_handle: app_handle.clone(),
      widget_factory,
    }
  }

  /// Executes a command as a child process.
  ///
  /// Validates widget's shell privileges before executing the command.
  pub async fn exec(
    &self,
    widget_id: &str,
    program: &str,
    args: ShellCommandArgs,
    options: &CommandOptions,
  ) -> anyhow::Result<ShellExecOutput> {
    self
      .check_shell_privilege(widget_id, program, args.clone())
      .await?;

    let args_vec: Vec<String> = args.into();
    let output = Shell::exec(program, &args_vec, options).await?;

    Ok(output)
  }

  /// Spawns a new child process.
  ///
  /// Validates widget's shell privileges before spawning the process.
  /// Shell events are emitted to the given widget.
  pub async fn spawn(
    &self,
    widget_id: &str,
    program: &str,
    args: ShellCommandArgs,
    options: &CommandOptions,
  ) -> anyhow::Result<ProcessId> {
    self
      .check_shell_privilege(widget_id, program, args.clone())
      .await?;

    let args_vec: Vec<String> = args.into();
    let mut child = Shell::spawn(program, &args_vec, options)?;
    let app_handle = self.app_handle.clone();
    let widget_id = widget_id.to_string();
    let pid = child.pid();

    // Create channels for write and kill signals.
    let (write_tx, mut write_rx) = mpsc::unbounded_channel::<Buffer>();
    let (kill_tx, mut kill_rx) = oneshot::channel();

    // Set up event handling.
    let event_task = tokio::spawn(async move {
      loop {
        tokio::select! {
          // Process events from the child.
          Some(event) = child.events().recv() => {
            let _ = app_handle.emit_to(widget_id.clone(), "shell-emit", ShellEmission {
              pid,
              event,
            });
          }

          // Process write requests.
          Some(buffer) = write_rx.recv() => {
            if let Err(err) = child.write(buffer.as_bytes()) {
              let _ = app_handle.emit_to(widget_id.clone(), "shell-emit", ShellEmission {
                pid,
                event: ChildProcessEvent::Error(format!("Write error: {}", err)),
              });
            }
          }

          // Kill the process when signal is received.
          _ = &mut kill_rx => {
            let _ = child.kill();
            break;
          }
        }
      }
    });

    self.children.lock().unwrap().insert(
      pid,
      ProcessHandle {
        write_tx,
        kill_tx,
        _event_task: event_task,
      },
    );

    Ok(pid)
  }

  /// Writes data to the standard input of a running process.
  pub fn write(
    &self,
    pid: ProcessId,
    buffer: Buffer,
  ) -> anyhow::Result<()> {
    if let Some(handle) = self.children.lock().unwrap().get(&pid) {
      handle
        .write_tx
        .send(buffer)
        .context("Failed to send write command.")?;
    }

    Ok(())
  }

  /// Terminates a running process.
  pub fn kill(&self, pid: ProcessId) -> anyhow::Result<()> {
    if let Some(handle) = self.children.lock().unwrap().remove(&pid) {
      handle
        .kill_tx
        .send(())
        .map_err(|_| anyhow::anyhow!("Failed to send kill command."))?;
    }

    Ok(())
  }

  /// Validates whether a widget has privilege to execute a program with
  /// given arguments.
  ///
  /// Returns an error if widget does not have privilege.
  async fn check_shell_privilege(
    &self,
    widget_id: &str,
    program: &str,
    args: ShellCommandArgs,
  ) -> anyhow::Result<()> {
    let widget = self
      .widget_factory
      .state_by_id(widget_id)
      .await
      .with_context(|| {
        format!("Widget with ID '{widget_id}' not found.")
      })?;

    let args_str: String = args.into();
    let shell_privileges = widget.config.privileges.shell_commands;

    // Check if any privilege matches the program.
    let program_privileges: Vec<_> = shell_privileges
      .iter()
      .filter(|privilege| privilege.program == program)
      .collect();

    if program_privileges.is_empty() {
      bail!("No shell privileges found for program '{program}'.");
    }

    for privilege in program_privileges {
      // Allow empty args if args regex is also empty.
      if privilege.args_regex.is_empty() {
        if args_str.is_empty() {
          return Ok(());
        }

        continue;
      }

      // Check if args match the regex pattern.
      if let Ok(re) = regex::Regex::new(&privilege.args_regex) {
        if re.is_match(&args_str) {
          return Ok(());
        }
      }
    }

    bail!(
      "Arguments '{}' are not allowed for program '{}'. Check widget's shell privileges.",
      args_str,
      program
    )
  }
}

impl Drop for ShellState {
  fn drop(&mut self) {
    let mut children = self.children.lock().unwrap();

    for (_, child) in children.drain() {
      let _ = child.kill_tx.send(());
    }
  }
}
