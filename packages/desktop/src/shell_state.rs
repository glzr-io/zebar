use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use shell::{Buffer, CommandEvent, CommandOptions, ProcessId, Shell};
use tauri::{AppHandle, Emitter};
use tokio::sync::{mpsc, oneshot};

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
  event: CommandEvent,
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

/// Manages the state and lifecycle of shell processes.
#[derive(Clone, Debug)]
pub struct ShellState {
  children: Arc<Mutex<HashMap<ProcessId, ProcessHandle>>>,
  app_handle: AppHandle,
}

impl ShellState {
  /// Creates a new `ShellState` instance.
  pub fn new(app_handle: &AppHandle) -> Self {
    Self {
      children: Arc::new(Mutex::new(HashMap::new())),
      app_handle: app_handle.clone(),
    }
  }

  /// Spawns a new child process.
  pub fn spawn(
    &self,
    program: &str,
    args: &Vec<String>,
    options: &CommandOptions,
  ) -> anyhow::Result<ProcessId> {
    let mut child = Shell::spawn(program, args, options)?;
    let pid = child.pid();
    let app_handle = self.app_handle.clone();

    // Create channels for write and kill signals.
    let (write_tx, mut write_rx) = mpsc::unbounded_channel::<Buffer>();
    let (kill_tx, mut kill_rx) = oneshot::channel();

    // Set up event handling.
    let event_task = tokio::spawn(async move {
      loop {
        tokio::select! {
          // Process events from the child.
          Some(event) = child.events().recv() => {
            let _ = app_handle.emit("shell-emit", ShellEmission {
              pid,
              event,
            });
          }

          // Process write requests.
          Some(buffer) = write_rx.recv() => {
            if let Err(err) = child.write(buffer.as_bytes()) {
              let _ = app_handle.emit("shell-emit", ShellEmission {
                pid,
                event: CommandEvent::Error(format!("Write error: {}", err)),
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
}

impl Drop for ShellState {
  fn drop(&mut self) {
    let mut children = self.children.lock().unwrap();

    for (_, child) in children.drain() {
      let _ = child.kill_tx.send(());
    }
  }
}
