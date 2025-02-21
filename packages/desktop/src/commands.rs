use std::{collections::HashMap, path::PathBuf, sync::Arc};

use serde::{Deserialize, Serialize};
use tauri::{State, Window};

#[cfg(target_os = "macos")]
use crate::common::macos::WindowExtMacOs;
#[cfg(target_os = "windows")]
use crate::common::windows::WindowExtWindows;
use crate::{
  config::{
    Config, CreateWidgetArgs, CreateWidgetPackArgs, WidgetConfig,
    WidgetPack, WidgetPlacement,
  },
  providers::{
    ProviderConfig, ProviderFunction, ProviderFunctionResponse,
    ProviderManager,
  },
  shell_state::{ShellCommandArgs, ShellState},
  widget_factory::{WidgetFactory, WidgetOpenOptions, WidgetState},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct StartWidgetArgs {
  pack_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct StartWidgetPresetArgs {
  pack_id: String,
}

#[tauri::command]
pub async fn widget_packs(
  config: State<'_, Arc<Config>>,
) -> Result<HashMap<String, WidgetPack>, String> {
  Ok(config.widget_packs().await)
}

#[tauri::command]
pub async fn widget_pack_by_id(
  pack_id: String,
  config: State<'_, Arc<Config>>,
) -> Result<Option<WidgetPack>, String> {
  Ok(config.widget_pack_by_id(&pack_id).await)
}

#[tauri::command]
pub async fn widget_states(
  widget_factory: State<'_, Arc<WidgetFactory>>,
) -> Result<HashMap<String, WidgetState>, String> {
  Ok(widget_factory.states().await)
}

#[tauri::command]
pub async fn start_widget(
  widget_name: String,
  placement: WidgetPlacement,
  args: StartWidgetArgs,
  widget_factory: State<'_, Arc<WidgetFactory>>,
) -> anyhow::Result<(), String> {
  widget_factory
    .start_widget(
      &args.pack_id,
      &widget_name,
      &WidgetOpenOptions::Standalone(placement),
    )
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn start_widget_preset(
  widget_name: String,
  preset_name: String,
  args: StartWidgetPresetArgs,
  widget_factory: State<'_, Arc<WidgetFactory>>,
) -> anyhow::Result<(), String> {
  widget_factory
    .start_widget(
      &args.pack_id,
      &widget_name,
      &WidgetOpenOptions::Preset(preset_name),
    )
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn stop_preset(
  config_path: String,
  preset_name: String,
  widget_factory: State<'_, Arc<WidgetFactory>>,
) -> anyhow::Result<(), String> {
  widget_factory
    .stop_by_preset(&PathBuf::from(config_path), &preset_name)
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn create_widget_pack(
  args: CreateWidgetPackArgs,
  config: State<'_, Arc<Config>>,
) -> anyhow::Result<WidgetPack, String> {
  config
    .create_widget_pack(args)
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn update_widget_pack(
  pack_id: String,
  args: CreateWidgetPackArgs,
  config: State<'_, Arc<Config>>,
) -> anyhow::Result<WidgetPack, String> {
  config
    .update_widget_pack(pack_id, args)
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn create_widget(
  args: CreateWidgetArgs,
  config: State<'_, Arc<Config>>,
) -> anyhow::Result<WidgetConfig, String> {
  config.create_widget(args).map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn update_widget_config(
  pack_id: String,
  widget_name: String,
  new_config: WidgetConfig,
  config: State<'_, Arc<Config>>,
) -> Result<(), String> {
  config
    .update_widget_config(&pack_id, &widget_name, new_config)
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn delete_widget(
  pack_name: String,
  widget_name: String,
  config: State<'_, Arc<Config>>,
) -> anyhow::Result<(), String> {
  config
    .delete_widget(&pack_name, &widget_name)
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
