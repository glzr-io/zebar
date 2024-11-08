use std::{
  collections::HashMap,
  path::PathBuf,
  sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
  },
};

use anyhow::Context;
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

#[cfg(target_os = "windows")]
use crate::config::WidgetEdge;
use crate::{
  common::{PathExt, WindowExt},
  config::{
    AnchorPoint, Config, ReserveSpaceConfig, WidgetConfig, WidgetPlacement,
  },
  monitor_state::MonitorState,
};

/// Manages the creation of Zebar widgets.
pub struct WidgetFactory {
  /// Handle to the Tauri application.
  app_handle: AppHandle,

  _close_rx: broadcast::Receiver<String>,

  pub close_tx: broadcast::Sender<String>,

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

  /// How the widget was opened.
  pub open_options: WidgetOpenOptions,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WidgetOpenOptions {
  Standalone(WidgetPlacement),
  Preset(String),
}

struct WidgetCoordinates {
  size: PhysicalSize<i32>,
  position: PhysicalPosition<i32>,
  monitor_size: PhysicalSize<i32>,
  scale_factor: f32,
  anchor: AnchorPoint,
  offset: PhysicalPosition<i32>,
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

  /// Opens widget from a given config path.
  ///
  /// Config path must be absolute.
  pub async fn start_widget(
    &self,
    config_path: &PathBuf,
    open_options: &WidgetOpenOptions,
  ) -> anyhow::Result<()> {
    let (config_path, widget_config) = self
      .config
      .widget_config_by_path(config_path)
      .await
      .with_context(|| {
        format!("No config found at path '{}'.", config_path.display())
      })?;

    // No-op if preset is already open.
    if let WidgetOpenOptions::Preset(_) = open_options {
      let is_preset_open = {
        self
          .widget_states
          .lock()
          .await
          .values()
          .find(|state| {
            state.config_path == config_path
              && state.open_options == *open_options
          })
          .is_some()
      };

      if is_preset_open {
        return Ok(());
      }
    }

    // Extract placement from widget preset (if applicable).
    let placement = match open_options {
      WidgetOpenOptions::Standalone(placement) => placement,
      WidgetOpenOptions::Preset(name) => {
        &widget_config
          .presets
          .iter()
          .find(|preset| preset.name == *name)
          .with_context(|| {
            format!(
              "No preset with name '{}' at config '{}'.",
              name,
              config_path.display()
            )
          })?
          .placement
      }
    };

    for coordinates in self.widget_coordinates(placement).await {
      let new_count =
        self.widget_count.fetch_add(1, Ordering::Relaxed) + 1;

      // Use running widget count as a unique label for the Tauri window.
      let widget_id = format!("widget-{}", new_count);

      let html_path = config_path
        .parent()
        .map(|dir| dir.join(&widget_config.html_path))
        .filter(|path| path.exists())
        .with_context(|| {
          format!(
            "HTML file not found at '{}' for config '{}'.",
            widget_config.html_path.display(),
            config_path.display()
          )
        })?;

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
      .focused(widget_config.focused)
      .skip_taskbar(!widget_config.shown_in_taskbar)
      .visible_on_all_workspaces(true)
      .transparent(widget_config.transparent)
      .shadow(false)
      .decorations(false)
      .resizable(widget_config.resizable)
      .build()?;

      info!(
        "Positioning widget to {:?} {:?}",
        coordinates.size, coordinates.position
      );

      let _ = window.set_size(coordinates.size);
      let _ = window.set_position(coordinates.position);

      // On Windows, we need to set the position twice to account for
      // different monitor scale factors.
      #[cfg(target_os = "windows")]
      {
        let _ = window.set_size(coordinates.size);
        let _ = window.set_position(coordinates.position);
      }

      let state = WidgetState {
        id: widget_id.clone(),
        config: widget_config.clone(),
        config_path: config_path.clone(),
        html_path: html_path.clone(),
        open_options: open_options.clone(),
      };

      _ = window.eval(&format!(
        "window.__ZEBAR_STATE={0}; sessionStorage.setItem('ZEBAR_STATE', JSON.stringify({0}))",
        serde_json::to_string(&state)?
      ));

      // On Windows, Tauri's `skip_taskbar` option isn't 100% reliable,
      // so we also set the window as a tool window.
      #[cfg(target_os = "windows")]
      let _ = window
        .as_ref()
        .window()
        .set_tool_window(!widget_config.shown_in_taskbar);

      if placement.reserve_space.enabled {
        self.reserve_space(
          &window,
          &placement.reserve_space,
          &coordinates,
        )?;
      }

      // On MacOS, we need to set the window as above the menu bar for it
      // to truly be always on top.
      #[cfg(target_os = "macos")]
      {
        if widget_config.z_order == crate::config::ZOrder::TopMost {
          let _ = window.as_ref().window().set_above_menu_bar();
        }
      }

      {
        let mut widget_states = self.widget_states.lock().await;
        widget_states.insert(state.id.clone(), state.clone());
      }

      self.register_window_events(&window, widget_id)?;
      self.open_tx.send(state);
    }

    Ok(())
  }

