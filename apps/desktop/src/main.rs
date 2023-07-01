// Prevents additional console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

#[tauri::command]
fn read_config_file(path: &str) -> String {
  fs::read(path)
    .expect("Unable to read file")
    .iter()
    .map(|&c| c as char)
    .collect::<String>()
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![read_config_file])
    .run(tauri::generate_context!())
    .expect("error while running Tauri application");
}
