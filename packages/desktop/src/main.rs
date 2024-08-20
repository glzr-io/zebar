use std::env;

use clap::Parser;
use cli::OpenWindowArgs;
use providers::config::ProviderConfig;
use tauri::{AppHandle, Manager, State, Window};
use tokio::sync::mpsc::{self};
use tracing::{level_filters::LevelFilter, warn};
use tracing_subscriber::EnvFilter;
use window_factory::{WindowFactory, WindowState};

use crate::{
  cli::{Cli, CliCommand},
  monitors::get_monitors_str,
  providers::provider_manager::ProviderManager,
  sys_tray::setup_sys_tray,
  util::window_ext::WindowExt,
};

mod cli;
mod monitors;
mod providers;
mod sys_tray;
mod user_config;
mod util;
mod window_factory;

#[tauri::command]
fn read_config_file(
  config_path_override: Option<&str>,
  app_handle: AppHandle,
) -> anyhow::Result<String, String> {
  user_config::read_file(config_path_override, app_handle)
    .map_err(|err| err.to_string())
}

#[tauri::command]
async fn get_open_window_args(
  window_label: String,
  window_factory: State<'_, WindowFactory>,
) -> anyhow::Result<Option<WindowState>, String> {
  Ok(window_factory.state_by_window_label(window_label).await)
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

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt()
    .with_env_filter(
      EnvFilter::from_env("LOG_LEVEL")
        .add_directive(LevelFilter::INFO.into()),
    )
    .init();

  tauri::async_runtime::set(tokio::runtime::Handle::current());

  tauri::Builder::default()
    .setup(|app| {
      let cli = Cli::parse();

      // Since most Tauri plugins and setup is not needed for the
      // `monitors` CLI command, the setup is conditional based on
      // the CLI command.
      match cli.command {
        CliCommand::Monitors(args) => {
          let monitors_str = get_monitors_str(app, args);
          cli::print_and_exit(monitors_str);
          Ok(())
        }
        CliCommand::Open(args) => {
          let (open_tx, open_rx) =
            mpsc::unbounded_channel::<OpenWindowArgs>();

          // If this is not the first instance of the app, this will emit
          // within the original instance and exit immediately.
          let open_tx_clone = open_tx.clone();
          app.handle().plugin(tauri_plugin_single_instance::init(
            move |_, args, _| {
              let cli = Cli::parse_from(args);

              // CLI command is guaranteed to be an open command here.
              if let CliCommand::Open(args) = cli.command {
                if let Err(err) = open_tx_clone.send(args) {
                  warn!("Failed to emit window's open args: {}", err);
                };
              }
            },
          ))?;

          app.handle().plugin(tauri_plugin_shell::init())?;
          app.handle().plugin(tauri_plugin_http::init())?;
          app.handle().plugin(tauri_plugin_dialog::init())?;

          // Initializes `ProviderManager` in Tauri state.
          let mut manager = ProviderManager::new();
          manager.init(app.handle());
          app.manage(manager);

          // Initializes `WindowFactory` in Tauri state.
          let mut window_factory = WindowFactory::new(open_tx, open_rx);
          window_factory.init(app.handle(), args);
          app.manage(window_factory);

          // Add application icon to system tray.
          setup_sys_tray(app)?;

          Ok(())
        }
      }
    })
    .invoke_handler(tauri::generate_handler![
      read_config_file,
      get_open_window_args,
      listen_provider,
      unlisten_provider,
      set_always_on_top,
      set_skip_taskbar
    ])
    .run(tauri::generate_context!())
    .expect("Failed to build Tauri application.");
}
