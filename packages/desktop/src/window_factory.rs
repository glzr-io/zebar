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

use crate::util::WindowExt;

/// Manages the creation of Zebar windows.
pub struct WindowFactory {
  app_handle: AppHandle,
  window_count: Arc<AtomicU32>,
  window_states: Arc<Mutex<HashMap<String, WindowState>>>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WindowState {
  pub window_id: String,
  pub window_label: String,
  pub args: HashMap<String, String>,
  pub env: HashMap<String, String>,
}

impl WindowFactory {
  pub fn new(app_handle: &AppHandle) -> Self {
    Self {
      app_handle: app_handle.clone(),
      window_count: Arc::new(AtomicU32::new(0)),
      window_states: Arc::new(Mutex::new(HashMap::new())),
    }
  }

  /// TODO: Pass in `WindowConfig`.
  pub fn try_open(&self) {
    let app_handle = self.app_handle.clone();
    let window_states = self.window_states.clone();
    let app_handle = app_handle.clone();
    let window_count = self.window_count.clone();

    task::spawn(async move {
      // Increment number of windows.
      let new_count = window_count.fetch_add(1, Ordering::Relaxed) + 1;

      let open_res = Self::open(&app_handle, new_count);

      match open_res {
        Ok(state) => {
          let mut window_states = window_states.lock().await;
          window_states.insert(state.window_label.clone(), state);
        }
        Err(err) => {
          error!("Failed to open window: {:?}", err);
        }
      }
    });
  }

  fn open(
    app_handle: &AppHandle,
    window_count: u32,
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
      window_label: window_count.to_string(),
      args: HashMap::new(),
      env: std::env::vars().collect(),
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

  /// Gets an open window's state by a given window label.
  pub async fn state_by_window_label(
    &self,
    window_label: String,
  ) -> Option<WindowState> {
    self.window_states.lock().await.get(&window_label).cloned()
  }
}
