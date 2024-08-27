use std::sync::Arc;

use anyhow::Context;
use tauri::{State, Window};

use crate::{
  common::WindowExt,
  config::Config,
  providers::{config::ProviderConfig, provider_manager::ProviderManager},
  window_factory::{WindowFactory, WindowState},
};

#[tauri::command]
pub async fn get_window_state(
  window_id: String,
  window_factory: State<'_, Arc<WindowFactory>>,
) -> anyhow::Result<Option<WindowState>, String> {
  Ok(window_factory.state_by_id(&window_id).await)
}

#[tauri::command]
pub async fn open_window(
  config_path: String,
  config: State<'_, Arc<Config>>,
  window_factory: State<'_, Arc<WindowFactory>>,
) -> anyhow::Result<(), String> {
  let window_config = config
    .window_config_by_abs_path(&config_path)
    .map_err(|err| err.to_string())?
    .context("Window config not found.")
    .map_err(|err| err.to_string())?;

  window_factory.open_one(window_config).await;

  Ok(())
}

#[tauri::command]
pub async fn listen_provider(
  config_hash: String,
  config: ProviderConfig,
  tracked_access: Vec<String>,
  provider_manager: State<'_, Arc<ProviderManager>>,
) -> anyhow::Result<(), String> {
  provider_manager
    .create(config_hash, config, tracked_access)
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
