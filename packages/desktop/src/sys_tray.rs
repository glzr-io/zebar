use std::{collections::HashMap, path::PathBuf, str::FromStr, sync::Arc};

use anyhow::{bail, Context};
use tauri::{
  image::Image,
  menu::{CheckMenuItem, Menu, MenuBuilder, Submenu, SubmenuBuilder},
  tray::{TrayIcon, TrayIconBuilder},
  AppHandle, Wry,
};
use tokio::task;
use tracing::{error, info};

use crate::{
  common::PathExt,
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
    let window_factory = self.window_factory.clone();
    let tooltip = format!("Zebar v{}", env!("VERSION_NUMBER"));

    let tray_icon = TrayIconBuilder::with_id("tray")
      .icon(self.icon_image()?)
      .menu(&self.create_tray_menu().await?)
      .tooltip(tooltip)
      .on_menu_event(move |app, event| {
        let app_handle = app.clone();
        let config = config.clone();
        let window_factory = window_factory.clone();

        task::spawn(async move {
          let event = MenuEvent::from_str(event.id.as_ref());

          if let Ok(event) = event {
            info!("Received tray menu event: {}", event.to_string());

            let res = Self::handle_menu_event(
              event,
              app_handle,
              config,
              window_factory,
            )
            .await;

            if let Err(err) = res {
              error!("{:?}", err);
            }
          }
        });
      })
      .build(&self.app_handle)?;

    Ok(tray_icon)
  }

  fn start_listener(self: Arc<Self>) {
    let mut config_changes_rx = self.config.changes_tx.subscribe();
    let mut window_open_rx = self.window_factory.open_tx.subscribe();
    let mut window_close_rx = self.window_factory.close_tx.subscribe();

    tokio::spawn(async move {
      loop {
        let event = tokio::select! {
          Ok(_) = config_changes_rx.recv() => "Config changes",
          Ok(_) = window_open_rx.recv() => "Window open",
          Ok(_) = window_close_rx.recv() => "Window close",
        };

        info!("{} received in system tray. Updating menu.", event);

        // Drain receiver channels if multiple events are queued up.
        config_changes_rx = config_changes_rx.resubscribe();
        window_open_rx = window_open_rx.resubscribe();
        window_close_rx = window_close_rx.resubscribe();

        if let Some(tray_icon) = self.tray_icon.as_ref() {
          if let Err(err) =
            self.create_tray_menu().await.and_then(|menu| {
              tray_icon
                .set_menu(Some(menu))
                .context("Failed to set tray menu.")
            })
          {
            error!(
              "Failed to update tray menu after {} event: {:?}",
              event, err
            );
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
  async fn handle_menu_event(
    event: MenuEvent,
    app_handle: AppHandle,
    config: Arc<Config>,
    window_factory: Arc<WindowFactory>,
  ) -> anyhow::Result<()> {
    match event {
      MenuEvent::ShowConfigFolder => {
        info!("Opening config folder from system tray.");

        config
          .open_config_dir()
          .context("Failed to open config folder.")
      }
      MenuEvent::Exit => {
        info!("Exiting through system tray.");

        app_handle.exit(0);
        Ok(())
      }
      MenuEvent::EnableWindowConfig(path) => {
        info!("Window config at path {} enabled.", path);

        let window_config = config
          .window_config_by_path(&path)
          .await?
          .context("Window config not found.")?;

        window_factory.open(window_config).await
      }
      MenuEvent::StartupWindowConfig(path) => {
        info!("Window config at path {} set to launch on startup.", path);

        // config.add_startup_config(path).await
        Ok(())
      }
    }
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

    // **
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
    config_path
      .strip_prefix(&config_dir.to_unicode_string())
      .and_then(|path| path.strip_suffix(".zebar.json"))
      .unwrap_or(config_path)
      .into()
  }
}
