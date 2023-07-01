// Prevents additional console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod user_config;

#[tauri::command]
fn read_config_file() -> Result<String, String> {
  user_config::read_file().map_err(|err| err.to_string())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![read_config_file])
    .run(tauri::generate_context!())
    .expect("Error while running Tauri application.");
}
