use anyhow::{Context, Result};
use tauri::{App, Runtime};

pub fn output_monitors<R: Runtime>(app: &mut App<R>) -> Result<()> {
  let monitors = app
    .available_monitors()
    .context("Unable to detect monitors")?;

  for monitor in monitors {
    // Get monitor name with escaped spaces.
    let monitor_name = monitor
      .name()
      .context("Unable to read monitor name")?
      .replace(' ', "\\ ");

    println!(
      "MONITOR_NAME={} MONITOR_X={} MONITOR_Y={} MONITOR_WIDTH={} MONITOR_HEIGHT={}",
      monitor_name,
      monitor.position().x,
      monitor.position().y,
      monitor.size().width,
      monitor.size().height
    );
  }

  Ok(())
}
