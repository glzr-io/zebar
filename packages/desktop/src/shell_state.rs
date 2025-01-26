use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};

use anyhow::Context;
use shell::{
  Buffer, ChildProcess, CommandEvent, CommandOptions, ProcessId, Shell,
};
use tauri::{AppHandle, Emitter};
use tokio::sync::{mpsc, oneshot};

/// Handle for managing a spawned child process.
#[derive(Debug)]
pub struct ProcessHandle {
  write_tx: mpsc::UnboundedSender<Buffer>,
  kill_tx: oneshot::Sender<()>,
  _event_task: tokio::task::JoinHandle<()>,
}

/// Events emitted by spawned child processes.
#[derive(Clone, serde::Serialize)]
#[serde(tag = "type", content = "event")]
pub enum ShellEvent {
  Stdout {
    pid: ProcessId,
    data: String,
  },
  Stderr {
    pid: ProcessId,
    data: String,
  },
  Error {
    pid: ProcessId,
    message: String,
  },
  Exit {
    pid: ProcessId,
    code: Option<i32>,
    signal: Option<i32>,
  },
}

/// Manages the state and lifecycle of shell processes.
#[derive(Clone, Debug)]
pub struct ShellState {
  children: Arc<Mutex<HashMap<ProcessId, ProcessHandle>>>,
  app_handle: AppHandle,
}

impl ShellState {
  /// Creates a new `ShellState` instance.
  pub fn new(app_handle: AppHandle) -> Self {
    Self {
      children: Arc::new(Mutex::new(HashMap::new())),
      app_handle,
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
    let (kill_tx, kill_rx) = oneshot::channel();

    // Set up event handling.
    let event_task = tokio::spawn(async move {
      tokio::select! {
        // Process events from the child.
        Some(event) = child.events().recv() => {
          Self::handle_event(&app_handle, pid, event);
        }

        // Process write requests.
        Some(buffer) = write_rx.recv() => {
          if let Err(err) = child.write(buffer.as_bytes()) {
            let _ = app_handle.emit("shell-event", ShellEvent::Error {
              pid,
              message: format!("Write error: {}", err),
            });
          }
        }

        // Kill the process when signal is received.
        _ = kill_rx => {
          let _ = child.kill();
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

  fn handle_event(
    app_handle: &AppHandle,
    pid: ProcessId,
    event: CommandEvent,
  ) {
    let shell_event = match event {
      CommandEvent::Stdout(buffer) => {
        if let Some(text) = buffer.as_str() {
          Some(ShellEvent::Stdout {
            pid,
            data: text.to_string(),
          })
        } else {
          None
        }
      }
      CommandEvent::Stderr(buffer) => {
        if let Some(text) = buffer.as_str() {
          Some(ShellEvent::Stderr {
            pid,
            data: text.to_string(),
          })
        } else {
          None
        }
      }
      CommandEvent::Error(message) => {
        Some(ShellEvent::Error { pid, message })
      }
      CommandEvent::Terminated(status) => Some(ShellEvent::Exit {
        pid,
        code: status.code,
        signal: status.signal,
      }),
    };

    if let Some(event) = shell_event {
      let _ = app_handle.emit("shell-event", event);
    }
  }

  /// Writes data to the standard input of a running process.
  pub fn stdin_write(
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
