use std::{
  collections::HashMap,
  fmt::{self, Display},
  str::FromStr,
  sync::Arc,
};

use anyhow::{bail, Context};
use tauri::{
  image::Image,
  menu::{CheckMenuItem, Menu, MenuBuilder, Submenu, SubmenuBuilder},
  tray::{
    MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder,
    TrayIconEvent,
  },
  AppHandle, Manager, WebviewUrl, WebviewWindowBuilder, Wry,
};
use tokio::task;
use tracing::{error, info};

use crate::{
  app_settings::{AppSettings, StartupConfig, VERSION_NUMBER},
  config::{Config, WidgetConfigEntry, WidgetPack},
  widget_factory::{WidgetFactory, WidgetOpenOptions, WidgetState},
};

#[derive(Debug, Clone)]
enum MenuEvent {
  ShowConfigFolder,
  ReloadConfigs,
  OpenSettings,
  BrowseWidgets,
  Exit,
  EditWidgetPack {
    pack_id: String,
  },
  EditWidget {
    pack_id: String,
    widget_name: String,
  },
  ToggleWidgetPreset {
    enable: bool,
    pack_id: String,
    widget_name: String,
    preset: String,
  },
  ToggleStartupWidgetConfig {
    enable: bool,
    pack_id: String,
    widget_name: String,
    preset: String,
  },
}

impl Display for MenuEvent {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      MenuEvent::ShowConfigFolder => write!(f, "show_config_folder"),
      MenuEvent::ReloadConfigs => write!(f, "reload_configs"),
      MenuEvent::OpenSettings => write!(f, "open_settings"),
      MenuEvent::BrowseWidgets => write!(f, "browse_widgets"),
      MenuEvent::Exit => write!(f, "exit"),
      MenuEvent::EditWidgetPack { pack_id } => {
        write!(f, "edit_widget_pack_{}", pack_id)
      }
      MenuEvent::EditWidget {
        pack_id,
        widget_name,
      } => {
        write!(f, "edit_widget_{}_{}", pack_id, widget_name)
      }
      MenuEvent::ToggleWidgetPreset {
        enable,
        preset,
        pack_id,
        widget_name,
      } => {
        write!(
          f,
          "toggle_widget_config_{}_{}_{}_{}",
          enable, preset, pack_id, widget_name
        )
      }
      MenuEvent::ToggleStartupWidgetConfig {
        enable,
        preset,
        pack_id,
        widget_name,
      } => {
        write!(
          f,
          "toggle_startup_widget_config_{}_{}_{}_{}",
          enable, preset, pack_id, widget_name
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
      ["browse", "widgets"] => Ok(Self::BrowseWidgets),
      ["exit"] => Ok(Self::Exit),
      ["edit", "widget", "pack", pack_id] => Ok(Self::EditWidgetPack {
        pack_id: pack_id.to_string(),
      }),
      ["edit", "widget", pack_id, widget_name] => Ok(Self::EditWidget {
        pack_id: pack_id.to_string(),
        widget_name: widget_name.to_string(),
      }),
      ["toggle", "widget", "config", enable @ ("true" | "false"), preset, pack_id, widget_name] => {
        Ok(Self::ToggleWidgetPreset {
          enable: *enable == "true",
          preset: preset.to_string(),
          pack_id: pack_id.to_string(),
          widget_name: widget_name.to_string(),
        })
      }
      ["toggle", "startup", "widget", "config", enable @ ("true" | "false"), preset, pack_id, widget_name] => {
        Ok(Self::ToggleStartupWidgetConfig {
          enable: *enable == "true",
          preset: preset.to_string(),
          pack_id: pack_id.to_string(),
          widget_name: widget_name.to_string(),
        })
      }
      _ => bail!("Invalid menu event: {}", event),
    }
  }
}

enum SettingsRoute {
  Index,
  WidgetPack {
    pack_id: String,
  },
  Widget {
    pack_id: String,
    widget_name: String,
  },
  Marketplace,
}

/// System tray icon for Zebar.
pub struct SysTray {
  app_handle: AppHandle,
  app_settings: Arc<AppSettings>,
  config: Arc<Config>,
  widget_factory: Arc<WidgetFactory>,
  tray_icon: Option<TrayIcon>,
}

