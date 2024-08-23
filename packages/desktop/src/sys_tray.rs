use anyhow::Context;
use tauri::{
  menu::MenuBuilder,
  tray::{TrayIcon, TrayIconBuilder},
};
use tracing::{error, info};

pub fn setup_sys_tray(app: &mut tauri::App) -> anyhow::Result<TrayIcon> {
  let icon_image = app
    .default_window_icon()
    .context("No icon defined in Tauri config.")?;

  let tray_menu = MenuBuilder::new(app)
    .text("show_config_folder", "Show config folder")
    .separator()
    .text("exit", "Exit")
    .build()?;

  let tray_icon = TrayIconBuilder::with_id("tray")
    .icon(icon_image.clone())
    .menu(&tray_menu)
    .tooltip(format!("Zebar v{}", env!("VERSION_NUMBER")))
    .on_menu_event(move |app, event| match event.id().as_ref() {
      "show_config_folder" => {
        info!("Opening config folder from system tray.");
        // if let Err(err) = open_config_dir(app) {
        //   error!("Failed to open config folder: {}", err);
        // }
      }
      "exit" => {
        info!("Exiting through system tray.");
        app.exit(0)
      }
      other => {
        error!("Unknown menu event: {}", other);
      }
    })
    .build(app)?;

  Ok(tray_icon)
}
