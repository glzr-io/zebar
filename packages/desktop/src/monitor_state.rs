use anyhow::bail;
use tauri::AppHandle;

use crate::{cli::OutputMonitorsArgs, config::MonitorSelection};

pub struct MonitorState {
  /// Handle to the Tauri application.
  app_handle: AppHandle,

  /// Available monitors sorted from left-to-right and top-to-bottom.
  monitors: Vec<Monitor>,
}

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
    Self {
      app_handle: app_handle.clone(),
      monitors: Self::monitors(app_handle),
    }
  }

  /// Returns a vector of available monitors sorted from left-to-right and
  /// top-to-bottom.
  fn monitors(app_handle: &AppHandle) -> Vec<Monitor> {
    let mut primary_monitor = app_handle.primary_monitor().unwrap_or(None);

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
  pub fn output_str(
    &self,
    args: OutputMonitorsArgs,
  ) -> anyhow::Result<String> {
    if self.monitors.len() == 0 {
      bail!("No monitors found")
    }

    let mut monitors_str = String::new();

    for monitor in &self.monitors {
      monitors_str += &format!(
      "MONITOR_NAME=\"{}\" MONITOR_X=\"{}\" MONITOR_Y=\"{}\" MONITOR_WIDTH=\"{}\" MONITOR_HEIGHT=\"{}\" MONITOR_SCALE_FACTOR=\"{}\"",
      monitor.name.clone().unwrap_or("".into()),
      monitor.x,
      monitor.y,
      monitor.width,
      monitor.height,
      monitor.scale_factor
    );

      monitors_str += match args.print0 {
        true => "\0",
        false => "\n",
      };
    }

    Ok(monitors_str)
  }

  pub fn monitors_by_selection(
    &self,
    monitor_selection: &MonitorSelection,
  ) -> Vec<&Monitor> {
    match monitor_selection {
      MonitorSelection::All => self.monitors.iter().collect(),
      MonitorSelection::Primary => self
        .monitors
        .iter()
        .filter(|monitor| monitor.is_primary)
        .collect(),
      MonitorSelection::Secondary => self
        .monitors
        .iter()
        .filter(|monitor| !monitor.is_primary)
        .collect(),
      MonitorSelection::Index(index) => {
        self.monitors.get(*index).into_iter().collect()
      }
      MonitorSelection::Name(name) => self
        .monitors
        .iter()
        .filter(|monitor| monitor.name.as_deref() == Some(name))
        .collect(),
    }
  }
}
