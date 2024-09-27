use std::sync::Arc;

use serde::Serialize;
use tauri::AppHandle;
use tokio::{
  sync::{broadcast, Mutex},
  task,
};
use tracing::info;

use crate::config::MonitorSelection;

pub struct MonitorState {
  _change_rx: broadcast::Receiver<Vec<Monitor>>,

  pub change_tx: broadcast::Sender<Vec<Monitor>>,

  /// Available monitors sorted from left-to-right and top-to-bottom.
  monitors: Arc<Mutex<Vec<Monitor>>>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Monitor {
  pub name: Option<String>,
  pub is_primary: bool,
  pub x: i32,
  pub y: i32,
  pub width: u32,
  pub height: u32,
  pub scale_factor: f64,
}

impl MonitorState {
  /// Creates a new `MonitorState` instance.
  pub fn new(app_handle: &AppHandle) -> Self {
    let (change_tx, _change_rx) = broadcast::channel(16);

    let monitors =
      Arc::new(Mutex::new(Self::available_monitors(app_handle)));

    Self::listen_changes(
      app_handle.clone(),
      monitors.clone(),
      change_tx.clone(),
    );

    Self {
      monitors,
      _change_rx,
      change_tx,
    }
  }

  /// Listens for display setting changes.
  ///
  /// Updates monitor state on scaling changes, monitor connections, and
  /// monitor disconnections. Does not update on working area changes.
  fn listen_changes(
    app_handle: AppHandle,
    monitors: Arc<Mutex<Vec<Monitor>>>,
    change_tx: broadcast::Sender<Vec<Monitor>>,
  ) {
    task::spawn(async move {
      loop {
        let new_monitors = Self::available_monitors(&app_handle);
        let mut monitors = monitors.lock().await;

        if *monitors != new_monitors {
          info!("Detected change in monitors.");
          let _ = change_tx.send(new_monitors.clone());
          *monitors = new_monitors;
        }

        tokio::time::sleep(std::time::Duration::from_secs(4)).await;
      }
    });
  }

  /// Gets available monitors on the system.
  ///
  /// Returns a vector of `Monitor` instances sorted from left-to-right and
  /// top-to-bottom.
  fn available_monitors(app_handle: &AppHandle) -> Vec<Monitor> {
    let primary_monitor = app_handle.primary_monitor().unwrap_or(None);

    let mut monitors = app_handle
      .available_monitors()
      .map(|monitors| {
        monitors
          .into_iter()
          .map(|monitor| Monitor {
            name: monitor.name().cloned(),
            is_primary: primary_monitor
              .as_ref()
              .map(|m| m.name() == monitor.name())
              .unwrap_or(false),
            x: monitor.position().x,
            y: monitor.position().y,
            width: monitor.size().width,
            height: monitor.size().height,
            scale_factor: monitor.scale_factor(),
          })
          .collect()
      })
      .unwrap_or(Vec::new());

    // Sort monitors from left-to-right, top-to-bottom.
    monitors.sort_by(|monitor_a, monitor_b| {
      if monitor_a.x == monitor_b.x {
        monitor_a.y.cmp(&monitor_b.y)
      } else {
        monitor_a.x.cmp(&monitor_b.x)
      }
    });

    monitors
  }

  /// Returns a string representation of the monitors.
  pub fn output_str(&self) -> anyhow::Result<String> {
    let monitors = self.monitors.try_lock()?;

    let monitors_str =
      serde_json::to_string::<Vec<Monitor>>(monitors.as_ref())?;

    Ok(monitors_str)
  }

  pub async fn monitors_by_selection(
    &self,
    monitor_selection: &MonitorSelection,
  ) -> Vec<Monitor> {
    let monitors = self.monitors.lock().await.clone();

    match monitor_selection {
      MonitorSelection::All => monitors,
      MonitorSelection::Primary => monitors
        .into_iter()
        .filter(|monitor| monitor.is_primary)
        .collect(),
      MonitorSelection::Secondary => monitors
        .into_iter()
        .filter(|monitor| !monitor.is_primary)
        .collect(),
      MonitorSelection::Index(index) => {
        monitors.get(*index).cloned().into_iter().collect()
      }
      MonitorSelection::Name(name) => monitors
        .into_iter()
        .filter(|monitor| monitor.name.as_deref() == Some(name))
        .collect(),
    }
  }
}