  /// Reserve space for the app bar if enabled.
  fn reserve_space(
    &self,
    window: &tauri::WebviewWindow,
    reserve_space: &ReserveSpaceConfig,
    coords: &WidgetCoordinates,
  ) -> anyhow::Result<()> {
    let edge = reserve_space.edge.unwrap_or_else(|| {
      // Default to whichever edge the widget appears to be on.
      let widget_horizontal = coords.size.width > coords.size.height;

      match (coords.anchor, widget_horizontal) {
        (AnchorPoint::Center, true) => WidgetEdge::Top,
        (AnchorPoint::Center, false) => WidgetEdge::Left,
        (AnchorPoint::TopCenter, _) => WidgetEdge::Top,
        (AnchorPoint::CenterLeft, _) => WidgetEdge::Left,
        (AnchorPoint::CenterRight, _) => WidgetEdge::Right,
        (AnchorPoint::BottomCenter, _) => WidgetEdge::Bottom,
        (AnchorPoint::TopLeft, true) => WidgetEdge::Top,
        (AnchorPoint::TopLeft, false) => WidgetEdge::Left,
        (AnchorPoint::TopRight, true) => WidgetEdge::Top,
        (AnchorPoint::TopRight, false) => WidgetEdge::Right,
        (AnchorPoint::BottomLeft, true) => WidgetEdge::Bottom,
        (AnchorPoint::BottomLeft, false) => WidgetEdge::Left,
        (AnchorPoint::BottomRight, true) => WidgetEdge::Bottom,
        (AnchorPoint::BottomRight, false) => WidgetEdge::Right,
      }
    });

    // Total height of monitor perpendicular to the edge.
    let total_height = if edge.is_horizontal() {
      monitor_size.height
    } else {
      monitor_size.width
    };

    let thickness = if let Some(thickness) = &reserve_space.thickness {
      thickness.to_px_scaled(total_height, scale_factor)
    } else {
      match edge {
        WidgetEdge::Top => coords.offset.y + coords.size.height,
        WidgetEdge::Bottom => -coords.offset.y + coords.size.height,
        WidgetEdge::Left => coords.offset.x + coords.size.width,
        WidgetEdge::Right => -coords.offset.x + coords.size.width,
      }
    };

    let reserve_size = if edge.is_horizontal() {
      PhysicalSize::new(monitor_size.width, thickness)
    } else {
      PhysicalSize::new(thickness, monitor_size.height)
    };

    let reserve_position = match edge {
      WidgetEdge::Top => PhysicalPosition::new(0, offset),
      WidgetEdge::Bottom => {
        PhysicalPosition::new(0, monitor_size.height - thickness - offset)
      }
      WidgetEdge::Left => PhysicalPosition::new(offset, 0),
      WidgetEdge::Right => {
        PhysicalPosition::new(monitor_size.width - thickness - offset, 0)
      }
    };

    tracing::info!(
      "Reserving app bar space on {:?}: {:?} {:?}",
      edge,
      reserve_size,
      reserve_position
    );

    window.as_ref().window().allocate_app_bar(
      reserve_size,
      reserve_position,
      edge,
    )
  }

