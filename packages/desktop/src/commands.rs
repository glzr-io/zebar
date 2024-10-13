use std::{path::PathBuf, sync::Arc};

use tauri::{State, Window};

use crate::{
  common::WindowExt,
  config::{Config, WidgetConfigEntry},
  providers::{ProviderConfig, ProviderManager},
  widget_factory::WidgetFactory,
};

#[tauri::command]
pub async fn widget_configs(
  config: State<'_, Arc<Config>>,
) -> Result<Vec<WidgetConfigEntry>, String> {
  Ok(config.widget_configs().await)
}

#[tauri::command]
pub async fn open_widget_default(
  config_path: String,
  config: State<'_, Arc<Config>>,
  widget_factory: State<'_, Arc<WidgetFactory>>,
) -> anyhow::Result<(), String> {
  let widget_config = config
    .widget_config_by_path(&PathBuf::from(config_path))
    .await
    .and_then(|opt| {
      opt.ok_or_else(|| anyhow::anyhow!("Widget config not found."))
    })
    .map_err(|err| err.to_string())?;

  widget_factory
    .open(widget_config)
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn listen_provider(
  config_hash: String,
  config: ProviderConfig,
  provider_manager: State<'_, Arc<ProviderManager>>,
) -> anyhow::Result<(), String> {
  provider_manager
    .create(config_hash, config)
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn unlisten_provider(
  config_hash: String,
  provider_manager: State<'_, Arc<ProviderManager>>,
) -> anyhow::Result<(), String> {
  provider_manager
    .destroy(config_hash)
    .await
    .map_err(|err| err.to_string())
}

/// Tauri's implementation of `always_on_top` places the window above
/// all normal windows (but not the MacOS menu bar). The following instead
/// sets the z-order of the window to be above the menu bar.
#[tauri::command]
pub fn set_always_on_top(window: Window) -> anyhow::Result<(), String> {
  #[cfg(target_os = "macos")]
  let res = window.set_above_menu_bar();

  #[cfg(not(target_os = "macos"))]
  let res = window.set_always_on_top(true);

  res.map_err(|err| err.to_string())
}

#[tauri::command]
pub fn set_skip_taskbar(
  window: Window,
  skip: bool,
) -> anyhow::Result<(), String> {
  window
    .set_skip_taskbar(skip)
    .map_err(|err| err.to_string())?;

  #[cfg(target_os = "windows")]
  window
    .set_tool_window(skip)
    .map_err(|err| err.to_string())?;

  Ok(())
}
