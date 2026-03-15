use std::{collections::HashMap, sync::Arc};

use tauri::{State, Window};

#[cfg(target_os = "macos")]
use crate::common::macos::WindowExtMacOs;
#[cfg(target_os = "windows")]
use crate::common::windows::WindowExtWindows;
use crate::{
  marketplace_installer::MarketplaceInstaller,
  providers::{
    ProviderConfig, ProviderFunction, ProviderFunctionResponse,
    ProviderManager,
  },
  shell_state::{ShellCommandArgs, ShellState},
  widget_factory::{WidgetFactory, WidgetOpenOptions, WidgetState},
  widget_pack::{
    CreateWidgetConfigArgs, CreateWidgetPackArgs, UpdateWidgetPackArgs,
    WidgetConfig, WidgetPack, WidgetPackManager, WidgetPlacement,
  },
};

#[tauri::command]
pub async fn widget_packs(
  widget_pack_manager: State<'_, Arc<WidgetPackManager>>,
) -> Result<Vec<WidgetPack>, String> {
  Ok(
    widget_pack_manager
      .widget_packs()
      .await
      .values()
      .cloned()
      .collect(),
  )
}

#[tauri::command]
pub async fn widget_states(
  widget_factory: State<'_, Arc<WidgetFactory>>,
) -> Result<HashMap<String, WidgetState>, String> {
  Ok(widget_factory.states().await)
}

#[tauri::command]
pub async fn start_widget(
  pack_id: String,
  widget_name: String,
  placement: WidgetPlacement,
  is_preview: bool,
  widget_factory: State<'_, Arc<WidgetFactory>>,
) -> anyhow::Result<(), String> {
  widget_factory
    .start_widget_by_id(
      &pack_id,
      &widget_name,
      &WidgetOpenOptions::Standalone(placement),
      is_preview,
    )
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn start_widget_preset(
  pack_id: String,
  widget_name: String,
  preset_name: String,
  is_preview: bool,
  widget_factory: State<'_, Arc<WidgetFactory>>,
) -> anyhow::Result<(), String> {
  widget_factory
    .start_widget_by_id(
      &pack_id,
      &widget_name,
      &WidgetOpenOptions::Preset(preset_name),
      is_preview,
    )
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn stop_widget_preset(
  pack_id: String,
  widget_name: String,
  preset_name: String,
  widget_factory: State<'_, Arc<WidgetFactory>>,
) -> anyhow::Result<(), String> {
  widget_factory
    .stop_by_preset(&pack_id, &widget_name, &preset_name)
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn create_widget_pack(
  args: CreateWidgetPackArgs,
  widget_pack_manager: State<'_, Arc<WidgetPackManager>>,
) -> anyhow::Result<WidgetPack, String> {
  widget_pack_manager
    .create_widget_pack(args)
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn update_widget_pack(
  pack_id: String,
  args: UpdateWidgetPackArgs,
  widget_pack_manager: State<'_, Arc<WidgetPackManager>>,
) -> anyhow::Result<WidgetPack, String> {
  widget_pack_manager
    .update_widget_pack(&pack_id, args)
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn delete_widget_pack(
  pack_id: String,
  widget_pack_manager: State<'_, Arc<WidgetPackManager>>,
) -> anyhow::Result<(), String> {
  widget_pack_manager
    .delete_widget_pack(&pack_id)
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn create_widget_config(
  args: CreateWidgetConfigArgs,
  widget_pack_manager: State<'_, Arc<WidgetPackManager>>,
) -> anyhow::Result<WidgetConfig, String> {
  widget_pack_manager
    .create_widget_config(args)
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn update_widget_config(
  pack_id: String,
  widget_name: String,
  new_config: WidgetConfig,
  widget_pack_manager: State<'_, Arc<WidgetPackManager>>,
) -> Result<WidgetConfig, String> {
  widget_pack_manager
    .update_widget_config(&pack_id, &widget_name, new_config)
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn delete_widget_config(
  pack_id: String,
  widget_name: String,
  widget_pack_manager: State<'_, Arc<WidgetPackManager>>,
) -> anyhow::Result<(), String> {
  widget_pack_manager
    .delete_widget_config(&pack_id, &widget_name)
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
    .stop(config_hash)
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn call_provider_function(
  config_hash: String,
  function: ProviderFunction,
  provider_manager: State<'_, Arc<ProviderManager>>,
) -> anyhow::Result<ProviderFunctionResponse, String> {
  provider_manager
    .call_function(config_hash, function)
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn install_widget_pack(
  pack_id: String,
  version: String,
  tarball_url: String,
  is_preview: bool,
  marketplace_manager: State<'_, Arc<MarketplaceInstaller>>,
) -> anyhow::Result<WidgetPack, String> {
  marketplace_manager
    .install(&pack_id, &version, &tarball_url, is_preview)
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn start_preview_widget(
  pack_config: WidgetPack,
  widget_name: String,
  preset_name: String,
  widget_factory: State<'_, Arc<WidgetFactory>>,
) -> anyhow::Result<(), String> {
  widget_factory
    .start_widget_by_pack(
      &pack_config,
      &widget_name,
      &WidgetOpenOptions::Preset(preset_name),
      true,
    )
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn stop_all_preview_widgets(
  widget_factory: State<'_, Arc<WidgetFactory>>,
) -> anyhow::Result<(), String> {
  widget_factory
    .stop_all_previews()
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

#[tauri::command]
pub async fn shell_exec(
  program: String,
  args: ShellCommandArgs,
  options: shell_util::CommandOptions,
  window: Window,
  shell_state: State<'_, ShellState>,
) -> anyhow::Result<shell_util::ShellExecOutput, String> {
  let widget_id = window.label();
  shell_state
    .exec(&widget_id, &program, args, &options)
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn shell_spawn(
  program: String,
  args: ShellCommandArgs,
  options: shell_util::CommandOptions,
  window: Window,
  shell_state: State<'_, ShellState>,
) -> anyhow::Result<shell_util::ProcessId, String> {
  let widget_id = window.label();
  shell_state
    .spawn(&widget_id, &program, args, &options)
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn shell_write(
  pid: shell_util::ProcessId,
  buffer: shell_util::Buffer,
  shell_state: State<'_, ShellState>,
) -> anyhow::Result<(), String> {
  shell_state
    .write(pid, buffer)
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn shell_kill(
  pid: shell_util::ProcessId,
  shell_state: State<'_, ShellState>,
) -> anyhow::Result<(), String> {
  shell_state.kill(pid).map_err(|err| err.to_string())
}
