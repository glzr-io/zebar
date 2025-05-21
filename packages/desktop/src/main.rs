// Prevent additional console window on Windows in release mode.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(iterator_try_collect)]

use std::{env, sync::Arc};

use clap::Parser;
use tauri::{
  async_runtime::block_on, AppHandle, Emitter, Manager, RunEvent,
};
use tokio::{sync::mpsc, task};
use tracing::{error, info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

#[cfg(target_os = "windows")]
use crate::common::windows::WindowExtWindows;
use crate::{
  app_settings::AppSettings,
  asset_server::setup_asset_server,
  cli::{Cli, CliCommand, MonitorType, QueryArgs},
  marketplace_installer::MarketplaceInstaller,
  monitor_state::MonitorState,
  providers::{ProviderEmission, ProviderManager},
  shell_state::ShellState,
  sys_tray::SysTray,
  widget_factory::{WidgetFactory, WidgetOpenOptions},
  widget_pack::{
    MonitorSelection, WidgetPack, WidgetPackManager, WidgetPlacement,
  },
};

mod app_settings;
mod asset_server;
mod cli;
mod commands;
mod common;
mod config_migration;
mod marketplace_installer;
mod monitor_state;
mod providers;
mod publish;
mod shell_state;
mod sys_tray;
mod widget_factory;
mod widget_pack;

#[macro_use]
extern crate rocket;

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

  tauri::async_runtime::set(tokio::runtime::Handle::current());

  let app = tauri::Builder::default()
    .setup(|app| {
      task::block_in_place(|| {
        block_on(async move {
          let cli = Cli::parse();

          match cli.command() {
            CliCommand::Query(args) => output_query(app, args),
            CliCommand::Publish(args) => {
              let result = publish::publish_widget_pack(&args).await;
              cli::print_and_exit(result);
              Ok(())
            }
            _ => {
              let start_res = start_app(app, cli).await;

              // If unable to start Zebar, the error is fatal and a message
              // dialog is shown.
              if let Err(err) = &start_res {
                // TODO: Show error dialog.
                error!("{:?}", err);
              };

              start_res
            }
          }?;

          Ok(())
        })
      })
    })
    .invoke_handler(tauri::generate_handler![
      commands::widget_packs,
      commands::widget_states,
      commands::start_widget,
      commands::start_widget_preset,
      commands::stop_widget_preset,
      commands::update_widget_config,
      commands::create_widget_pack,
      commands::update_widget_pack,
      commands::delete_widget_pack,
      commands::create_widget_config,
      commands::delete_widget_config,
      commands::listen_provider,
      commands::unlisten_provider,
      commands::call_provider_function,
      commands::install_widget_pack,
      commands::start_preview_widget,
      commands::stop_all_preview_widgets,
      commands::set_always_on_top,
      commands::set_skip_taskbar,
      commands::shell_exec,
      commands::shell_spawn,
      commands::shell_write,
      commands::shell_kill,
    ])
    .build(tauri::generate_context!())?;

  app.run(|app, event| {
    if let RunEvent::ExitRequested { code, api, .. } = &event {
      if code.is_none() {
        // Keep the message loop running even if all windows are closed.
        api.prevent_exit();
      } else {
        // Deallocate any appbars on Windows.
        #[cfg(target_os = "windows")]
        {
          for (_, window) in app.webview_windows() {
            let _ = window.as_ref().window().deallocate_app_bar();
          }
        }
      }
    }
  });

  Ok(())
}

/// Query state and print to the console.
fn output_query(app: &tauri::App, args: QueryArgs) -> anyhow::Result<()> {
  match args {
    QueryArgs::Monitors => {
      let monitors = MonitorState::new(&app.handle());
      cli::print_and_exit(monitors.output_str());
      Ok(())
    }
  }
}

/// Starts Zebar - either with a specific widget or all widgets.
async fn start_app(app: &mut tauri::App, cli: Cli) -> anyhow::Result<()> {
  tracing_subscriber::fmt()
    .with_env_filter(
      EnvFilter::from_env("LOG_LEVEL")
        .add_directive(LevelFilter::INFO.into()),
    )
    .init();

  let config_dir_override = match cli.command() {
    CliCommand::Startup(args) => args.config_dir,
    _ => None,
  };

  // Initialize `AppSettings` in Tauri state.
  let app_settings =
    Arc::new(AppSettings::new(app.handle(), config_dir_override)?);
  app.manage(app_settings.clone());

  // Initialize `MarketplaceInstaller` in Tauri state.
  let (marketplace_installer, install_rx) =
    MarketplaceInstaller::new(app.handle(), app_settings.clone())?;
  app.manage(marketplace_installer.clone());

  // Initialize `WidgetPackManager` in Tauri state.
  let widget_pack_manager = Arc::new(WidgetPackManager::new(
    app_settings.clone(),
    marketplace_installer.clone(),
  )?);
  app.manage(widget_pack_manager.clone());

  // Initialize `MonitorState` in Tauri state.
  let monitor_state = Arc::new(MonitorState::new(app.handle()));
  app.manage(monitor_state.clone());

  // Initialize `WidgetFactory` in Tauri state.
  let widget_factory = Arc::new(WidgetFactory::new(
    app.handle(),
    app_settings.clone(),
    widget_pack_manager.clone(),
    monitor_state.clone(),
  ));
  app.manage(widget_factory.clone());

  // If this is not the first instance of the app, this will emit within
  // the original instance and exit immediately. The CLI command is
  // guaranteed to be one of the open commands here.
  setup_single_instance(app, widget_factory.clone())?;

  setup_asset_server();

  // Prevent windows from showing up in the dock on MacOS.
  #[cfg(target_os = "macos")]
  app.set_activation_policy(tauri::ActivationPolicy::Accessory);

  // Allow assets to be resolved from the config directory and the
  // marketplace download directory.
  for dir in [
    &app_settings.config_dir,
    &app_settings.marketplace_download_dir,
  ] {
    app.asset_protocol_scope().allow_directory(dir, true)?;
  }

  app.manage(ShellState::new(app.handle(), widget_factory.clone()));
  app.handle().plugin(tauri_plugin_dialog::init())?;
  app.handle().plugin(tauri_plugin_shell::init())?;

  // Initialize `ProviderManager` in Tauri state.
  let (manager, emit_rx) = ProviderManager::new(app.handle());
  app.manage(manager.clone());

  // Open widgets based on CLI command.
  open_widgets_by_cli_command(cli, widget_factory.clone()).await?;

  // Add application icon to system tray.
  let tray = SysTray::new(
    app.handle(),
    app_settings.clone(),
    widget_pack_manager.clone(),
    widget_factory.clone(),
  )
  .await?;

  listen_events(
    app.handle(),
    app_settings,
    widget_pack_manager,
    monitor_state,
    widget_factory,
    tray,
    manager,
    emit_rx,
    install_rx,
  );

  Ok(())
}

