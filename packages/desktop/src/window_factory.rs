use std::{
  collections::HashMap,
  sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
  },
};

use serde::Serialize;
use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};
use tokio::sync::Mutex;
use tracing::info;

use crate::{
  common::WindowExt,
  config::{WindowAnchor, WindowConfig, WindowConfigEntry},
  monitor_state::MonitorState,
};

/// Manages the creation of Zebar windows.
pub struct WindowFactory {
  /// Handle to the Tauri application.
  app_handle: AppHandle,

  /// Reference to `MonitorState` for window positioning.
  monitor_state: Arc<MonitorState>,

  /// Running total of windows created.
  ///
  /// Used to generate unique window labels.
  window_count: Arc<AtomicU32>,

  /// Map of window labels to window states.
  window_states: Arc<Mutex<HashMap<String, WindowState>>>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WindowState {
  /// Unique identifier for the window.
  ///
  /// Used as the window label.
  pub window_id: String,

  /// User-defined config for the window.
  pub config: WindowConfig,

  /// Absolute path to the window's config file.
  pub config_path: String,

  /// Absolute path to the window's HTML file.
  pub html_path: String,
}

pub struct WindowPlacement {
  width: f64,
  height: f64,
  x: f64,
  y: f64,
}

impl WindowFactory {
  /// Creates a new `WindowFactory` instance.
  pub fn new(
    app_handle: &AppHandle,
    monitor_state: Arc<MonitorState>,
  ) -> Self {
    Self {
      app_handle: app_handle.clone(),
      monitor_state,
      window_count: Arc::new(AtomicU32::new(0)),
      window_states: Arc::new(Mutex::new(HashMap::new())),
    }
  }

  /// Opens windows from a given config entry.
  pub async fn open(
    &self,
    config_entry: WindowConfigEntry,
  ) -> anyhow::Result<()> {
    let WindowConfigEntry {
      config,
      config_path,
      html_path,
    } = &config_entry;

    for placement in self.window_placements(config) {
      let new_count =
        self.window_count.fetch_add(1, Ordering::Relaxed) + 1;

      info!("Creating window #{} from {}", new_count, config_path);

      // Note that window label needs to be globally unique.
      let window = WebviewWindowBuilder::new(
        &self.app_handle,
        new_count.to_string(),
        WebviewUrl::App(
          format!("http://asset.localhost/{}", html_path).into(),
        ),
      )
      .title("Zebar")
      .inner_size(placement.width, placement.height)
      .position(placement.x, placement.y)
      .focused(config.launch_options.focused)
      .skip_taskbar(!config.launch_options.shown_in_taskbar)
      .visible_on_all_workspaces(true)
      .transparent(config.launch_options.transparent)
      .shadow(false)
      .decorations(false)
      .resizable(config.launch_options.resizable)
      .build()?;

      let state = WindowState {
        window_id: new_count.to_string(),
        config: config.clone(),
        config_path: config_path.clone(),
        html_path: html_path.clone(),
      };

      _ = window.eval(&format!(
        "window.__ZEBAR_INITIAL_STATE={}",
        serde_json::to_string(&state)?
      ));

      // Tauri's `skip_taskbar` option isn't 100% reliable, so we
      // also set the window as a tool window.
      #[cfg(target_os = "windows")]
      let _ = window
        .as_ref()
        .window()
        .set_tool_window(!config.launch_options.shown_in_taskbar);

      let mut window_states = self.window_states.lock().await;
      window_states.insert(state.window_id.clone(), state);
    }

    Ok(())
  }

  /// Returns coordinates for window placement based on the given config.
  fn window_placements(
    &self,
    config: &WindowConfig,
  ) -> Vec<WindowPlacement> {
    let mut placements = vec![];

    for placement in config.launch_options.placements.iter() {
      let monitors = self
        .monitor_state
        .monitors_by_selection(&placement.monitor_selection);

      for monitor in monitors {
        let (anchor_x, anchor_y) = match placement.anchor {
          WindowAnchor::TopLeft => (monitor.x, monitor.y),
          WindowAnchor::TopCenter => {
            (monitor.x + (monitor.width as i32 / 2), monitor.y)
          }
          WindowAnchor::TopRight => {
            (monitor.x + monitor.width as i32, monitor.y)
          }
          WindowAnchor::CenterLeft => {
            (monitor.x, monitor.y + (monitor.height as i32 / 2))
          }
          WindowAnchor::Center => (
            monitor.x + (monitor.width as i32 / 2),
            monitor.y + (monitor.height as i32 / 2),
          ),
          WindowAnchor::CenterRight => (
            monitor.x + monitor.width as i32,
            monitor.y + (monitor.height as i32 / 2),
          ),
          WindowAnchor::BottomLeft => {
            (monitor.x, monitor.y + monitor.height as i32)
          }
          WindowAnchor::BottomCenter => (
            monitor.x + (monitor.width as i32 / 2),
            monitor.y + monitor.height as i32,
          ),
          WindowAnchor::BottomRight => (
            monitor.x + monitor.width as i32,
            monitor.y + monitor.height as i32,
          ),
        };

        let placement = WindowPlacement {
          width: placement.width.to_px(monitor.width as i32) as f64,
          height: placement.height.to_px(monitor.height as i32) as f64,
          x: (anchor_x + placement.offset_x.to_px(monitor.width as i32))
            as f64,
          y: (anchor_y + placement.offset_y.to_px(monitor.height as i32))
            as f64,
        };

        placements.push(placement);
      }
    }

    placements
  }

  /// Gets an open window's state by a given window ID.
  pub async fn state_by_id(&self, window_id: &str) -> Option<WindowState> {
    self.window_states.lock().await.get(window_id).cloned()
  }
}
