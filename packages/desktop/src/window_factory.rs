use std::{
  collections::HashMap,
  sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
  },
};

use serde::Serialize;
use tauri::{AppHandle, Runtime, WebviewUrl, WebviewWindowBuilder};
use tokio::{
  sync::{
    mpsc::{self},
    Mutex,
  },
  task,
};
use tracing::{error, info};

use crate::{cli::OpenWindowArgs, util::window_ext::WindowExt};

/// Manages the creation of Zebar windows.
pub struct WindowFactory {
  open_tx: mpsc::UnboundedSender<OpenWindowArgs>,
  open_rx: Option<mpsc::UnboundedReceiver<OpenWindowArgs>>,
  window_count: Arc<AtomicU32>,
  windows: Arc<Mutex<HashMap<String, WindowState>>>,
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
  pub fn new(
    open_tx: mpsc::UnboundedSender<OpenWindowArgs>,
    open_rx: mpsc::UnboundedReceiver<OpenWindowArgs>,
  ) -> Self {
    Self {
      open_tx,
      open_rx: Some(open_rx),
      window_count: Arc::new(AtomicU32::new(0)),
      windows: Arc::new(Mutex::new(HashMap::new())),
    }
  }

  /// Starts listening for provider outputs and emits them to frontend
  /// clients.
  pub fn init<R: Runtime>(
    &mut self,
    app_handle: &AppHandle<R>,
    args: OpenWindowArgs,
  ) {
    let mut open_rx = self.open_rx.take().unwrap();
    let windows = self.windows.clone();
    let app_handle = app_handle.clone();
    let window_count = self.window_count.clone();

    // Prevent windows from showing up in the dock on MacOS.
    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    self.open_tx.send(args).unwrap();

    task::spawn(async move {
      while let Some(open_args) = open_rx.recv().await {
        let new_count = window_count.fetch_add(1, Ordering::Relaxed) + 1;

        let open_res =
          Self::open(app_handle.clone(), open_args, new_count);

        match open_res {
          Ok(state) => {
            let mut windows = windows.lock().await;
            windows.insert(state.window_id.clone(), state);
          }
          Err(err) => {
            error!("Failed to open window: {:?}", err);
          }
        }
      }
    });
  }

  pub fn open<R: Runtime>(
    app_handle: AppHandle<R>,
    open_args: OpenWindowArgs,
    window_count: u32,
  ) -> anyhow::Result<WindowState> {
    let args = open_args.args.unwrap_or(vec![]).into_iter().collect();

    info!(
      "Creating window #{} '{}' with args: {:#?}",
      window_count, open_args.window_id, args
    );

    // Window label needs to be globally unique. Hence add a prefix with
    // the window count to handle cases where multiple of the same window
    // are opened.
    let window_label =
      format!("{}-{}", window_count, &open_args.window_id);

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
    .build()?;

    let state = WindowState {
      window_id: open_args.window_id.clone(),
      window_label: window_label.clone(),
      args,
      env: std::env::vars().collect(),
    };

    _ = window.eval(&format!(
      "window.__ZEBAR_OPEN_ARGS={}",
      serde_json::to_string(&state)?
    ));

    // Tauri's `skip_taskbar` option isn't 100% reliable, so we
    // also set the window as a tool window.
    #[cfg(target_os = "windows")]
    let _ = window.as_ref().window().set_tool_window(true);

    Ok(state)
  }

  /// Gets an open window's state by a given label.
  pub async fn state_by_window_label(
    &self,
    window_label: String,
  ) -> Option<WindowState> {
    let windows = self.windows.lock().await;
    windows.get(&window_label).cloned()
  }
}
