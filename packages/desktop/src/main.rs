// Prevents additional console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Manager};

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

fn main() {
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
    .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
      println!("{}, {argv:?}, {cwd}", app.package_info().name);
      app
        .emit_all("single-instance", Payload { args: argv, cwd })
        .unwrap();
    }))
    .invoke_handler(tauri::generate_handler![read_config_file])
    .run(tauri::generate_context!())
    .expect("Error while running Tauri application.");
}
