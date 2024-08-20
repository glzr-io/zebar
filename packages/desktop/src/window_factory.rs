use std::{
  collections::HashMap,
  sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
  },
};

use serde::Serialize;
use tauri::{AppHandle, Runtime, WebviewUrl, WebviewWindowBuilder};
use tokio::{sync::Mutex, task};
use tracing::{error, info};

use crate::{cli::OpenWindowArgs, util::window_ext::WindowExt};

/// Manages the creation of Zebar windows.
pub struct WindowFactory {
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
  pub fn new() -> Self {
    Self {
      window_count: Arc::new(AtomicU32::new(0)),
      window_states: Arc::new(Mutex::new(HashMap::new())),
    }
  }

  pub fn open<R: Runtime>(
    &self,
    app_handle: &AppHandle<R>,
    open_args: OpenWindowArgs,
    // window_count: u32,
  ) -> anyhow::Result<()> {
    let app_handle = app_handle.clone();
    let new_count = self.window_count.fetch_add(1, Ordering::Relaxed) + 1;
    let window_states = self.window_states.clone();
    let args = open_args.args.unwrap_or(vec![]).into_iter().collect();

    info!(
      "Creating window #{} '{}' with args: {:#?}",
      new_count, open_args.window_id, args
    );

    task::spawn(async move {
      // Window label needs to be globally unique. Hence add a prefix with
      // the window count to handle cases where multiple of the same window
      // are opened.
      let window_label = format!("{}-{}", new_count, &open_args.window_id);

      let window = WebviewWindowBuilder::new(
        &app_handle,
        &window_label,
        WebviewUrl::default(),
      )
      .title(format!("Zebar - {}", open_args.window_id))
      .inner_size(500., 500.)
      .focused(false)
      .skip_taskbar(true)
      .visible_on_all_workspaces(true)
      .transparent(true)
      .shadow(false)
      .decorations(false)
      .resizable(false)
      .build()
      .unwrap();

      let state = WindowState {
        window_id: open_args.window_id.clone(),
        window_label: window_label.clone(),
        args,
        env: std::env::vars().collect(),
      };

      _ = window.eval(&format!(
        "window.__ZEBAR_OPEN_ARGS={}",
        serde_json::to_string(&state).unwrap()
      ));

      // Tauri's `skip_taskbar` option isn't 100% reliable, so we
      // also set the window as a tool window.
      #[cfg(target_os = "windows")]
      let _ = window.as_ref().window().set_tool_window(true);

      let mut window_states = window_states.lock().await;
      window_states.insert(state.window_id.clone(), state);
    });

    Ok(())
  }

  pub async fn open2(
    &self,
    open_args: OpenWindowArgs,
  ) -> Option<WindowState> {
    // TODO
    None
  }

  /// Gets an open window's state by a given window label.
  pub async fn state_by_window_label(
    &self,
    window_label: String,
  ) -> Option<WindowState> {
    let windows = self.window_states.lock().await;
    windows.get(&window_label).cloned()
  }
}
