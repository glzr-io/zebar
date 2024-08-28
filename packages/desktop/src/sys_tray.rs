use std::{path::PathBuf, sync::Arc};

use anyhow::Context;
use tauri::{
  menu::{MenuBuilder, Submenu, SubmenuBuilder},
  tray::{TrayIcon, TrayIconBuilder},
  AppHandle, Wry,
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
      .item(&Self::configs_menu(app_handle, config.clone())?)
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

  /// Creates a submenu for the window configs.
  fn configs_menu(
    app_handle: &AppHandle,
    config: Arc<Config>,
  ) -> anyhow::Result<Submenu<Wry>> {
    let configs_menu = SubmenuBuilder::new(app_handle, "Window configs");

    // Add each window config to the menu.
    let configs_menu = config
      .window_configs
      .iter()
      .fold(configs_menu, |menu, window_config| {
        menu.text(
          window_config.config_path.clone(),
          Self::format_config_path(
            &window_config.config_path,
            &config.config_dir,
          ),
        )
      })
      .build()?;

    Ok(configs_menu)
  }

  /// Formats the config path for display in the system tray.
  fn format_config_path(
    config_path: &str,
    config_dir: &PathBuf,
  ) -> String {
    let config_pathbuf = PathBuf::from(config_path);

    config_pathbuf
      .strip_prefix(config_dir)
      .unwrap_or(&config_pathbuf)
      .with_extension("")
      .to_string_lossy()
      .to_string()
  }
}
