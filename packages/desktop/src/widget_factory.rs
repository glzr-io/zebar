use std::{
  collections::HashMap,
  path::PathBuf,
  sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
  },
};

use anyhow::{bail, Context};
use serde::Serialize;
use tauri::{
  AppHandle, LogicalPosition, LogicalSize, Manager, PhysicalPosition,
  PhysicalSize, WebviewUrl, WebviewWindowBuilder, WindowEvent,
};
use tokio::{
  sync::{broadcast, Mutex},
  task,
};
use tracing::{error, info};

use crate::{
  common::{PathExt, WindowExt},
  config::{AnchorPoint, Config, WidgetConfig, WidgetConfigEntry},
  monitor_state::MonitorState,
};

/// Manages the creation of Zebar widgets.
pub struct WidgetFactory {
  /// Handle to the Tauri application.
  app_handle: AppHandle,

  _close_rx: broadcast::Receiver<WidgetState>,

  pub close_tx: broadcast::Sender<WidgetState>,

  /// Reference to `Config`.
  config: Arc<Config>,

  _open_rx: broadcast::Receiver<WidgetState>,

  pub open_tx: broadcast::Sender<WidgetState>,

  /// Reference to `MonitorState`.
  ///
  /// Used for widget positioning.
  monitor_state: Arc<MonitorState>,

  /// Running total of widgets created.
  ///
  /// Used to generate unique widget ID's which are used as Tauri window
  /// labels.
  widget_count: Arc<AtomicU32>,

  /// Map of widget ID's to their states.
  widget_states: Arc<Mutex<HashMap<String, WidgetState>>>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WidgetState {
  /// Unique identifier for the widget.
  ///
  /// Used as the Tauri window label.
  pub id: String,

  /// User-defined config for the widget.
  pub config: WidgetConfig,

  /// Absolute path to the widget's config file.
  pub config_path: PathBuf,

  /// Absolute path to the widget's HTML file.
  pub html_path: PathBuf,
}

impl WidgetFactory {
  /// Creates a new `WidgetFactory` instance.
  pub fn new(
    app_handle: &AppHandle,
    config: Arc<Config>,
    monitor_state: Arc<MonitorState>,
  ) -> Self {
    let (open_tx, _open_rx) = broadcast::channel(16);
    let (close_tx, _close_rx) = broadcast::channel(16);

    Self {
      app_handle: app_handle.clone(),
      _close_rx,
      close_tx,
      config,
      _open_rx,
      open_tx,
      monitor_state,
      widget_count: Arc::new(AtomicU32::new(0)),
      widget_states: Arc::new(Mutex::new(HashMap::new())),
    }
  }

  /// Opens widget from a given config entry.
  pub async fn open(
    &self,
    config_entry: WidgetConfigEntry,
  ) -> anyhow::Result<()> {
    let WidgetConfigEntry {
      config,
      config_path,
      html_path,
    } = &config_entry;

    for (size, position) in self.widget_placements(config).await {
      // Use running widget count as a unique label for the Tauri window.
      let new_count =
        self.widget_count.fetch_add(1, Ordering::Relaxed) + 1;
      let widget_id = new_count.to_string();

      if !html_path.exists() {
        bail!(
          "HTML file not found at {} for config {}.",
          html_path.display(),
          config_path.display()
        );
      }

      info!(
        "Creating window for widget #{} from {}",
        new_count,
        config_path.display()
      );

      let webview_url = WebviewUrl::App(
        Self::to_asset_url(&html_path.to_unicode_string()).into(),
      );

      // Note that window label needs to be globally unique.
      let window = WebviewWindowBuilder::new(
        &self.app_handle,
        widget_id.clone(),
        webview_url,
      )
      .title("Zebar")
      .inner_size(size.width, size.height)
      .position(position.x, position.y)
      .focused(config.focused)
      .skip_taskbar(!config.shown_in_taskbar)
      .visible_on_all_workspaces(true)
      .transparent(config.transparent)
      .shadow(false)
      .decorations(false)
      .resizable(config.resizable)
      .build()?;

      let state = WidgetState {
        id: widget_id.clone(),
        config: config.clone(),
        config_path: config_path.clone(),
        html_path: html_path.clone(),
      };

      _ = window.eval(&format!(
        "window.__ZEBAR_STATE={0}; sessionStorage.setItem('ZEBAR_STATE', JSON.stringify({0}))",
        serde_json::to_string(&state)?
      ));

      // On Windows, Tauri's `skip_taskbar` option isn't 100% reliable, so
      // we also set the window as a tool window.
      #[cfg(target_os = "windows")]
      let _ = window
        .as_ref()
        .window()
        .set_tool_window(!config.shown_in_taskbar);

      // On Windows, there's an issue where the window size is constrained
      // when initially created. To work around this, apply the size and
      // position settings again after launch.
      #[cfg(target_os = "windows")]
      {
        let _ = window.set_size(size);
        let _ = window.set_position(position);
        let _ = window.set_size(size);
        // Is it also required to set the position twice or only the size?
        let _ = window.set_position(position);
      }

      let mut widget_states = self.widget_states.lock().await;
      widget_states.insert(state.id.clone(), state.clone());

      self.register_window_events(&window, widget_id);
      self.open_tx.send(state)?;
    }

    Ok(())
  }

