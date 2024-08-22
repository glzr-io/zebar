#![feature(async_closure)]
use std::{collections::HashMap, env};

use clap::Parser;
use tauri::{Manager, State, Window};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

use crate::{
  cli::{Cli, CliCommand, OpenWindowArgs, OutputMonitorsArgs},
  monitors::get_monitors_str,
  providers::{config::ProviderConfig, provider_manager::ProviderManager},
  sys_tray::setup_sys_tray,
  util::WindowExt,
  window_factory::{WindowFactory, WindowState},
};

mod cli;
mod monitors;
mod providers;
mod sys_tray;
mod user_config;
mod util;
mod window_factory;

#[tauri::command]
async fn get_open_window_args(
  window_label: String,
  window_factory: State<'_, WindowFactory>,
) -> anyhow::Result<Option<WindowState>, String> {
  Ok(window_factory.state_by_window_label(window_label).await)
}

#[tauri::command]
fn open_window(
  config_path: String,
  args: HashMap<String, String>,
  window_factory: State<'_, WindowFactory>,
) -> anyhow::Result<(), String> {
  window_factory.try_open(OpenWindowArgs {
    config_path,
    args: Some(args.into_iter().collect()),
  });

  Ok(())
}

#[tauri::command]
async fn listen_provider(
  config_hash: String,
  config: ProviderConfig,
  tracked_access: Vec<String>,
  provider_manager: State<'_, ProviderManager>,
) -> anyhow::Result<(), String> {
  provider_manager
    .create(config_hash, config, tracked_access)
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
async fn unlisten_provider(
  config_hash: String,
  provider_manager: State<'_, ProviderManager>,
) -> anyhow::Result<(), String> {
  provider_manager
    .destroy(config_hash)
    .await
    .map_err(|err| err.to_string())
}

/// Tauri's implementation of `always_on_top` places the window above
/// all normal windows (but not the MacOS menu bar). The following instead
/// sets the z-order of the window to be above the menu bar.
#[tauri::command]
fn set_always_on_top(window: Window) -> anyhow::Result<(), String> {
  #[cfg(target_os = "macos")]
  let res = window.set_above_menu_bar();

  #[cfg(not(target_os = "macos"))]
  let res = window.set_always_on_top(true);

  res.map_err(|err| err.to_string())
}

#[tauri::command]
fn set_skip_taskbar(
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

/// Main entry point for the application.
///
/// Conditionally starts Zebar or runs a CLI command based on the given
/// subcommand.
#[tokio::main]
async fn main() {
  let cli = Cli::parse();

  match cli.command {
    CliCommand::Monitors(args) => output_monitors(args),
    _ => start_app(cli),
  }
}

/// Prints available monitors to console.
fn output_monitors(args: OutputMonitorsArgs) {
  tauri::Builder::default().setup(|app| {
    let monitors_str = get_monitors_str(app, args);
    cli::print_and_exit(monitors_str);
    Ok(())
  });
}

/// Starts Zebar - either with a specific window or all windows.
fn start_app(cli: Cli) {
  tracing_subscriber::fmt()
    .with_env_filter(
      EnvFilter::from_env("LOG_LEVEL")
        .add_directive(LevelFilter::INFO.into()),
    )
    .init();

  tauri::async_runtime::set(tokio::runtime::Handle::current());

  tauri::Builder::default()
    .setup(|app| {
      // If this is not the first instance of the app, this will
      // emit within the original instance and exit
      // immediately.
      app.handle().plugin(tauri_plugin_single_instance::init(
        move |app, args, _| {
          let cli = Cli::parse_from(args);

          // CLI command is guaranteed to be an open command here.
          if let CliCommand::Open(args) = cli.command {
            app.state::<WindowFactory>().try_open(args);
          }
        },
      ))?;

      let config = user_config::read_file(None, app.handle())?;

      // Prevent windows from showing up in the dock on MacOS.
      #[cfg(target_os = "macos")]
      app.set_activation_policy(tauri::ActivationPolicy::Accessory);

      // Open window with the given args and initialize
      // `WindowFactory` in Tauri state.
      let window_factory = WindowFactory::new(app.handle());
      window_factory.try_open(open_args);
      app.manage(window_factory);

      app.handle().plugin(tauri_plugin_shell::init())?;
      app.handle().plugin(tauri_plugin_http::init())?;
      app.handle().plugin(tauri_plugin_dialog::init())?;

      // Initialize `ProviderManager` in Tauri state.
      let mut manager = ProviderManager::new();
      manager.init(app.handle());
      app.manage(manager);

      // Add application icon to system tray.
      setup_sys_tray(app)?;

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      get_open_window_args,
      open_window,
      listen_provider,
      unlisten_provider,
      set_always_on_top,
      set_skip_taskbar
    ])
    .run(tauri::generate_context!())
    .expect("Failed to build Tauri application.");
}
