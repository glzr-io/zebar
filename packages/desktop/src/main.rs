// Prevents additional console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use providers::{config::ProviderConfig, manager::ProviderManager};
use tauri::{AppHandle, Manager, State};
use tauri_plugin_cli::CliExt;

mod providers;
mod user_config;

#[derive(Clone, serde::Serialize)]
struct Payload {
  args: Vec<String>,
  cwd: String,
}

#[tauri::command]
fn read_config_file(
  config_path_override: Option<&str>,
  app_handle: AppHandle,
) -> Result<String, String> {
  user_config::read_file(config_path_override, app_handle)
    .map_err(|err| err.to_string())
}

#[tauri::command()]
async fn listen_provider(
  config_hash: String,
  config: ProviderConfig,
  tracked_access: Vec<String>,
  provider_manager: State<'_, ProviderManager>,
) -> Result<(), String> {
  provider_manager
    .listen(config_hash, config, tracked_access)
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command()]
async fn unlisten_provider(
  config_hash: String,
  provider_manager: State<'_, ProviderManager>,
) -> Result<(), String> {
  provider_manager
    .unlisten(config_hash)
    .await
    .map_err(|err| err.to_string())
}

#[tokio::main]
async fn main() {
  tauri::async_runtime::set(tokio::runtime::Handle::current());
  tracing_subscriber::fmt::init();

  tauri::Builder::default()
    .plugin(tauri_plugin_cli::init())
    .setup(|app| {
      let _cli_matches = app.cli().matches()?;
      Ok(())
    })
    .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
      println!("{}, {argv:?}, {cwd}", app.package_info().name);
      app
        .emit("single-instance", Payload { args: argv, cwd })
        .unwrap();
    }))
    .setup(providers::manager::init)
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_http::init())
    .invoke_handler(tauri::generate_handler![
      read_config_file,
      listen_provider,
      unlisten_provider
    ])
    .run(tauri::generate_context!())
    .expect("Error while running Tauri application.");
}
