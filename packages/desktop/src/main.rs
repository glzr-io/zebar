#![feature(async_closure)]
#![feature(iterator_try_collect)]

use std::{env, sync::Arc};

use anyhow::Context;
use clap::Parser;
use tauri::Manager;
use tokio::task;
use tracing::{error, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

use crate::{
  cli::{Cli, CliCommand, OutputMonitorsArgs},
  config::Config,
  monitor_state::MonitorState,
  providers::provider_manager::ProviderManager,
  sys_tray::SysTray,
  window_factory::WindowFactory,
};

mod cli;
mod commands;
mod common;
mod config;
mod monitor_state;
mod providers;
mod sys_tray;
mod window_factory;

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

      // Initialize `MonitorState` in Tauri state.
      let monitor_state = Arc::new(MonitorState::new(app.handle()));
      app.manage(monitor_state.clone());

      // Initialize `WindowFactory` in Tauri state.
      let window_factory =
        Arc::new(WindowFactory::new(app.handle(), monitor_state));
      app.manage(window_factory.clone());

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
                let window_factory_clone = window_factory_clone.clone();
                task::spawn(async move {
                  window_factory_clone.open(window_config).await;
                });
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
        _ => config.startup_window_configs()?,
      };

      task::spawn(async move {
        for window_config in window_configs {
          if let Err(err) = window_factory.open(window_config).await {
            error!("Failed to open window: {:?}", err);
          }
        }
      });

      app.handle().plugin(tauri_plugin_shell::init())?;
      app.handle().plugin(tauri_plugin_http::init())?;
      app.handle().plugin(tauri_plugin_dialog::init())?;

      // Initialize `ProviderManager` in Tauri state.
      let manager = Arc::new(ProviderManager::new(app.handle()));
      app.manage(manager);

      // Add application icon to system tray.
      let _ = SysTray::new(app.handle(), config);

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      commands::get_window_state,
      commands::open_window,
      commands::listen_provider,
      commands::unlisten_provider,
      commands::set_always_on_top,
      commands::set_skip_taskbar
    ])
    .run(tauri::generate_context!())?;

  Ok(())
}
