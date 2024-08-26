use std::{
  collections::HashMap,
  sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
  },
};

use serde::Serialize;
use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};
use tokio::{sync::Mutex, task};
use tracing::{error, info};

use crate::{
  common::WindowExt,
  config::{WindowConfig, WindowConfigEntry},
};

/// Manages the creation of Zebar windows.
pub struct WindowFactory {
  /// Handle to the Tauri application.
  app_handle: AppHandle,

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
}

impl WindowFactory {
  pub fn new(app_handle: &AppHandle) -> Self {
    Self {
      app_handle: app_handle.clone(),
      window_count: Arc::new(AtomicU32::new(0)),
      window_states: Arc::new(Mutex::new(HashMap::new())),
    }
  }

  /// Opens all windows in given config entries.
  pub fn open_all(&self, config_entires: Vec<WindowConfigEntry>) {
    for config in config_entires {
      self.open_one(config);
    }
  }

  /// Opens a single window from a given config entry.
  pub fn open_one(&self, config_entry: WindowConfigEntry) {
    let app_handle = self.app_handle.clone();
    let window_states = self.window_states.clone();
    let window_count = self.window_count.clone();

    task::spawn(async move {
      // Increment number of windows.
      let new_count = window_count.fetch_add(1, Ordering::Relaxed) + 1;

      let open_res =
        Self::create_window(&app_handle, new_count, config_entry);

      match open_res {
        Ok(state) => {
          let mut window_states = window_states.lock().await;
          window_states.insert(state.window_id.clone(), state);
        }
        Err(err) => {
          error!("Failed to open window: {:?}", err);
        }
      }
    });
  }

  /// Spawns a new webview window with properties from the given config
  /// entry.
  fn create_window(
    app_handle: &AppHandle,
    window_count: u32,
    config_entry: WindowConfigEntry,
  ) -> anyhow::Result<WindowState> {
    let WindowConfigEntry {
      config,
      config_path,
      html_path,
    } = config_entry;

    info!("Creating window #{} from {}", window_count, config_path);

    // Note that window label needs to be globally unique.
    let window = WebviewWindowBuilder::new(
      app_handle,
      window_count.to_string(),
      WebviewUrl::App(
        format!("http://asset.localhost/{}", html_path).into(),
      ),
    )
    .title("Zebar")
    .inner_size(500., 500.)
    .focused(config.launch_options.focused)
    .skip_taskbar(!config.launch_options.shown_in_taskbar)
    .visible_on_all_workspaces(true)
    .transparent(config.launch_options.transparent)
    .shadow(false)
    .decorations(false)
    .resizable(config.launch_options.resizable)
    .build()?;

    let state = WindowState {
      window_id: window_count.to_string(),
      config: config.clone(),
      config_path: config_path.clone(),
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

    Ok(state)
  }

  /// Gets an open window's state by a given window ID.
  pub async fn state_by_id(&self, window_id: &str) -> Option<WindowState> {
    self.window_states.lock().await.get(window_id).cloned()
  }
}
