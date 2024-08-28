use std::{fmt, path::PathBuf, str::FromStr, sync::Arc};

use anyhow::Context;
use tauri::{
  image::Image,
  menu::{CheckMenuItem, Menu, MenuBuilder, Submenu, SubmenuBuilder},
  tray::{TrayIcon, TrayIconBuilder},
  AppHandle, Wry,
};
use tracing::{error, info, warn};

use crate::config::{Config, WindowConfigEntry};

#[derive(Debug, Clone)]
enum MenuEvent {
  ShowConfigFolder,
  Exit,
  EnableWindowConfig(String),
  StartupWindowConfig(String),
}

impl fmt::Display for MenuEvent {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      MenuEvent::ShowConfigFolder => write!(f, "show_config_folder"),
      MenuEvent::Exit => write!(f, "exit"),
      MenuEvent::EnableWindowConfig(id) => {
        write!(f, "enable_window_config_{}", id)
      }
      MenuEvent::StartupWindowConfig(id) => {
        write!(f, "startup_window_config_{}", id)
      }
    }
  }
}

impl FromStr for MenuEvent {
  type Err = String;

  fn from_str(event: &str) -> Result<Self, Self::Err> {
    if event == "show_config_folder" {
      Ok(MenuEvent::ShowConfigFolder)
    } else if event == "exit" {
      Ok(MenuEvent::Exit)
    } else if let Some(id) = event.strip_prefix("enable_window_config_") {
      Ok(MenuEvent::EnableWindowConfig(id.to_string()))
    } else if let Some(id) = event.strip_prefix("startup_window_config_") {
      Ok(MenuEvent::StartupWindowConfig(id.to_string()))
    } else {
      Err(format!("Invalid menu event: {}", event))
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
    let tooltip = format!("Zebar v{}", env!("VERSION_NUMBER"));

    let tray_icon = TrayIconBuilder::with_id("tray")
      .icon(Self::icon_image(app_handle)?)
      .menu(&Self::tray_menu(app_handle, config.clone())?)
      .tooltip(tooltip)
      .on_menu_event(move |app, event| {
        if let Ok(event) = MenuEvent::from_str(&event.id().as_ref()) {
          Self::handle_menu_event(event, app, config.clone());
        } else {
          warn!("Unknown menu event: {}", event.id().as_ref());
        }
      })
      .build(app_handle)?;

    Ok(tray_icon)
  }

  /// Returns the image to use for the system tray icon.
  fn icon_image(app_handle: &AppHandle) -> anyhow::Result<Image> {
    app_handle
      .default_window_icon()
      .cloned()
      .context("No icon defined in Tauri config.")
  }

  /// Creates and returns the main system tray menu.
  fn tray_menu(
    app_handle: &AppHandle,
    config: Arc<Config>,
  ) -> anyhow::Result<Menu<Wry>> {
    let tray_menu = MenuBuilder::new(app_handle)
      .item(&Self::configs_menu(app_handle, config)?)
      .text(MenuEvent::ShowConfigFolder, "Show config folder")
      .separator()
      .text(MenuEvent::Exit, "Exit")
      .build()?;

    Ok(tray_menu)
  }

  /// Callback for system tray menu events.
  fn handle_menu_event(
    event: MenuEvent,
    app_handle: &AppHandle,
    config: Arc<Config>,
  ) {
    match event {
      MenuEvent::ShowConfigFolder => {
        info!("Opening config folder from system tray.");

        if let Err(err) = config.open_config_dir() {
          error!("Failed to open config folder: {}", err);
        }
      }
      MenuEvent::Exit => {
        info!("Exiting through system tray.");
        app_handle.exit(0)
      }
      MenuEvent::EnableWindowConfig(id) => {
        info!("Window config enabled: {}", id);
      }
      MenuEvent::StartupWindowConfig(id) => {
        info!("Window config set to launch on startup: {}", id);
      }
    }
  }

  /// Creates and returns a submenu for the window configs.
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

  /// Creates and returns a submenu for the given window config.
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
