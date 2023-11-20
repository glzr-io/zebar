use anyhow::{bail, Context, Result};
use tauri::{App, Runtime};

pub fn get_monitors_str<R: Runtime>(
  app: &mut App<R>,
  print0: bool,
) -> Result<String> {
  let monitors = app
    .available_monitors()
    .context("Unable to detect monitors")?;

  if monitors.len() == 0 {
    bail!("No monitors found")
  }

  let mut monitors_str = String::new();

  for monitor in monitors {
    monitors_str += &format!(
      "MONITOR_NAME=\"{}\" MONITOR_X=\"{}\" MONITOR_Y=\"{}\" MONITOR_WIDTH=\"{}\" MONITOR_HEIGHT=\"{}\"",
      monitor.name().context("Unable to read monitor name")?,
      monitor.position().x,
      monitor.position().y,
      monitor.size().width,
      monitor.size().height
    );

    monitors_str += match print0 {
      true => "\0",
      false => "\n",
    };
  }

  Ok(monitors_str)
}
