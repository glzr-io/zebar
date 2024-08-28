use std::{fmt, path::PathBuf, str::FromStr, sync::Arc};

use anyhow::Context;
use tauri::{
  menu::{CheckMenuItem, MenuBuilder, Submenu, SubmenuBuilder},
  tray::{TrayIcon, TrayIconBuilder},
  AppHandle, Wry,
};
use tracing::{error, info};

use crate::config::{Config, WindowConfigEntry};

#[derive(Debug, Clone)]
enum MenuId {
  ShowConfigFolder,
  Exit,
  EnableWindowConfig(String),
  StartupWindowConfig(String),
}

impl fmt::Display for MenuId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      MenuId::ShowConfigFolder => write!(f, "show_config_folder"),
      MenuId::Exit => write!(f, "exit"),
      MenuId::EnableWindowConfig(path) => {
        write!(f, "enable_window_config_{}", path)
      }
      MenuId::StartupWindowConfig(path) => {
        write!(f, "startup_window_config_{}", path)
      }
    }
  }
}

impl FromStr for MenuId {
  type Err = String;

  fn from_str(id_str: &str) -> Result<Self, Self::Err> {
    if id_str == "show_config_folder" {
      Ok(MenuId::ShowConfigFolder)
    } else if id_str == "exit" {
      Ok(MenuId::Exit)
    } else if let Some(path) = id_str.strip_prefix("enable_window_config_")
    {
      Ok(MenuId::EnableWindowConfig(path.to_string()))
    } else if let Some(path) =
      id_str.strip_prefix("startup_window_config_")
    {
      Ok(MenuId::StartupWindowConfig(path.to_string()))
    } else {
      Err(format!("Invalid menu ID: {}", id_str))
    }
  }
}

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
      .text(MenuId::ShowConfigFolder, "Show config folder")
      .separator()
      .text(MenuId::Exit, "Exit")
      .build()?;

    let tray_icon = TrayIconBuilder::with_id("tray")
      .icon(icon_image.clone())
      .menu(&tray_menu)
      .tooltip(format!("Zebar v{}", env!("VERSION_NUMBER")))
      .on_menu_event(move |app, event| {
        if let Ok(event) = MenuId::from_str(&event.id().0) {
          match event {
            MenuId::ShowConfigFolder => {
              info!("Opening config folder from system tray.");

              if let Err(err) = config.open_config_dir() {
                error!("Failed to open config folder: {}", err);
              }
            }
            MenuId::Exit => {
              info!("Exiting through system tray.");
              app.exit(0)
            }
            MenuId::EnableWindowConfig(id) => {
              info!("Window config enabled: {}", id);
            }
            MenuId::StartupWindowConfig(id) => {
              info!("Window config set to launch on startup: {}", id);
            }
          }
        } else {
          println!("Unknown menu event: {}", event.id().0);
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

  fn config_menu(
    app_handle: &AppHandle,
    config_entry: &WindowConfigEntry,
  ) -> anyhow::Result<Submenu<Wry>> {
    // TODO: Get whether it's enabled.
    let enabled_item =
      CheckMenuItem::new(app_handle, "Enabled", true, true, None::<&str>)?;

    // TODO: Get whether it's launched on startup.
    let startup_item = CheckMenuItem::new(
      app_handle,
      "Launch on startup",
      true,
      true,
      None::<&str>,
    )?;

    // TODO: Show the label for the config.
    let config_menu = SubmenuBuilder::new(app_handle, "Window config")
      .item(&enabled_item)
      .item(&startup_item)
      .build()?;

    Ok(config_menu)
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