  /// Converts a file path to a Tauri asset URL.
  ///
  /// Returns a string that can be used as a webview URL.
  pub fn to_asset_url(file_path: &str) -> String {
    if cfg!(target_os = "windows") {
      format!("http://asset.localhost/{}", file_path)
    } else {
      format!("asset://localhost/{}", file_path)
    }
  }

  /// Registers window events for a given widget.
  fn register_window_events(
    &self,
    window: &tauri::WebviewWindow,
    widget_id: String,
  ) {
    let widget_states = self.widget_states.clone();
    let close_tx = self.close_tx.clone();

    window.on_window_event(move |event| {
      if let WindowEvent::Destroyed = event {
        let widget_states = widget_states.clone();
        let close_tx = close_tx.clone();
        let widget_id = widget_id.clone();

        task::spawn(async move {
          let mut widget_states = widget_states.lock().await;

          // Remove the widget state and broadcast the close event.
          if let Some(state) = widget_states.remove(&widget_id) {
            if let Err(err) = close_tx.send(state) {
              error!("Failed to send window close event: {:?}", err);
            }
          }
        });
      }
    });
  }

  /// Returns coordinates for window placement based on the given config.
  async fn widget_placements(
    &self,
    config: &WidgetConfig,
  ) -> Vec<(PhysicalSize<f64>, PhysicalPosition<f64>)> {
    let mut placements = vec![];

    for placement in config.default_placements.iter() {
      let monitors = self
        .monitor_state
        .monitors_by_selection(&placement.monitor_selection)
        .await;

      for monitor in monitors {
        let (anchor_x, anchor_y) = match placement.anchor {
          AnchorPoint::TopLeft => (monitor.x, monitor.y),
          AnchorPoint::TopCenter => {
            (monitor.x + (monitor.width as i32 / 2), monitor.y)
          }
          AnchorPoint::TopRight => {
            (monitor.x + monitor.width as i32, monitor.y)
          }
          AnchorPoint::CenterLeft => {
            (monitor.x, monitor.y + (monitor.height as i32 / 2))
          }
          AnchorPoint::Center => (
            monitor.x + (monitor.width as i32 / 2),
            monitor.y + (monitor.height as i32 / 2),
          ),
          AnchorPoint::CenterRight => (
            monitor.x + monitor.width as i32,
            monitor.y + (monitor.height as i32 / 2),
          ),
          AnchorPoint::BottomLeft => {
            (monitor.x, monitor.y + monitor.height as i32)
          }
          AnchorPoint::BottomCenter => (
            monitor.x + (monitor.width as i32 / 2),
            monitor.y + monitor.height as i32,
          ),
          AnchorPoint::BottomRight => (
            monitor.x + monitor.width as i32,
            monitor.y + monitor.height as i32,
          ),
        };

        let size = {
          let size = PhysicalSize::<f64>::from_logical(
            LogicalSize::new(
              placement.width.to_px(monitor.width as i32),
              placement.height.to_px(monitor.height as i32),
            ),
            monitor.scale_factor,
          );
          PhysicalSize::new(
            f64::min(size.width, monitor.width as f64),
            f64::min(size.height, monitor.height as f64),
          )
        };

        let offset = PhysicalPosition::<f64>::from_logical(
          LogicalPosition::new(
            placement.offset_x.to_px(monitor.width as i32),
            placement.offset_y.to_px(monitor.height as i32),
          ),
          monitor.scale_factor,
        );
        let position = PhysicalPosition::new(
          anchor_x as f64 + offset.x,
          anchor_y as f64 + offset.y,
        );

        placements.push((size, position));
      }
    }

    placements
  }

  /// Closes a single widget by a given widget ID.
  pub fn stop_by_id(&self, widget_id: &str) -> anyhow::Result<()> {
    let window = self
      .app_handle
      .get_webview_window(widget_id)
      .context("No Tauri window found for the given widget ID.")?;

    window.close()?;

    Ok(())
  }

  /// Closes all widgets with the given config path.
  pub async fn stop_by_path(
    &self,
    config_path: &PathBuf,
  ) -> anyhow::Result<()> {
    let widget_states = self.states_by_config_path().await;

    let found_widget_states = widget_states
      .get(config_path)
      .context("No widgets found with the given config path.")?;

    for widget_state in found_widget_states {
      self.stop_by_id(&widget_state.id)?;
    }

    Ok(())
  }

  /// Relaunches all currently open widgets.
  pub async fn relaunch_all(&self) -> anyhow::Result<()> {
    let widget_states = self.states_by_config_path().await;

    for (config_path, _) in widget_states {
      let _ = self.stop_by_path(&config_path).await;

      let widget_config = self
        .config
        .widget_config_by_path(&config_path)
        .await?
        .context("Widget config not found.")?;

      self.open(widget_config).await?;
    }

    Ok(())
  }

  /// Returns widget state by a given widget ID.
  pub async fn state_by_id(&self, widget_id: &str) -> Option<WidgetState> {
    self.widget_states.lock().await.get(widget_id).cloned()
  }

  /// Returns widget states grouped by their config paths.
  pub async fn states_by_config_path(
    &self,
  ) -> HashMap<PathBuf, Vec<WidgetState>> {
    self.widget_states.lock().await.values().fold(
      HashMap::new(),
      |mut acc, state| {
        acc
          .entry(state.config_path.clone())
          .or_insert_with(Vec::new)
          .push(state.clone());

        acc
      },
    )
  }
}