  /// Opens presets that are configured to be launched on startup.
  pub async fn startup(&self) -> anyhow::Result<()> {
    let startup_configs = self.config.startup_configs().await;

    for startup_config in startup_configs {
      self
        .start_widget(
          &startup_config.path,
          &WidgetOpenOptions::Preset(startup_config.preset),
        )
        .await?;
    }

    Ok(())
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
  ) -> anyhow::Result<()> {
    let widget_states = self.widget_states.clone();
    let close_tx = self.close_tx.clone();

    #[cfg(target_os = "windows")]
    let window_handle =
      window.hwnd().context("Failed to get window handle.")?.0 as _;

    window.on_window_event(move |event| {
      if let WindowEvent::Destroyed = event {
        let widget_states = widget_states.clone();
        let close_tx = close_tx.clone();
        let widget_id = widget_id.clone();

        // Ensure appbar space is deallocated on close.
        #[cfg(target_os = "windows")]
        crate::common::remove_app_bar(window_handle);

        task::spawn(async move {
          let mut widget_states = widget_states.lock().await;

          // Remove the widget state.
          let _ = widget_states.remove(&widget_id);

          // Broadcast the close event.
          if let Err(err) = close_tx.send(widget_id) {
            error!("Failed to send window close event: {:?}", err);
          }
        });
      }
    });

    Ok(())
  }

  /// Returns coordinates for window placement based on the given config.
  async fn widget_coordinates(
    &self,
    placement: &WidgetPlacement,
  ) -> Vec<WidgetCoordinates> {
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

      coordinates.push(WidgetCoordinates {
        size: window_size,
        position: window_position,
        monitor_size: PhysicalSize::new(monitor_width, monitor_height),
        scale_factor: monitor.scale_factor,
        anchor: placement.anchor,
        offset: PhysicalPosition::new(offset_x, offset_y),
      });
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
  pub async fn _stop_by_path(
    &self,
    config_path: &PathBuf,
  ) -> anyhow::Result<()> {
    let widget_states = self.states_by_path().await;

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
    let widget_states = self.states_by_path().await;

    let found_widget_states = widget_states
      .get(config_path)
      .context("No widgets found with the given config path.")?
      .iter()
      .filter(|state| {
        matches!(
          &state.open_options,
          WidgetOpenOptions::Preset(name) if name == preset_name
        )
      });

    for widget_state in found_widget_states {
      self.stop_by_id(&widget_state.id)?;
    }

    Ok(())
  }

  /// Relaunches all currently open widgets.
  pub async fn relaunch_all(&self) -> anyhow::Result<()> {
    let widget_ids =
      { self.widget_states.lock().await.keys().cloned().collect() };

    self.relaunch_by_ids(&widget_ids).await
  }

  /// Relaunches widgets with the given widget ID's.
  pub async fn relaunch_by_ids(
    &self,
    widget_ids: &Vec<String>,
  ) -> anyhow::Result<()> {
    let changed_states = {
      let mut widget_states = self.widget_states.lock().await;

      // Optimistically clear running widget states.
      widget_ids
        .iter()
        .filter_map(|id| widget_states.remove(id))
        .collect::<Vec<_>>()
    };

    for widget_state in changed_states {
      info!(
        "Relaunching widget #{} from {}",
        widget_state.id,
        widget_state.config_path.display()
      );

      let _ = self.stop_by_id(&widget_state.id);

      self
        .start_widget(
          &widget_state.config_path,
          &widget_state.open_options,
        )
        .await?;
    }

    Ok(())
  }

  /// Relaunches widgets with the given config paths.
  pub async fn relaunch_by_paths(
    &self,
    config_paths: &Vec<PathBuf>,
  ) -> anyhow::Result<()> {
    let widget_ids = {
      self
        .widget_states
        .lock()
        .await
        .iter()
        .filter(|(_, state)| config_paths.contains(&state.config_path))
        .map(|(id, _)| id.clone())
        .collect::<Vec<_>>()
    };

    self.relaunch_by_ids(&widget_ids).await
  }

  /// Returns widget states by their widget ID's.
  pub async fn states(&self) -> HashMap<String, WidgetState> {
    self.widget_states.lock().await.clone()
  }

  /// Returns widget states grouped by their config paths.
  pub async fn states_by_path(
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