impl SysTray {
  /// Creates a new system tray icon for Zebar.
  pub async fn new(
    app_handle: &AppHandle,
    app_settings: Arc<AppSettings>,
    config: Arc<Config>,
    widget_factory: Arc<WidgetFactory>,
  ) -> anyhow::Result<SysTray> {
    let mut sys_tray = Self {
      app_handle: app_handle.clone(),
      app_settings,
      config,
      widget_factory,
      tray_icon: None,
    };

    sys_tray.tray_icon = Some(sys_tray.create_tray_icon().await?);

    Ok(sys_tray)
  }

  async fn create_tray_icon(&self) -> anyhow::Result<TrayIcon> {
    let tooltip = format!("Zebar v{}", VERSION_NUMBER);

    // Linting: `mut` needed for Windows where `tray_icon` is modified with
    // additional click handler.
    #[allow(unused_mut)]
    let mut tray_icon = TrayIconBuilder::with_id("tray")
      .icon(self.icon_image()?)
      .menu(&self.create_tray_menu().await?)
      .tooltip(tooltip)
      .on_menu_event({
        let app_settings = self.app_settings.clone();
        let config = self.config.clone();
        let widget_factory = self.widget_factory.clone();

        move |app_handle, event| {
          if let Ok(menu_event) = MenuEvent::from_str(event.id.as_ref()) {
            Self::handle_menu_event(
              menu_event,
              app_handle.clone(),
              app_settings.clone(),
              config.clone(),
              widget_factory.clone(),
            );
          }
        }
      });

    // Show the settings window on left click (Windows-only).
    #[cfg(windows)]
    {
      tray_icon = tray_icon
        .show_menu_on_left_click(false)
        .on_tray_icon_event({
          let app_handle = self.app_handle.clone();
          let app_settings = self.app_settings.clone();
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
                app_settings.clone(),
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
    let widget_packs = self.config.widget_packs().await;
    let widget_states = self.widget_factory.states().await;
    let startup_configs = self.app_settings.startup_configs().await;

    let configs_menu = self.create_packs_menu(
      &widget_packs,
      &widget_states,
      &startup_configs,
    )?;

    let mut tray_menu = MenuBuilder::new(&self.app_handle)
      .text(MenuEvent::OpenSettings, "Open settings")
      .text(MenuEvent::BrowseWidgets, "Browse widgets...")
      .item(&configs_menu)
      .text(MenuEvent::ReloadConfigs, {
        #[cfg(windows)]
        {
          // Windows needs to triple escape ampersands.
          "Empty cache &&& reload configs"
        }
        #[cfg(not(windows))]
        {
          "Empty cache & reload configs"
        }
      })
      .separator();

    // Add submenus for currently active widget packs.
    if !widget_states.is_empty() {
      for (pack_id, pack) in &widget_packs {
        if widget_states
          .values()
          .any(|state| state.pack_id == *pack_id)
        {
          let pack_menu = self.create_pack_menu(
            pack,
            &widget_states,
            &startup_configs,
          )?;

          tray_menu = tray_menu.item(&pack_menu);
        }
      }

      tray_menu = tray_menu.separator();
    }

    let tray_menu = tray_menu.text(MenuEvent::Exit, "Exit").build()?;

    // Set "Open settings" as the default menu item on Windows.
    #[cfg(windows)]
    {
      use tauri::menu::ContextMenu;
      use windows::Win32::UI::WindowsAndMessaging::{
        SetMenuDefaultItem, HMENU,
      };

      let _ = unsafe {
        SetMenuDefaultItem(
          HMENU(tray_menu.hpopupmenu()? as _),
          0_u32,
          true.into(),
        )
      };
    }

    Ok(tray_menu)
  }

  /// Callback for system tray menu events.
  fn handle_menu_event(
    event: MenuEvent,
    app_handle: AppHandle,
    app_settings: Arc<AppSettings>,
    config: Arc<Config>,
    widget_factory: Arc<WidgetFactory>,
  ) {
    task::spawn(async move {
      info!("Received tray menu event: {:?}", event);

      let event_res = match event {
        MenuEvent::ShowConfigFolder => app_settings
          .open_config_dir()
          .context("Failed to open config folder."),
        MenuEvent::ReloadConfigs => {
          widget_factory.clear_cache();

          // TODO: Error handling.
          let _ = app_settings.reload().await;
          config.reload().await
        }
        MenuEvent::OpenSettings => {
          Self::open_settings_window(&app_handle, SettingsRoute::Index)
        }
        MenuEvent::BrowseWidgets => Self::open_settings_window(
          &app_handle,
          SettingsRoute::Marketplace,
        ),
        MenuEvent::Exit => {
          app_handle.exit(0);
          Ok(())
        }
        MenuEvent::EditWidget {
          pack_id,
          widget_name,
        } => Self::open_settings_window(
          &app_handle,
          SettingsRoute::Widget {
            pack_id,
            widget_name,
          },
        ),
        MenuEvent::EditWidgetPack { pack_id } => {
          Self::open_settings_window(
            &app_handle,
            SettingsRoute::WidgetPack { pack_id },
          )
        }
        MenuEvent::ToggleWidgetPreset {
          enable,
          pack_id,
          widget_name,
          preset,
        } => match enable {
          true => {
            widget_factory
              .start_widget(
                &pack_id,
                &widget_name,
                &WidgetOpenOptions::Preset(preset),
              )
              .await
          }
          false => {
            widget_factory
              .stop_by_preset(&pack_id, &widget_name, &preset)
              .await
          }
        },
        MenuEvent::ToggleStartupWidgetConfig {
          enable,
          preset,
          pack_id,
          widget_name,
        } => match enable {
          true => {
            app_settings
              .add_startup_config(&pack_id, &widget_name, &preset)
              .await
          }
          false => {
            app_settings
              .remove_startup_config(&pack_id, &widget_name, &preset)
              .await
          }
        },
      };

      if let Err(err) = event_res {
        error!("Error handling menu event: {:?}", err);
      }
    });
  }

  fn open_settings_window(
    app_handle: &AppHandle,
    route: SettingsRoute,
  ) -> anyhow::Result<()> {
    // Get existing settings window if it's already open.
    let settings_window = app_handle.get_webview_window("settings");

    let route = match route {
      SettingsRoute::Index => "/index.html".to_string(),
      SettingsRoute::WidgetPack { pack_id } => {
        format!("/index.html#/packs/{}", pack_id)
      }
      SettingsRoute::Widget {
        pack_id,
        widget_name,
      } => {
        format!("/index.html#/packs/{}/{}", pack_id, widget_name)
      }
      SettingsRoute::Marketplace => "/index.html#/marketplace".to_string(),
    };

    match &settings_window {
      None => {
        let window = WebviewWindowBuilder::new(
          app_handle,
          "settings",
          WebviewUrl::App(route.into()),
        )
        .title("Settings - Zebar")
        .focused(true)
        .visible(true)
        .inner_size(900., 600.)
        .build()
        .context("Failed to build the settings window.")?;

        // Ensure window is shown and activated.
        window.show()?;
        window.set_focus()?;

        Ok(())
      }
      Some(window) => {
        window
          .eval(&format!("location.replace('{}')", route))
          .context("Failed to navigate to widget edit page.")?;

        window
          .set_focus()
          .context("Failed to focus the settings window.")?;

        Ok(())
      }
    }
  }

  /// Creates and returns a submenu for the widget packs.
  fn create_packs_menu(
    &self,
    widget_packs: &HashMap<String, WidgetPack>,
    widget_states: &HashMap<String, WidgetState>,
    startup_configs: &[StartupConfig],
  ) -> anyhow::Result<Submenu<Wry>> {
    let mut packs_menu =
      SubmenuBuilder::new(&self.app_handle, "Widget packs");

    // Add each widget config to the menu.
    for pack in widget_packs.values() {
      let pack_menu =
        self.create_pack_menu(pack, widget_states, startup_configs)?;

      packs_menu = packs_menu.item(&pack_menu);
    }

    Ok(packs_menu.build()?)
  }

  /// Creates and returns a submenu for the given widget pack.
  fn create_pack_menu(
    &self,
    pack: &WidgetPack,
    widget_states: &HashMap<String, WidgetState>,
    startup_configs: &[StartupConfig],
  ) -> anyhow::Result<Submenu<Wry>> {
    let pack_states = widget_states
      .values()
      .filter(|state| state.pack_id == pack.id)
      .collect::<Vec<_>>();

    let label = match pack_states.len() {
      0 => pack.id.clone(),
      _ => format!("({}) {}", pack_states.len(), pack.id),
    };

    let mut pack_menu = SubmenuBuilder::new(&self.app_handle, label);

    // Add each widget config within the pack as a submenu.
    if !pack.widget_configs.is_empty() {
      for config_entry in &pack.widget_configs {
        let widget_menu = self.create_widget_menu(
          &pack.id,
          config_entry,
          &pack_states,
          startup_configs,
        )?;

        pack_menu = pack_menu.item(&widget_menu);
      }

      pack_menu = pack_menu.separator();
    }

    pack_menu = pack_menu.text(
      MenuEvent::EditWidgetPack {
        pack_id: pack.id.clone(),
      },
      "Settings",
    );

    Ok(pack_menu.build()?)
  }

  /// Creates and returns a submenu for the given widget config.
  fn create_widget_menu(
    &self,
    pack_id: &String,
    config_entry: &WidgetConfigEntry,
    pack_states: &[&WidgetState],
    startup_configs: &[StartupConfig],
  ) -> anyhow::Result<Submenu<Wry>> {
    let widget_states = pack_states
      .iter()
      .filter(|state| state.name == config_entry.value.name)
      .collect::<Vec<_>>();

    let label = match widget_states.len() {
      0 => config_entry.value.name.clone(),
      _ => {
        format!("({}) {}", widget_states.len(), config_entry.value.name)
      }
    };

    let mut widget_menu = SubmenuBuilder::new(&self.app_handle, label);

    // Add a toggle for starting/stopping each preset of the widget.
    if !config_entry.value.presets.is_empty() {
      for preset in &config_entry.value.presets {
        let preset_states = widget_states
          .iter()
          .filter(|state| {
            state.open_options
              == WidgetOpenOptions::Preset(preset.name.clone())
          })
          .collect::<Vec<_>>();

        let label = match preset_states.len() {
          0 => format!("Run '{}'", preset.name),
          _ => format!("({}) Run '{}'", preset_states.len(), preset.name),
        };

        let toggle_preset_item = CheckMenuItem::with_id(
          &self.app_handle,
          MenuEvent::ToggleWidgetPreset {
            enable: preset_states.is_empty(),
            preset: preset.name.clone(),
            pack_id: pack_id.clone(),
            widget_name: config_entry.value.name.clone(),
          },
          label,
          true,
          !preset_states.is_empty(),
          None::<&str>,
        )?;

        widget_menu = widget_menu.item(&toggle_preset_item);
      }

      widget_menu = widget_menu.separator();
    }

    widget_menu = widget_menu.text(
      MenuEvent::EditWidget {
        pack_id: pack_id.clone(),
        widget_name: config_entry.value.name.clone(),
      },
      "Settings",
    );

    // Add a submenu for toggling the widget's presets on startup.
    if !config_entry.value.presets.is_empty() {
      let mut widget_startup_menu =
        SubmenuBuilder::new(&self.app_handle, "Run on start-up");

      for preset in &config_entry.value.presets {
        let is_launched_on_startup =
          startup_configs.iter().any(|config| {
            config.pack_id == *pack_id
              && config.widget_name == config_entry.value.name
              && config.preset == preset.name
          });

        let toggle_startup_item = CheckMenuItem::with_id(
          &self.app_handle,
          MenuEvent::ToggleStartupWidgetConfig {
            enable: !is_launched_on_startup,
            pack_id: pack_id.clone(),
            widget_name: config_entry.value.name.clone(),
            preset: preset.name.clone(),
          },
          preset.name.clone(),
          true,
          is_launched_on_startup,
          None::<&str>,
        )?;

        widget_startup_menu =
          widget_startup_menu.item(&toggle_startup_item);
      }

      widget_menu = widget_menu.item(&widget_startup_menu.build()?);
    }

    Ok(widget_menu.build()?)
  }
}
