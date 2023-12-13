// Prevents additional console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(unix_sigpipe)]

use std::{
  collections::HashMap,
  env::{self},
  sync::Arc,
};

use anyhow::Result;
use clap::Parser;
use cli::{Cli, CliCommand};
use monitors::get_monitors_str;
use providers::{config::ProviderConfig, manager::ProviderManager};
use serde::Serialize;
use tauri::{
  AppHandle, Monitor, RunEvent, State, Window, WindowBuilder, WindowUrl,
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
            let window_count = Arc::new(Mutex::new(0));

            while let Some(create_args) = rx.recv().await {
              let mut window_count = window_count.lock().await;
              *window_count += 1;

              info!(
                "Creating window #{} '{}' with args: {:#?}",
                window_count, create_args.window_id, create_args.args
              );

              let window = WindowBuilder::new(
                &app_handle,
                format!("{}-{}", &create_args.window_id, window_count),
                WindowUrl::default(),
              )
              .title(format!("Zebar - {}", create_args.window_id))
              .inner_size(500., 500.)
              .decorations(false)
              .resizable(false)
              .build()
              .unwrap();

              let initial_state = get_initial_state(
                &app_handle,
                &window,
                create_args.args,
                create_args.env,
              )
              .unwrap();

              let initial_state_str =
                serde_json::to_string(&initial_state).unwrap();

              info!("Window initial state: {}", initial_state_str);

              _ = window.eval(&format!(
                "window.__ZEBAR_INIT_STATE={}",
                initial_state_str,
              ));
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

  app.run(|_, event| {
    if let RunEvent::ExitRequested { api, .. } = &event {
      // Keep the message loop running even if all windows are closed.
      api.prevent_exit();
    }
  })
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct InitialState {
  args: HashMap<String, String>,
  env: HashMap<String, String>,
  current_window: WindowInfo,
  current_monitor: Option<MonitorInfo>,
  primary_monitor: Option<MonitorInfo>,
  monitors: Vec<MonitorInfo>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MonitorInfo {
  name: String,
  x: i32,
  y: i32,
  width: u32,
  height: u32,
  scale_factor: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WindowInfo {
  x: i32,
  y: i32,
  width: u32,
  height: u32,
  scale_factor: f64,
}

fn get_initial_state(
  app_handle: &AppHandle,
  window: &Window,
  args: HashMap<String, String>,
  env: HashMap<String, String>,
) -> Result<InitialState> {
  let monitors = app_handle
    .available_monitors()?
    .iter()
    .map(|monitor| to_monitor_info(&monitor))
    .collect();

  Ok(InitialState {
    args,
    env,
    current_window: to_window_info(&window)?,
    current_monitor: window
      .current_monitor()?
      .map(|monitor| to_monitor_info(&monitor)),
    primary_monitor: app_handle
      .primary_monitor()?
      .map(|monitor| to_monitor_info(&monitor)),
    monitors,
  })
}

fn to_window_info(window: &Window) -> Result<WindowInfo> {
  let window_position = window.outer_position()?;
  let window_size = window.outer_size()?;

  Ok(WindowInfo {
    scale_factor: window.scale_factor()?,
    width: window_size.width,
    height: window_size.height,
    x: window_position.x,
    y: window_position.y,
  })
}

fn to_monitor_info(monitor: &Monitor) -> MonitorInfo {
  let monitor_position = monitor.position();
  let monitor_size = monitor.size();

  MonitorInfo {
    name: monitor.name().unwrap_or(&"".to_owned()).to_string(),
    scale_factor: monitor.scale_factor(),
    width: monitor_size.width,
    height: monitor_size.height,
    x: monitor_position.x,
    y: monitor_position.y,
  }
}
