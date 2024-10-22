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
  AppHandle, Manager, PhysicalPosition, PhysicalSize, WebviewUrl,
  WebviewWindowBuilder, WindowEvent,
};
use tokio::{
  sync::{broadcast, Mutex},
  task,
};
use tracing::{error, info};

use crate::{
  common::{PathExt, WindowExt},
  config::{
    AnchorPoint, Config, WidgetConfig, WidgetConfigEntry, WidgetPlacement,
    ZOrder,
  },
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
  pub async fn start_widget(
    &self,
    config_path: &PathBuf,
    placement: &WidgetPlacement,
  ) -> anyhow::Result<()> {
    todo!()
  }

  /// Opens widget from a given config entry.
  pub async fn start_preset(
    &self,
    config_path: &PathBuf,
    preset_name: &str,
  ) -> anyhow::Result<()> {
    let WidgetConfigEntry {
      config,
      config_path,
      config_dir: html_path,
    } = &config_entry;

    for (size, position) in self.widget_coordinates(config).await {
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
      .focused(config.focused)
      .skip_taskbar(!config.shown_in_taskbar)
      .visible_on_all_workspaces(true)
      .transparent(config.transparent)
      .shadow(false)
      .decorations(false)
      .resizable(config.resizable)
      .build()?;

      info!("Positioning widget to {:?} {:?}", size, position);
      let _ = window.set_size(size);
      let _ = window.set_position(position);

      // On Windows, we need to set the position twice to account for
      // different monitor scale factors.
      #[cfg(target_os = "windows")]
      {
        let _ = window.set_size(size);
        let _ = window.set_position(position);
      }

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

      // On MacOS, we need to set the window as above the menu bar for it
      // to truly be always on top.
      #[cfg(target_os = "macos")]
      {
        if config.z_order == ZOrder::TopMost {
          let _ = window.as_ref().window().set_above_menu_bar();
        }
      }

      {
        let mut widget_states = self.widget_states.lock().await;
        widget_states.insert(state.id.clone(), state.clone());
      }

      self.register_window_events(&window, widget_id);
      self.open_tx.send(state)?;
    }

    Ok(())
  }

  /// Opens presets that are configured to be launched on startup.
  pub async fn startup(&self) -> anyhow::Result<()> {
    todo!()
  }

  /// Converts a file path to a Tauri asset URL.
  ///
  /// Returns a string that can be used as a webview URL.
  fn to_asset_url(file_path: &str) -> String {
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
  async fn widget_coordinates(
    &self,
    placement: &WidgetPlacement,
  ) -> Vec<(PhysicalSize<i32>, PhysicalPosition<i32>)> {
    let mut coordinates = vec![];

    let monitors = self
      .monitor_state
      .monitors_by_selection(&placement.monitor_selection)
      .await;

    for monitor in monitors {
      let monitor_width = monitor.width as i32;
      let monitor_height = monitor.height as i32;

      // Pixel values should be scaled by the monitor's scale factor,
      // whereas percentage values are left as-is. This is because the
      // percentage values are already relative to the monitor's size.
      let window_width = placement
        .width
        .to_px_scaled(monitor_width, monitor.scale_factor);

      let window_height = placement
        .height
        .to_px_scaled(monitor_height, monitor.scale_factor);

      let window_size = PhysicalSize::new(window_width, window_height);

      let (anchor_x, anchor_y) = match placement.anchor {
        AnchorPoint::TopLeft => (monitor.x, monitor.y),
        AnchorPoint::TopCenter => (
          monitor.x + (monitor_width / 2) - (window_size.width / 2),
          monitor.y,
        ),
        AnchorPoint::TopRight => {
          (monitor.x + monitor_width - window_size.width, monitor.y)
        }
        AnchorPoint::CenterLeft => (
          monitor.x,
          monitor.y + (monitor_height / 2) - (window_size.height / 2),
        ),
        AnchorPoint::Center => (
          monitor.x + (monitor_width / 2) - (window_size.width / 2),
          monitor.y + (monitor_height / 2) - (window_size.height / 2),
        ),
        AnchorPoint::CenterRight => (
          monitor.x + monitor_width - window_size.width,
          monitor.y + (monitor_height / 2) - (window_size.height / 2),
        ),
        AnchorPoint::BottomLeft => {
          (monitor.x, monitor.y + monitor_height - window_size.height)
        }
        AnchorPoint::BottomCenter => (
          monitor.x + (monitor_width / 2) - (window_size.width / 2),
          monitor.y + monitor_height - window_size.height,
        ),
        AnchorPoint::BottomRight => (
          monitor.x + monitor_width - window_size.width,
          monitor.y + monitor_height - window_size.height,
        ),
      };

      let offset_x = placement
        .offset_x
        .to_px_scaled(monitor_width, monitor.scale_factor);

      let offset_y = placement
        .offset_y
        .to_px_scaled(monitor_height, monitor.scale_factor);

      let window_position =
        PhysicalPosition::new(anchor_x + offset_x, anchor_y + offset_y);

      coordinates.push((window_size, window_position));
    }

    coordinates
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

  /// Closes all widgets of the given preset name.
  pub async fn stop_by_preset(
    &self,
    config_path: &PathBuf,
    preset_name: &str,
  ) -> anyhow::Result<()> {
    todo!()
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

      self.start_preset(widget_config).await?;
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
