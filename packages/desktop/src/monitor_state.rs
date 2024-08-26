use anyhow::bail;
use tauri::AppHandle;

use crate::cli::OutputMonitorsArgs;

pub struct MonitorState {
  app_handle: AppHandle,
  monitors: Vec<Monitor>,
}

pub struct Monitor {
  name: Option<String>,
  x: i32,
  y: i32,
  width: u32,
  height: u32,
  scale_factor: f64,
}

impl MonitorState {
  /// Creates a new `MonitorState` instance.
  pub fn new(app_handle: &AppHandle) -> Self {
    let monitors = app_handle
      .available_monitors()
      .map(|monitors| {
        monitors
          .into_iter()
          .map(|monitor| Monitor {
            name: monitor.name().cloned(),
            x: monitor.position().x,
            y: monitor.position().y,
            width: monitor.size().width,
            height: monitor.size().height,
            scale_factor: monitor.scale_factor(),
          })
          .collect()
      })
      .unwrap_or(Vec::new());

    Self {
      app_handle: app_handle.clone(),
      monitors,
    }
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
}
