// Prevents additional console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(unix_sigpipe)]

use std::{
  collections::HashMap,
  env::{self},
  sync::Arc,
};

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Cli, CliCommand};
use monitors::get_monitors_str;
use providers::{config::ProviderConfig, manager::ProviderManager};
use serde::Serialize;
use tauri::{
  path::BaseDirectory, AppHandle, Manager, RunEvent, State, WindowBuilder,
  WindowUrl,
};
use tokio::{
  sync::{mpsc, Mutex},
  task,
};
use tracing::info;

mod cli;
mod monitors;
mod providers;
mod user_config;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct OpenWindowArgs {
  window_id: String,
  args: HashMap<String, String>,
  env: HashMap<String, String>,
}

struct OpenWindowArgsMap(Arc<Mutex<HashMap<String, OpenWindowArgs>>>);

#[tauri::command]
fn read_config_file(
  config_path_override: Option<&str>,
  app_handle: AppHandle,
) -> Result<String, String> {
  user_config::read_file(config_path_override, app_handle)
    .map_err(|err| err.to_string())
}

#[tauri::command]
async fn get_open_window_args(
  window_label: String,
  open_window_args_map: State<'_, OpenWindowArgsMap>,
) -> Result<Option<OpenWindowArgs>, String> {
  Ok(
    open_window_args_map
      .0
      .lock()
      .await
      .get(&window_label)
      .map(|open_args| open_args.clone()),
  )
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
#[unix_sigpipe = "sig_dfl"]
async fn main() {
  tauri::async_runtime::set(tokio::runtime::Handle::current());
  tracing_subscriber::fmt::init();

  let app = tauri::Builder::default()
    .setup(|app| {
      let cli = Cli::parse();

      // Since most Tauri plugins and setup is not needed for the `monitors`
      // CLI command, the setup is conditional based on the CLI command.
      match cli.command {
        CliCommand::Monitors { print0 } => {
          let monitors_str = get_monitors_str(app, print0);
          cli::print_and_exit(monitors_str);
          Ok(())
        }
        CliCommand::Open { window_id, args } => {
          let (tx, mut rx) = mpsc::unbounded_channel::<OpenWindowArgs>();
          let tx_clone = tx.clone();

          // If this is not the first instance of the app, this will emit
          // to the original instance and exit immediately.
          app.handle().plugin(tauri_plugin_single_instance::init(
            move |_, args, _| {
              let cli = Cli::parse_from(args);

              // CLI command is guaranteed to be an open command here.
              if let CliCommand::Open { window_id, args } = cli.command {
                _ = tx_clone.send(get_open_args(window_id, args));
              }
            },
          ))?;

          let open_args = get_open_args(window_id, args);

          if let Err(err) = tx.send(open_args.clone()) {
            info!("Error sending message: {}", err);
          }

          app.handle().plugin(tauri_plugin_shell::init())?;
          app.handle().plugin(tauri_plugin_http::init())?;

          providers::manager::init(app)?;

          let args_map = OpenWindowArgsMap(Default::default());
          let args_map_ref = args_map.0.clone();
          app.manage(args_map);

          let app_handle = app.handle().clone();

          // Handle creation of new windows (both from the initial and
          // subsequent instances of the application)
          _ = task::spawn(async move {
            let window_count = Arc::new(Mutex::new(0));

            while let Some(open_args) = rx.recv().await {
              let mut window_count = window_count.lock().await;
              *window_count += 1;

              info!(
                "Creating window #{} '{}' with args: {:#?}",
                window_count, open_args.window_id, open_args.args
              );

              // Window label needs to be globally unique. Hence add a prefix
              // with the window count to handle cases where multiple of the
              // same window are opened.
              let window_label =
                format!("{}-{}", window_count, &open_args.window_id);

              let window = WindowBuilder::new(
                &app_handle,
                &window_label,
                WindowUrl::default(),
              )
              .title(format!("Zebar - {}", open_args.window_id))
              .data_directory(
                // Set a different data dir for each window. Temporary fix
                // until #8196 is resolved.
                // Ref: https://github.com/tauri-apps/tauri/issues/8196
                app_handle
                  .path()
                  .resolve(
                    format!(".glazer/tmp-{}", window_count),
                    BaseDirectory::Home,
                  )
                  .context("Unable to get home directory.")
                  .unwrap(),
              )
              .inner_size(500., 500.)
              .decorations(false)
              .resizable(false)
              .build()
              .unwrap();

              _ = window.eval(&format!(
                "window.__ZEBAR_OPEN_ARGS={}",
                serde_json::to_string(&open_args).unwrap()
              ));

              let mut args_map = args_map_ref.lock().await;
              args_map.insert(window_label, open_args);
            }
          });

          Ok(())
        }
      }
    })
    .invoke_handler(tauri::generate_handler![
      read_config_file,
      get_open_window_args,
      listen_provider,
      unlisten_provider
    ])
    .build(tauri::generate_context!())
    .expect("Error while building Tauri application");

  app.run(|_, event| {
    if let RunEvent::ExitRequested { api, .. } = &event {
      // Keep the message loop running even if all windows are closed.
      api.prevent_exit();
    }
  })
}

fn get_open_args(
  window_id: String,
  args: Option<Vec<(String, String)>>,
) -> OpenWindowArgs {
  OpenWindowArgs {
    window_id,
    args: args.unwrap_or(vec![]).into_iter().collect(),
    env: env::vars().collect(),
  }
}
