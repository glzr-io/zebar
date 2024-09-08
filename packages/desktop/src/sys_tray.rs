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
  ToggleWindowConfig { enable: bool, path: PathBuf },
  ToggleStartupWindowConfig { enable: bool, path: PathBuf },
}

impl ToString for MenuEvent {
  fn to_string(&self) -> String {
    match self {
      MenuEvent::ShowConfigFolder => "show_config_folder".to_string(),
      MenuEvent::Exit => "exit".to_string(),
      MenuEvent::ToggleWindowConfig { enable, path } => {
        format!(
          "toggle_window_config_{}_{}",
          enable,
          path.to_unicode_string()
        )
      }
      MenuEvent::ToggleStartupWindowConfig { enable, path } => {
        format!(
          "toggle_startup_window_config_{}_{}",
          enable,
          path.to_unicode_string()
        )
      }
    }
  }
}

impl FromStr for MenuEvent {
  type Err = anyhow::Error;

  fn from_str(event: &str) -> Result<Self, Self::Err> {
    let parts: Vec<&str> = event.split('_').collect();

    match parts.as_slice() {
      ["show", "config", "folder"] => Ok(Self::ShowConfigFolder),
      ["exit"] => Ok(Self::Exit),
      ["toggle", "window", "config", enable @ ("true" | "false"), path @ ..] => {
        Ok(Self::ToggleWindowConfig {
          enable: *enable == "true",
          path: PathBuf::from(path.join("_")),
        })
      }
      ["toggle", "startup", "window", "config", enable @ ("true" | "false"), path @ ..] => {
        Ok(Self::ToggleStartupWindowConfig {
          enable: *enable == "true",
          path: PathBuf::from(path.join("_")),
        })
      }
      _ => bail!("Invalid menu event: {}", event),
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
    let mut window_open_rx = self.window_factory.open_tx.subscribe();
    let mut window_close_rx = self.window_factory.close_tx.subscribe();
    let mut settings_change_rx =
      self.config.settings_change_tx.subscribe();

    tokio::spawn(async move {
      loop {
        let event = tokio::select! {
          Ok(_) = window_open_rx.recv() => "Window open",
          Ok(_) = window_close_rx.recv() => "Window close",
          Ok(_) = settings_change_rx.recv() => "Settings change",
        };

        info!("{} received in system tray. Updating menu.", event);

        // Drain receiver channels if multiple events are queued up.
        window_open_rx = window_open_rx.resubscribe();
        window_close_rx = window_close_rx.resubscribe();
        settings_change_rx = settings_change_rx.resubscribe();

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

    let startup_config_paths = self
      .config
      .startup_window_configs()
      .await?
      .into_iter()
      .map(|entry| entry.config_path)
      .collect();

    let configs_menu = self
      .create_configs_menu(&window_states, &startup_config_paths)
      .await?;

    let mut tray_menu = MenuBuilder::new(&self.app_handle)
      .item(&configs_menu)
      .text(MenuEvent::ShowConfigFolder, "Show config folder")
      .separator();

    // Add submenus for currently active windows.
    if !window_states.is_empty() {
      for (config_path, states) in window_states {
        let label = format!(
          "({}) {}",
          states.len(),
          Self::format_config_path(&self.config, &config_path)
        );

        tray_menu = tray_menu.item(&self.create_config_menu(
          &config_path,
          &label,
          !states.is_empty(),
          startup_config_paths.contains(&config_path),
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
      MenuEvent::ToggleWindowConfig { enable, path } => {
        info!(
          "Window config '{}' to be enabled: {}",
          path.display(),
          enable
        );

        match enable {
          true => {
            let window_config = config
              .window_config_by_path(&path)
              .await?
              .context("Window config not found.")?;

            window_factory.open(window_config).await
          }
          false => window_factory.close_by_path(&path).await,
        }
      }
      MenuEvent::ToggleStartupWindowConfig { enable, path } => {
        info!(
          "Window config '{}' to be launched on startup: {}",
          path.display(),
          enable
        );

        match enable {
          true => config.add_startup_config(&path).await,
          false => config.remove_startup_config(&path).await,
        }
      }
    }
  }

  /// Creates and returns a submenu for the window configs.
  async fn create_configs_menu(
    &self,
    window_states: &HashMap<PathBuf, Vec<WindowState>>,
    startup_config_paths: &Vec<PathBuf>,
  ) -> anyhow::Result<Submenu<Wry>> {
    let mut configs_menu =
      SubmenuBuilder::new(&self.app_handle, "Window configs");

    // Add each window config to the menu.
    for window_config in &self.config.window_configs().await {
      let label =
        Self::format_config_path(&self.config, &window_config.config_path);

      let menu_item = self.create_config_menu(
        &window_config.config_path,
        &label,
        window_states.contains_key(&window_config.config_path),
        startup_config_paths.contains(&window_config.config_path),
      )?;

      configs_menu = configs_menu.item(&menu_item);
    }

    Ok(configs_menu.build()?)
  }

  /// Creates and returns a submenu for the given window config.
  fn create_config_menu(
    &self,
    config_path: &PathBuf,
    label: &str,
    is_enabled: bool,
    is_launched_on_startup: bool,
  ) -> anyhow::Result<Submenu<Wry>> {
    let enabled_item = CheckMenuItem::with_id(
      &self.app_handle,
      MenuEvent::ToggleWindowConfig {
        enable: !is_enabled,
        path: config_path.clone(),
      },
      "Enabled",
      true,
      is_enabled,
      None::<&str>,
    )?;

    let startup_item = CheckMenuItem::with_id(
      &self.app_handle,
      MenuEvent::ToggleStartupWindowConfig {
        enable: !is_launched_on_startup,
        path: config_path.clone(),
      },
      "Launch on startup",
      true,
      is_launched_on_startup,
      None::<&str>,
    )?;

    let config_menu = SubmenuBuilder::new(&self.app_handle, label)
      .item(&enabled_item)
      .item(&startup_item);

    Ok(config_menu.build()?)
  }

  /// Formats the config path for display in the system tray.
  fn format_config_path(
    config: &Arc<Config>,
    config_path: &PathBuf,
  ) -> String {
    let path = config
      .strip_config_dir(config_path)
      .unwrap_or(config_path.clone())
      .to_unicode_string();

    path.strip_suffix(".zebar.json").unwrap_or(&path).into()
  }
}
