use std::sync::Arc;

use anyhow::Context;
use tauri::{
  menu::MenuBuilder,
  tray::{TrayIcon, TrayIconBuilder},
  AppHandle,
};
use tracing::{error, info};

use crate::config::Config;

const SHOW_CONFIG_FOLDER_ID: &str = "show_config_folder";
const EXIT_ID: &str = "exit";

/// System tray icon for Zebar.
pub struct SysTray {}

impl SysTray {
  /// Creates a new system tray icon for Zebar.
  pub fn new(
    app_handle: &AppHandle,
    config: Arc<Config>,
  ) -> anyhow::Result<TrayIcon> {
    let icon_image = app_handle
      .default_window_icon()
      .context("No icon defined in Tauri config.")?;

    let tray_menu = MenuBuilder::new(app_handle)
      .text(SHOW_CONFIG_FOLDER_ID, "Show config folder")
      .separator()
      .text(EXIT_ID, "Exit")
      .build()?;

    let tray_icon = TrayIconBuilder::with_id("tray")
      .icon(icon_image.clone())
      .menu(&tray_menu)
      .tooltip(format!("Zebar v{}", env!("VERSION_NUMBER")))
      .on_menu_event(move |app, event| match event.id().as_ref() {
        SHOW_CONFIG_FOLDER_ID => {
          info!("Opening config folder from system tray.");

          if let Err(err) = config.open_config_dir() {
            error!("Failed to open config folder: {}", err);
          }
        }
        EXIT_ID => {
          info!("Exiting through system tray.");
          app.exit(0)
        }
        other => {
          error!("Unknown menu event: {}", other);
        }
      })
      .build(app_handle)?;

    Ok(tray_icon)
  }
}
