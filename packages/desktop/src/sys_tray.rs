use std::{collections::HashMap, path::PathBuf, str::FromStr, sync::Arc};

use anyhow::{bail, Context};
use tauri::{
  image::Image,
  menu::{CheckMenuItem, Menu, MenuBuilder, Submenu, SubmenuBuilder},
  tray::{
    MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder,
    TrayIconEvent,
  },
  AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder,
  Wry,
};
use tokio::{sync::Mutex, task};
use tracing::{error, info};

use crate::{
  common::PathExt,
  config::{Config, WidgetConfig, WidgetPreset},
  widget_factory::{WidgetFactory, WidgetOpenOptions, WidgetState},
};

#[derive(Debug, Clone)]
enum MenuEvent {
  ShowConfigFolder,
  ReloadConfigs,
  OpenSettings,
  Exit,
  ToggleWidgetPreset {
    enable: bool,
    preset: String,
    path: PathBuf,
  },
  ToggleStartupWidgetConfig {
    enable: bool,
    path: PathBuf,
  },
}

impl ToString for MenuEvent {
  fn to_string(&self) -> String {
    match self {
      MenuEvent::ShowConfigFolder => "show_config_folder".to_string(),
      MenuEvent::ReloadConfigs => "reload_configs".to_string(),
      MenuEvent::OpenSettings => "open_settings".to_string(),
      MenuEvent::Exit => "exit".to_string(),
      MenuEvent::ToggleWidgetPreset {
        enable,
        preset,
        path,
      } => {
        format!(
          "toggle_widget_config_{}_{}_{}",
          enable,
          preset,
          path.to_unicode_string(),
        )
      }
      MenuEvent::ToggleStartupWidgetConfig { enable, path } => {
        format!(
          "toggle_startup_widget_config_{}_{}",
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
      ["reload", "configs"] => Ok(Self::ReloadConfigs),
      ["open", "settings"] => Ok(Self::OpenSettings),
      ["exit"] => Ok(Self::Exit),
      ["toggle", "widget", "config", enable @ ("true" | "false"), preset, path @ ..] => {
        Ok(Self::ToggleWidgetPreset {
          enable: *enable == "true",
          preset: preset.to_string(),
          path: PathBuf::from(path.join("_")),
        })
      }
      ["toggle", "startup", "widget", "config", enable @ ("true" | "false"), path @ ..] => {
        Ok(Self::ToggleStartupWidgetConfig {
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
  widget_factory: Arc<WidgetFactory>,
  tray_icon: Option<TrayIcon>,
}

impl SysTray {
  /// Creates a new system tray icon for Zebar.
  pub async fn new(
    app_handle: &AppHandle,
    config: Arc<Config>,
    widget_factory: Arc<WidgetFactory>,
  ) -> anyhow::Result<Arc<SysTray>> {
    let mut sys_tray = Self {
      app_handle: app_handle.clone(),
      config,
      widget_factory,
      tray_icon: None,
    };

    sys_tray.tray_icon = Some(sys_tray.create_tray_icon().await?);

    Ok(Arc::new(sys_tray))
  }

  async fn create_tray_icon(&self) -> anyhow::Result<TrayIcon> {
    let tooltip = format!("Zebar v{}", env!("VERSION_NUMBER"));

    let tray_icon = TrayIconBuilder::with_id("tray")
      .icon(self.icon_image()?)
      .menu(&self.create_tray_menu().await?)
      .tooltip(tooltip)
      .on_menu_event({
        let config = self.config.clone();
        let widget_factory = self.widget_factory.clone();

        move |app_handle, event| {
          if let Ok(menu_event) = MenuEvent::from_str(event.id.as_ref()) {
            Self::handle_menu_event(
              menu_event,
              app_handle.clone(),
              config.clone(),
              widget_factory.clone(),
            );
          }
        }
      });

    // Show the settings window on left click (Windows-only).
    #[cfg(windows)]
    {
      let tray_icon =
        tray_icon.menu_on_left_click(false).on_tray_icon_event({
          let app_handle = self.app_handle.clone();
          let config = self.config.clone();
          let widget_factory = self.widget_factory.clone();

          move |_, event| {
            if let TrayIconEvent::Click {
              button: MouseButton::Left,
              button_state: MouseButtonState::Down,
              ..
            } = event
            {
              Self::handle_menu_event(
                MenuEvent::OpenSettings,
                app_handle.clone(),
                config.clone(),
                widget_factory.clone(),
              );
            }
          }
        });
    }

    Ok(tray_icon.build(&self.app_handle)?)
  }

  pub async fn refresh(&self) -> anyhow::Result<()> {
    info!("Updating system tray menu.");

    if let Some(tray_icon) = self.tray_icon.as_ref() {
      let tray_menu = self.create_tray_menu().await?;
      tray_icon.set_menu(Some(tray_menu))?;
    }

    Ok(())
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
    let widget_configs = self.config.widget_configs().await;
    let widget_states = self.widget_factory.states_by_config_path().await;
    let startup_configs = self.config.startup_widget_configs().await?;

    let configs_menu = self
      .create_configs_menu(
        &widget_configs,
        &widget_states,
        &startup_configs,
      )
      .await?;

    let mut tray_menu = MenuBuilder::new(&self.app_handle)
      .text(MenuEvent::OpenSettings, "Open settings")
      .item(&configs_menu)
      .text(MenuEvent::ReloadConfigs, "Reload configs")
      .separator();

    // Add submenus for currently active widget.
    if !widget_states.is_empty() {
      for (config_path, config) in &widget_configs {
        if let Some(_) = widget_states.get(config_path) {
          let config_menu = self.create_config_menu(
            &config_path,
            config,
            &widget_states,
            &startup_configs,
          )?;

          tray_menu = tray_menu.item(&config_menu);
        }
      }

      tray_menu = tray_menu.separator();
    }

    tray_menu = tray_menu.text(MenuEvent::Exit, "Exit");

    Ok(tray_menu.build()?)
  }

  /// Callback for system tray menu events.
  fn handle_menu_event(
    event: MenuEvent,
    app_handle: AppHandle,
    config: Arc<Config>,
    widget_factory: Arc<WidgetFactory>,
  ) {
    task::spawn(async move {
      info!("Received tray menu event: {:?}", event);

      let event_res = match event {
        MenuEvent::ShowConfigFolder => config
          .open_config_dir()
          .context("Failed to open config folder."),
        MenuEvent::ReloadConfigs => config.reload().await,
        MenuEvent::OpenSettings => Self::open_settings_window(&app_handle),
        MenuEvent::Exit => {
          app_handle.exit(0);
          Ok(())
        }
        MenuEvent::ToggleWidgetPreset {
          enable,
          path,
          preset,
        } => match enable {
          true => {
            widget_factory
              .start_widget(&path, &WidgetOpenOptions::Preset(preset))
              .await
          }
          false => widget_factory.stop_by_preset(&path, &preset).await,
        },
        MenuEvent::ToggleStartupWidgetConfig { enable, path } => {
          match enable {
            true => config.add_startup_config(&path).await,
            false => config.remove_startup_config(&path).await,
          }
        }
      };

      if let Err(err) = event_res {
        error!("Error handling menu event: {:?}", err);
      }
    });
  }

  fn open_settings_window(app_handle: &AppHandle) -> anyhow::Result<()> {
    // Get existing settings window if it's already open.
    let settings_window = app_handle.get_webview_window("settings");

    match &settings_window {
      None => {
        WebviewWindowBuilder::new(
          app_handle,
          "settings",
          WebviewUrl::default(),
        )
        .title("Settings - Zebar")
        .focused(true)
        .inner_size(900., 600.)
        .build()
        .context("Failed to build the settings window.")?;

        Ok(())
      }
      Some(window) => window
        .set_focus()
        .context("Failed to focus the settings window."),
    }
  }

  /// Creates and returns a submenu for the widget configs.
  async fn create_configs_menu(
    &self,
    widget_configs: &HashMap<PathBuf, WidgetConfig>,
    widget_states: &HashMap<PathBuf, Vec<WidgetState>>,
    startup_configs: &HashMap<PathBuf, WidgetConfig>,
  ) -> anyhow::Result<Submenu<Wry>> {
    let mut configs_menu =
      SubmenuBuilder::new(&self.app_handle, "Widget configs");

    // Add each widget config to the menu.
    for (config_path, widget_config) in widget_configs {
      let config_menu = self.create_config_menu(
        config_path,
        &widget_config,
        widget_states,
        startup_configs,
      )?;

      configs_menu = configs_menu.item(&config_menu);
    }

    Ok(configs_menu.build()?)
  }

  /// Creates and returns a submenu for the given widget config.
  fn create_config_menu(
    &self,
    config_path: &PathBuf,
    widget_config: &WidgetConfig,
    widget_states: &HashMap<PathBuf, Vec<WidgetState>>,
    startup_configs: &HashMap<PathBuf, WidgetConfig>,
  ) -> anyhow::Result<Submenu<Wry>> {
    let label = Self::format_config_path(&self.config, config_path);

    let mut presets_menu = SubmenuBuilder::new(&self.app_handle, label);

    // Add each widget config to the menu.
    for preset in &widget_config.presets {
      let preset_menu = self.create_preset_menu(
        config_path,
        &preset,
        // TODO: Pass correct enabled + launch on startup states.
        // widget_states.contains_key(&preset.config_path),
        // startup_config_paths.contains(&preset.config_path),
        false,
        false,
      )?;

      presets_menu = presets_menu.item(&preset_menu);
    }

    Ok(presets_menu.build()?)
  }

  /// Creates and returns a submenu for the given widget config.
  fn create_preset_menu(
    &self,
    config_path: &PathBuf,
    preset: &WidgetPreset,
    is_enabled: bool,
    is_launched_on_startup: bool,
  ) -> anyhow::Result<Submenu<Wry>> {
    let enabled_item = CheckMenuItem::with_id(
      &self.app_handle,
      MenuEvent::ToggleWidgetPreset {
        enable: !is_enabled,
        preset: preset.name.clone(),
        path: config_path.clone(),
      },
      "Enabled",
      true,
      is_enabled,
      None::<&str>,
    )?;

    let startup_item = CheckMenuItem::with_id(
      &self.app_handle,
      MenuEvent::ToggleStartupWidgetConfig {
        enable: !is_launched_on_startup,
        path: config_path.clone(),
      },
      "Launch on startup",
      true,
      is_launched_on_startup,
      None::<&str>,
    )?;

    let config_menu = SubmenuBuilder::new(&self.app_handle, &preset.name)
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
