// Prevent additional console window on Windows in release mode.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(async_closure)]
#![feature(iterator_try_collect)]

use std::{env, sync::Arc};

use anyhow::Context;
use clap::Parser;
use tauri::{async_runtime::block_on, Manager, RunEvent};
use tokio::task;
use tracing::{error, info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

use crate::{
  cli::{Cli, CliCommand, OutputMonitorsArgs},
  config::Config,
  monitor_state::MonitorState,
  providers::ProviderManager,
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
  // Attach to parent console on Windows in release mode.
  #[cfg(all(windows, not(debug_assertions)))]
  {
    use windows::Win32::System::Console::{
      AttachConsole, ATTACH_PARENT_PROCESS,
    };
    let _ = unsafe { AttachConsole(ATTACH_PARENT_PROCESS) };
  }

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

  let app = tauri::Builder::default()
    .setup(move |app| {
      task::block_in_place(|| {
        block_on(async move {
          let config_dir_override = match cli.command() {
            CliCommand::Open(args) => args.config_dir,
            CliCommand::OpenAll(args) => args.config_dir,
            _ => None,
          };

          // Initialize `Config` in Tauri state.
          let config =
            Arc::new(Config::new(app.handle(), config_dir_override)?);
          app.manage(config.clone());

          // Initialize `MonitorState` in Tauri state.
          let monitor_state = Arc::new(MonitorState::new(app.handle()));
          app.manage(monitor_state.clone());

          // Initialize `WindowFactory` in Tauri state.
          let window_factory = Arc::new(WindowFactory::new(
            app.handle(),
            config.clone(),
            monitor_state.clone(),
          ));
          app.manage(window_factory.clone());

          // If this is not the first instance of the app, this will emit
          // within the original instance and exit immediately. The CLI
          // command is guaranteed to be one of the open commands here.
          setup_single_instance(
            app,
            config.clone(),
            window_factory.clone(),
          )?;

          // Prevent windows from showing up in the dock on MacOS.
          #[cfg(target_os = "macos")]
          app.set_activation_policy(tauri::ActivationPolicy::Accessory);

          // Allow assets to be resolved from the config directory.
          app
            .asset_protocol_scope()
            .allow_directory(&config.config_dir, true)?;

          app.handle().plugin(tauri_plugin_shell::init())?;
          app.handle().plugin(tauri_plugin_http::init())?;
          app.handle().plugin(tauri_plugin_dialog::init())?;

          // Initialize `ProviderManager` in Tauri state.
          let manager = Arc::new(ProviderManager::new(app.handle()));
          app.manage(manager);

          // Open windows based on CLI command.
          open_windows_by_cli_command(
            cli,
            config.clone(),
            window_factory.clone(),
          )
          .await?;

          // Add application icon to system tray.
          let tray = SysTray::new(
            app.handle(),
            config.clone(),
            window_factory.clone(),
          )
          .await?;

          listen_events(config, monitor_state, window_factory, tray);

          Ok(())
        })
      })
    })
    .invoke_handler(tauri::generate_handler![
      commands::start_widget,
      commands::listen_provider,
      commands::unlisten_provider,
      commands::set_always_on_top,
      commands::set_skip_taskbar
    ])
    .build(tauri::generate_context!())?;

  app.run(|_, event| {
    if let RunEvent::ExitRequested { code, api, .. } = &event {
      if code.is_none() {
        // Keep the message loop running even if all windows are closed.
        api.prevent_exit();
      }
    }
  });

  Ok(())
}

fn listen_events(
  config: Arc<Config>,
  monitor_state: Arc<MonitorState>,
  window_factory: Arc<WindowFactory>,
  tray: Arc<SysTray>,
) {
  let mut window_open_rx = window_factory.open_tx.subscribe();
  let mut window_close_rx = window_factory.close_tx.subscribe();
  let mut settings_change_rx = config.settings_change_tx.subscribe();
  let mut monitors_change_rx = monitor_state.change_tx.subscribe();
  let mut window_configs_change_rx =
    config.window_configs_change_tx.subscribe();

  task::spawn(async move {
    loop {
      let res = tokio::select! {
        Ok(_) = window_open_rx.recv() => {
          info!("Window opened.");
          tray.refresh().await
        },
        Ok(_) = window_close_rx.recv() => {
          info!("Window closed.");
          tray.refresh().await
        },
        Ok(_) = settings_change_rx.recv() => {
          info!("Settings changed.");
          tray.refresh().await
        },
        Ok(_) = monitors_change_rx.recv() => {
          info!("Monitors changed.");
          window_factory.relaunch_all().await
        },
        Ok(_) = window_configs_change_rx.recv() => {
          info!("Window configs changed.");
          window_factory.relaunch_all().await
        },
      };

      if let Err(err) = res {
        error!("{:?}", err);
      }
    }
  });
}

/// Setup single instance Tauri plugin.
fn setup_single_instance(
  app: &tauri::App,
  config: Arc<Config>,
  window_factory: Arc<WindowFactory>,
) -> anyhow::Result<()> {
  app.handle().plugin(tauri_plugin_single_instance::init(
    move |_, args, _| {
      let config = config.clone();
      let window_factory = window_factory.clone();

      task::spawn(async move {
        let res = match Cli::try_parse_from(args) {
          Ok(cli) => {
            // No-op if no subcommand is provided.
            if cli.command() != CliCommand::Empty {
              open_windows_by_cli_command(cli, config, window_factory)
                .await
            } else {
              Ok(())
            }
          }
          _ => Err(anyhow::anyhow!("Failed to parse CLI arguments.")),
        };

        if let Err(err) = res {
          error!("{:?}", err);
        }
      });
    },
  ))?;

  Ok(())
}

/// Opens windows based on CLI command.
async fn open_windows_by_cli_command(
  cli: Cli,
  config: Arc<Config>,
  window_factory: Arc<WindowFactory>,
) -> anyhow::Result<()> {
  let window_configs = match cli.command() {
    CliCommand::Open(args) => {
      let window_config = config
        .window_config_by_path(&config.join_config_dir(&args.config_path))
        .await?
        .with_context(|| {
          format!(
            "Window config not found at {}.",
            args.config_path.display()
          )
        })?;

      vec![window_config]
    }
    _ => config.startup_window_configs().await?,
  };

  for window_config in window_configs {
    if let Err(err) = window_factory.open(window_config).await {
      error!("Failed to open window: {:?}", err);
    }
  }

  Ok(())
}
