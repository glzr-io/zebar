// Prevents additional console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
  collections::HashMap,
  env::{self},
};

use clap::Parser;
use cli::{Cli, CliCommand};
use monitors::output_monitors;
use providers::{config::ProviderConfig, manager::ProviderManager};
use tauri::{AppHandle, RunEvent, State, WindowBuilder, WindowUrl};
use tokio::{sync::mpsc, task};
use tracing::info;

mod cli;
mod monitors;
mod providers;
mod user_config;

#[derive(Clone, Debug)]
struct CreateWindowArgs {
  window_id: String,
  args: HashMap<String, String>,
  env: HashMap<String, String>,
}

#[tauri::command]
fn read_config_file(
  config_path_override: Option<&str>,
  app_handle: AppHandle,
) -> Result<String, String> {
  user_config::read_file(config_path_override, app_handle)
    .map_err(|err| err.to_string())
}

#[tauri::command]
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

#[tauri::command]
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

  let app = tauri::Builder::default()
    .setup(|app| {
      let cli = Cli::parse();

      // Since most Tauri plugins and setup is not needed for the `monitors`
      // CLI command, the setup is conditional based on the CLI command.
      match cli.command {
        CliCommand::Monitors => {
          output_monitors(app)?;
          std::process::exit(0);
        }
        CliCommand::Open { window_id, args } => {
          let (tx, mut rx) = mpsc::unbounded_channel();

          let create_args = CreateWindowArgs {
            window_id,
            args: args.unwrap_or(vec![]).into_iter().collect(),
            env: env::vars().collect(),
          };

          _ = tx.send(create_args.clone());

          // If this is not the first instance of the app, emit the window to
          // create to the original instance and exit immediately.
          app.handle().plugin(tauri_plugin_single_instance::init(
            move |_, _, _| {
              _ = tx.send(create_args.clone());
            },
          ))?;

          app.handle().plugin(tauri_plugin_shell::init())?;
          app.handle().plugin(tauri_plugin_http::init())?;

          providers::manager::init(app)?;

          let app_handle = app.handle().clone();

          // Handle creation of new windows (both from the initial and
          // subsequent instances of the application)
          _ = task::spawn(async move {
            while let Some(create_args) = rx.recv().await {
              info!(
                "Creating window '{}' with args: {:#?}",
                create_args.window_id, create_args.args
              );

              _ = WindowBuilder::new(
                &app_handle,
                &create_args.window_id,
                WindowUrl::default(),
              )
              .title(format!("Zebar - {}", create_args.window_id))
              .inner_size(500., 500.)
              .decorations(false)
              .resizable(false)
              .build()
              .unwrap();
            }
          });

          Ok(())
        }
      }
    })
    .invoke_handler(tauri::generate_handler![
      read_config_file,
      listen_provider,
      unlisten_provider
    ])
    .build(tauri::generate_context!())
    .expect("Error while building Tauri application");

  app.run(|_app_handle, _event| {
    if let RunEvent::ExitRequested { api, .. } = &_event {
      // Keep the message loop running even if all windows are closed.
      api.prevent_exit();
    }
  })
}
