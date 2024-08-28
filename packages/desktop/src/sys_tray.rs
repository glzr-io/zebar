use std::{collections::HashMap, path::PathBuf, str::FromStr, sync::Arc};

use anyhow::{bail, Context};
use tauri::{
  image::Image,
  menu::{CheckMenuItem, Menu, MenuBuilder, Submenu, SubmenuBuilder},
  tray::{TrayIcon, TrayIconBuilder},
  AppHandle, Wry,
};
use tracing::{error, info};

use crate::{
  config::Config,
  window_factory::{WindowFactory, WindowState},
};

#[derive(Debug, Clone)]
enum MenuEvent {
  ShowConfigFolder,
  Exit,
  EnableWindowConfig(String),
  StartupWindowConfig(String),
}

impl ToString for MenuEvent {
  fn to_string(&self) -> String {
    match self {
      MenuEvent::ShowConfigFolder => "show_config_folder".to_string(),
      MenuEvent::Exit => "exit".to_string(),
      MenuEvent::EnableWindowConfig(id) => format!("enable_{}", id),
      MenuEvent::StartupWindowConfig(id) => format!("startup_{}", id),
    }
  }
}

impl FromStr for MenuEvent {
  type Err = anyhow::Error;

  fn from_str(event: &str) -> Result<Self, Self::Err> {
    if event == "show_config_folder" {
      Ok(MenuEvent::ShowConfigFolder)
    } else if event == "exit" {
      Ok(MenuEvent::Exit)
    } else if let Some(id) = event.strip_prefix("enable_") {
      Ok(MenuEvent::EnableWindowConfig(id.to_string()))
    } else if let Some(id) = event.strip_prefix("startup_") {
      Ok(MenuEvent::StartupWindowConfig(id.to_string()))
    } else {
      bail!("Invalid menu event: {}", event)
    }
  }
}

/// System tray icon for Zebar.
pub struct SysTray {
  app_handle: AppHandle,
  config: Arc<Config>,
  window_factory: Arc<WindowFactory>,
  tray_icon: Option<TrayIcon>,
}

impl SysTray {
  /// Creates a new system tray icon for Zebar.
  pub async fn new(
    app_handle: &AppHandle,
    config: Arc<Config>,
    window_factory: Arc<WindowFactory>,
  ) -> anyhow::Result<Arc<SysTray>> {
    let mut sys_tray = Self {
      app_handle: app_handle.clone(),
      config,
      window_factory,
      tray_icon: None,
    };

    sys_tray.tray_icon = Some(sys_tray.create_tray_icon().await?);

    let sys_tray = Arc::new(sys_tray);
    sys_tray.clone().start_listener();

    Ok(sys_tray)
  }

  async fn create_tray_icon(&self) -> anyhow::Result<TrayIcon> {
    let config = self.config.clone();
    let tooltip = format!("Zebar v{}", env!("VERSION_NUMBER"));

    let tray_icon = TrayIconBuilder::with_id("tray")
      .icon(self.icon_image()?)
      .menu(&self.create_tray_menu().await?)
      .tooltip(tooltip)
      .on_menu_event(move |app, event| {
        let event = MenuEvent::from_str(event.id.as_ref());

        let event_res = event.map(|event| {
          Self::handle_menu_event(event, &app, config.clone())
        });

        if let Err(err) = event_res {
          error!("{:?}", err);
        }
      })
      .build(&self.app_handle)?;

    Ok(tray_icon)
  }

  fn start_listener(self: Arc<Self>) {
    let config = self.config.clone();
    let mut changes_rx = config.changes_tx.subscribe();

    tokio::spawn(async move {
      tokio::select! {
        Ok(_) = changes_rx.recv() => {
          if let Some(tray_icon) = self.tray_icon.as_ref() {
            tray_icon.set_menu(Some(self.create_tray_menu().await.unwrap()));
          }
        }
      }
    });
  }

  /// Returns the image to use for the system tray icon.
  fn icon_image(&self) -> anyhow::Result<Image> {
    self
      .app_handle
      .default_window_icon()
      .cloned()
      .context("No icon defined in Tauri config.")
  }

  /// Creates and returns the main system tray menu.
  async fn create_tray_menu(&self) -> anyhow::Result<Menu<Wry>> {
    let window_states = self.window_factory.states_by_config_path().await;

    let mut tray_menu = MenuBuilder::new(&self.app_handle)
      .item(&self.create_configs_menu(&window_states).await?)
      .text(MenuEvent::ShowConfigFolder, "Show config folder")
      .separator();

    // Add submenus for currently active windows.
    if !window_states.is_empty() {
      for (config_path, states) in window_states {
        let label = format!(
          "({}) {}",
          states.len(),
          Self::format_config_path(&config_path, &self.config.config_dir)
        );

        tray_menu = tray_menu.item(&self.create_config_menu(
          &config_path,
          &label,
          &states,
        )?);
      }

      tray_menu = tray_menu.separator();
    }

    tray_menu = tray_menu.text(MenuEvent::Exit, "Exit");

    Ok(tray_menu.build()?)
  }

  /// Callback for system tray menu events.
  fn handle_menu_event(
    event: MenuEvent,
    app_handle: &AppHandle,
    config: Arc<Config>,
  ) -> anyhow::Result<()> {
    match event {
      MenuEvent::ShowConfigFolder => {
        info!("Opening config folder from system tray.");

        config
          .open_config_dir()
          .context("Failed to open config folder.")?;
      }
      MenuEvent::Exit => {
        info!("Exiting through system tray.");

        app_handle.exit(0)
      }
      MenuEvent::EnableWindowConfig(id) => {
        info!("Window config at index {} enabled.", id);
      }
      MenuEvent::StartupWindowConfig(id) => {
        info!("Window config at index {} set to launch on startup.", id);
      }
    };

    Ok(())
  }

  /// Creates and returns a submenu for the window configs.
  async fn create_configs_menu(
    &self,
    window_states: &HashMap<String, Vec<WindowState>>,
  ) -> anyhow::Result<Submenu<Wry>> {
    let mut configs_menu =
      SubmenuBuilder::new(&self.app_handle, "Window configs");

    // Add each window config to the menu.
    for window_config in &self.config.window_configs().await {
      let label = Self::format_config_path(
        &window_config.config_path,
        &self.config.config_dir,
      );

      let menu_item = self.create_config_menu(
        &window_config.config_path,
        &label,
        window_states
          .get(&window_config.config_path)
          .unwrap_or(&vec![]),
      )?;

      configs_menu = configs_menu.item(&menu_item);
    }

    Ok(configs_menu.build()?)
  }

  /// Creates and returns a submenu for the given window config.
  fn create_config_menu(
    &self,
    config_path: &str,
    label: &str,
    window_states: &Vec<WindowState>,
  ) -> anyhow::Result<Submenu<Wry>> {
    let enabled_item = CheckMenuItem::with_id(
      &self.app_handle,
      MenuEvent::EnableWindowConfig(config_path.to_string()),
      "Enabled",
      true,
      !window_states.is_empty(),
      None::<&str>,
    )?;

    // TODO: Get whether it's launched on startup.
    let startup_item = CheckMenuItem::with_id(
      &self.app_handle,
      MenuEvent::StartupWindowConfig(config_path.to_string()),
      "Launch on startup",
      true,
      true,
      None::<&str>,
    )?;

    let config_menu = SubmenuBuilder::new(&self.app_handle, label)
      .item(&enabled_item)
      .item(&startup_item);

    Ok(config_menu.build()?)
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
