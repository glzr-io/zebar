#![feature(async_closure)]
use std::{env, sync::Arc};

use anyhow::Context;
use clap::Parser;
use tauri::{Manager, State, Window};
use tracing::{error, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

use crate::{
  cli::{Cli, CliCommand, OutputMonitorsArgs},
  common::WindowExt,
  config::Config,
  monitor_state::MonitorState,
  providers::{config::ProviderConfig, provider_manager::ProviderManager},
  sys_tray::setup_sys_tray,
  window_factory::{WindowFactory, WindowState},
};

mod cli;
mod common;
mod config;
mod monitor_state;
mod providers;
mod sys_tray;
mod window_factory;

#[tauri::command]
async fn get_window_state(
  window_id: String,
  window_factory: State<'_, Arc<WindowFactory>>,
) -> anyhow::Result<Option<WindowState>, String> {
  Ok(window_factory.state_by_id(&window_id).await)
}

#[tauri::command]
async fn open_window(
  config_path: String,
  config: State<'_, Arc<Config>>,
  window_factory: State<'_, Arc<WindowFactory>>,
) -> anyhow::Result<(), String> {
  let window_config = config
    .window_config_by_abs_path(&config_path)
    .map_err(|err| err.to_string())?
    .context("Window config not found.")
    .map_err(|err| err.to_string())?;

  window_factory.open_one(window_config);

  Ok(())
}

#[tauri::command]
async fn listen_provider(
  config_hash: String,
  config: ProviderConfig,
  tracked_access: Vec<String>,
  provider_manager: State<'_, Arc<ProviderManager>>,
) -> anyhow::Result<(), String> {
  provider_manager
    .create(config_hash, config, tracked_access)
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
async fn unlisten_provider(
  config_hash: String,
  provider_manager: State<'_, Arc<ProviderManager>>,
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
async fn main() -> anyhow::Result<()> {
  let cli = Cli::parse();

  match cli.command() {
    CliCommand::Monitors(args) => output_monitors(args),
    _ => {
      let start_res = start_app(cli);

      // If unable to start Zebar, the error is fatal and a message dialog
      // is shown.
      if let Err(err) = &start_res {
        // TODO: Show error dialog.
        error!("{:?}", err);
      };

      start_res
    }
  }
}

/// Prints available monitors to console.
fn output_monitors(args: OutputMonitorsArgs) -> anyhow::Result<()> {
  let _ = tauri::Builder::default().setup(|app| {
    let monitors = MonitorState::new(app.handle());
    cli::print_and_exit(monitors.output_str(args));
    Ok(())
  });

  Ok(())
}

/// Starts Zebar - either with a specific window or all windows.
fn start_app(cli: Cli) -> anyhow::Result<()> {
  tracing_subscriber::fmt()
    .with_env_filter(
      EnvFilter::from_env("LOG_LEVEL")
        .add_directive(LevelFilter::INFO.into()),
    )
    .init();

  tauri::async_runtime::set(tokio::runtime::Handle::current());

  tauri::Builder::default()
    .setup(move |app| {
      // Initialize `Config` in Tauri state.
      let config = Arc::new(Config::new(app.handle())?);
      app.manage(config.clone());

      let window_factory = Arc::new(WindowFactory::new(app.handle()));

      // If this is not the first instance of the app, this will emit
      // within the original instance and exit immediately.
      let config_clone = config.clone();
      let window_factory_clone = window_factory.clone();
      app.handle().plugin(tauri_plugin_single_instance::init(
        move |_, args, _| {
          let cli = Cli::parse_from(args);

          // CLI command is guaranteed to be one of the open commands here.
          if let CliCommand::Open(args) = cli.command() {
            let window_config_res = config_clone
              .window_config_by_rel_path(&args.config_path)
              .and_then(|res| {
                res.ok_or_else(|| {
                  anyhow::anyhow!(
                    "Window config not found at {}.",
                    args.config_path
                  )
                })
              });

            match window_config_res {
              Ok(window_config) => {
                window_factory_clone.open_one(window_config);
              }
              Err(err) => {
                error!("{:?}", err);
              }
            }
          }
        },
      ))?;

      // Prevent windows from showing up in the dock on MacOS.
      #[cfg(target_os = "macos")]
      app.set_activation_policy(tauri::ActivationPolicy::Accessory);

      // Get window configs to open on start.
      let window_configs = match cli.command() {
        CliCommand::Open(args) => {
          let window_config = config
            .window_config_by_rel_path(&args.config_path)?
            .with_context(|| {
              format!("Window config not found at {}.", args.config_path)
            })?;

          vec![window_config]
        }
        _ => config.window_configs.clone(),
      };

      window_factory.open_all(window_configs);

      // Initialize `WindowFactory` in Tauri state.
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
      get_window_state,
      open_window,
      listen_provider,
      unlisten_provider,
      set_always_on_top,
      set_skip_taskbar
    ])
    .run(tauri::generate_context!())?;

  Ok(())
}
