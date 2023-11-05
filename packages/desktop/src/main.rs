// Prevents additional console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use providers::{config::ProviderConfig, manager::ProviderManager};
use tauri::{AppHandle, Manager, State};

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
  options_hash: String,
  options: ProviderConfig,
  tracked_access: Vec<String>,
  provider_manager: State<'_, ProviderManager>,
) -> Result<(), String> {
  provider_manager
    .listen(options_hash, options, tracked_access)
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command()]
async fn unlisten_provider(
  options_hash: String,
  provider_manager: State<'_, ProviderManager>,
) -> Result<(), String> {
  provider_manager
    .unlisten(options_hash)
    .await
    .map_err(|err| err.to_string())
}

#[tokio::main]
async fn main() {
  tauri::async_runtime::set(tokio::runtime::Handle::current());
  tracing_subscriber::fmt::init();

  tauri::Builder::default()
    .setup(|app| {
      match app.get_cli_matches() {
        Ok(matches) => {
          println!("{:?}", matches);
        }
        Err(_) => panic! {"CLI Parsing Error"},
      };
      Ok(())
    })
    .setup(providers::manager::init)
    .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
      println!("{}, {argv:?}, {cwd}", app.package_info().name);
      app
        .emit_all("single-instance", Payload { args: argv, cwd })
        .unwrap();
    }))
    .invoke_handler(tauri::generate_handler![
      read_config_file,
      listen_provider,
      unlisten_provider
    ])
    .run(tauri::generate_context!())
    .expect("Error while running Tauri application.");
}