/// Listens for events and updates state accordingly.
#[allow(clippy::too_many_arguments)]
fn listen_events(
  app_handle: &AppHandle,
  app_settings: Arc<AppSettings>,
  widget_pack_manager: Arc<WidgetPackManager>,
  monitor_state: Arc<MonitorState>,
  widget_factory: Arc<WidgetFactory>,
  tray: SysTray,
  manager: Arc<ProviderManager>,
  mut emit_rx: mpsc::UnboundedReceiver<ProviderEmission>,
  mut install_rx: mpsc::Receiver<WidgetPack>,
) {
  let app_handle = app_handle.clone();
  let mut widget_open_rx = widget_factory.open_tx.subscribe();
  let mut widget_close_rx = widget_factory.close_tx.subscribe();
  let mut settings_change_rx = app_settings.settings_change_tx.subscribe();
  let mut monitors_change_rx = monitor_state.change_tx.subscribe();
  let mut widget_configs_change_rx =
    widget_pack_manager.widget_configs_change_tx.subscribe();
  let mut widget_packs_change_rx =
    widget_pack_manager.widget_packs_change_tx.subscribe();

  task::spawn(async move {
    loop {
      let res = tokio::select! {
        Ok(widget_state) = widget_open_rx.recv() => {
          info!("Widget opened.");
          let _ = tray.refresh().await;
          let _ = app_handle.emit("widget-opened", widget_state);
          Ok(())
        },
        Ok(widget_id) = widget_close_rx.recv() => {
          info!("Widget closed.");
          let _ = tray.refresh().await;
          let _ = app_handle.emit("widget-closed", widget_id);
          Ok(())
        },
        Ok(_) = settings_change_rx.recv() => {
          info!("Settings changed.");
          tray.refresh().await
        },
        Ok(_) = widget_packs_change_rx.recv() => {
          info!("Widget packs changed.");
          tray.refresh().await
        },
        Ok(_) = monitors_change_rx.recv() => {
          info!("Monitors changed.");
          widget_factory.relaunch_all().await
        },
        Ok((pack_id, changed_config)) = widget_configs_change_rx.recv() => {
          info!("Widget config changed.");
          widget_factory
            .relaunch_by_name(&pack_id, &changed_config.name)
            .await
        },
        Some(provider_emission) = emit_rx.recv() => {
          info!("Provider emission: {:?}", provider_emission);
          let _ = app_handle.emit("provider-emit", provider_emission.clone());
          manager.update_cache(provider_emission).await;
          Ok(())
        },
        Some(pack) = install_rx.recv() => {
          info!("Widget pack installed: {:?}", pack);
          widget_pack_manager.register_widget_pack(pack).await;
          Ok(())
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
  widget_factory: Arc<WidgetFactory>,
) -> anyhow::Result<()> {
  app.handle().plugin(tauri_plugin_single_instance::init(
    move |_, args, _| {
      let widget_factory = widget_factory.clone();

      task::spawn(async move {
        let res = match Cli::try_parse_from(args) {
          Ok(cli) => {
            // No-op if no subcommand is provided.
            if cli.command() != CliCommand::Empty {
              open_widgets_by_cli_command(cli, widget_factory).await
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

/// Opens widgets based on CLI command.
async fn open_widgets_by_cli_command(
  cli: Cli,
  widget_factory: Arc<WidgetFactory>,
) -> anyhow::Result<()> {
  let res = match cli.command() {
    CliCommand::StartWidget(args) => {
      widget_factory
        .start_widget_by_id(
          &args.pack_id,
          &args.widget_name,
          &WidgetOpenOptions::Standalone(WidgetPlacement {
            anchor: args.anchor,
            offset_x: args.offset_x,
            offset_y: args.offset_y,
            width: args.width,
            height: args.height,
            monitor_selection: match args.monitor_type {
              MonitorType::All => MonitorSelection::All,
              MonitorType::Primary => MonitorSelection::Primary,
              MonitorType::Secondary => MonitorSelection::Secondary,
            },
            dock_to_edge: Default::default(),
          }),
          false,
        )
        .await
    }
    CliCommand::StartWidgetPreset(args) => {
      widget_factory
        .start_widget_by_id(
          &args.pack_id,
          &args.widget_name,
          &WidgetOpenOptions::Preset(args.preset_name),
          false,
        )
        .await
    }
    CliCommand::Startup(_) | CliCommand::Empty => {
      widget_factory.startup().await
    }
    _ => unreachable!(),
  };

  if let Err(err) = res {
    error!("Failed to open widgets: {:?}", err);
  }

  Ok(())
}
