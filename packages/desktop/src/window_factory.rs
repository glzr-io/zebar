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

use crate::{common::WindowExt, config::WindowConfig};

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

  pub fn open_all(&self, configs: Vec<WindowConfig>) {
    for config in configs {
      self.open_one(config);
    }
  }

  pub fn open_one(&self, config: WindowConfig) {
    let app_handle = self.app_handle.clone();
    let window_states = self.window_states.clone();
    let window_count = self.window_count.clone();

    task::spawn(async move {
      // Increment number of windows.
      let new_count = window_count.fetch_add(1, Ordering::Relaxed) + 1;

      let open_res = Self::create_window(&app_handle, new_count, config);

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

  fn create_window(
    app_handle: &AppHandle,
    window_count: u32,
    config: WindowConfig,
  ) -> anyhow::Result<WindowState> {
    info!("Creating window #{}", window_count);

    let window = WebviewWindowBuilder::new(
      app_handle,
      // Window label needs to be globally unique.
      window_count.to_string(),
      WebviewUrl::default(),
    )
    .title("Zebar")
    .inner_size(500., 500.)
    .focused(false)
    .skip_taskbar(true)
    .visible_on_all_workspaces(true)
    .transparent(false)
    .shadow(false)
    .decorations(false)
    .resizable(false)
    .build()?;

    let state = WindowState {
      window_id: window_count.to_string(),
      config,
    };

    _ = window.eval(&format!(
      "window.__ZEBAR_INITIAL_STATE={}",
      serde_json::to_string(&state)?
    ));

    // Tauri's `skip_taskbar` option isn't 100% reliable, so we
    // also set the window as a tool window.
    #[cfg(target_os = "windows")]
    let _ = window.as_ref().window().set_tool_window(true);

    Ok(state)
  }

  /// Gets an open window's state by a given window ID.
  pub async fn state_by_id(&self, window_id: &str) -> Option<WindowState> {
    self.window_states.lock().await.get(window_id).cloned()
  }
}
