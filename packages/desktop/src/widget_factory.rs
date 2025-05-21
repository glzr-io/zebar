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
  self, AppHandle, Manager, PhysicalPosition, PhysicalSize, WebviewUrl,
  WebviewWindowBuilder, WindowEvent,
};
use tokio::{
  sync::{broadcast, Mutex},
  task,
};
use tracing::{error, info};

#[cfg(target_os = "macos")]
use crate::common::macos::WindowExtMacOs;
#[cfg(target_os = "windows")]
use crate::common::windows::{remove_app_bar, WindowExtWindows};
use crate::{
  app_settings::AppSettings,
  asset_server::create_init_url,
  common::PathExt,
  config::{
    AnchorPoint, Config, DockConfig, DockEdge, WidgetConfig, WidgetPack,
    WidgetPlacement,
  },
  monitor_state::{Monitor, MonitorState},
};

/// Manages the creation of Zebar widgets.
#[derive(Debug)]
pub struct WidgetFactory {
  /// Handle to the Tauri application.
  app_handle: AppHandle,

  /// Reference to `AppSettings`.
  app_settings: Arc<AppSettings>,

  _close_rx: broadcast::Receiver<String>,

  pub close_tx: broadcast::Sender<String>,

  /// Reference to `Config`.
  config: Arc<Config>,

  _open_rx: broadcast::Receiver<WidgetState>,

  pub open_tx: broadcast::Sender<WidgetState>,

  /// Reference to `MonitorState`.
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

  /// Name of the widget.
  pub name: String,

  /// Unique identifier for the widget pack.
  pub pack_id: String,

  /// Whether the widget is a preview.
  pub is_preview: bool,

  /// Handle to the underlying Tauri window.
  ///
  /// This is only available on Windows.
  pub window_handle: Option<isize>,

  /// User-defined config for the widget.
  pub config: WidgetConfig,

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
  offset: PhysicalPosition<i32>,
  anchor: AnchorPoint,
  monitor: Monitor,
}

impl WidgetCoordinates {
  /// Gets which monitor edge (top, bottom, left, right) the widget is
  /// closest to.
  ///
  /// This is determined by dividing the monitor into four triangular
  /// quadrants (forming an "X") and checking which quadrant contains the
  /// widget's center point.
  fn closest_edge(&self) -> DockEdge {
    let widget_center = PhysicalPosition::new(
      self.position.x + (self.size.width / 2),
      self.position.y + (self.size.height / 2),
    );

    let monitor_center = PhysicalPosition::new(
      self.monitor.x + (self.monitor.width as i32 / 2),
      self.monitor.y + (self.monitor.height as i32 / 2),
    );

    // Get relative position from monitor center.
    let delta_x = widget_center.x - monitor_center.x;
    let delta_y = widget_center.y - monitor_center.y;

    match delta_x.abs() > delta_y.abs() {
      // Widget is in left or right triangle.
      true => {
        if delta_x > 0 {
          DockEdge::Right
        } else {
          DockEdge::Left
        }
      }
      // Widget is in top or bottom triangle.
      false => {
        if delta_y > 0 {
          DockEdge::Bottom
        } else {
          DockEdge::Top
        }
      }
    }
  }
}

impl WidgetFactory {
  /// Creates a new `WidgetFactory` instance.
  pub fn new(
    app_handle: &AppHandle,
    app_settings: Arc<AppSettings>,
    config: Arc<Config>,
    monitor_state: Arc<MonitorState>,
  ) -> Self {
    let (open_tx, _open_rx) = broadcast::channel(16);
    let (close_tx, _close_rx) = broadcast::channel(16);

    Self {
      app_handle: app_handle.clone(),
      app_settings,
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

  /// Opens widget by a given widget pack ID and widget name.
  pub async fn start_widget_by_id(
    &self,
    pack_id: &str,
    widget_name: &str,
    open_options: &WidgetOpenOptions,
    is_preview: bool,
  ) -> anyhow::Result<()> {
    let widget_pack = self
      .config
      .widget_pack_by_id(pack_id)
      .await
      .with_context(|| {
        format!("No widget pack found for '{}'.", pack_id)
      })?;

    self
      .start_widget_by_pack(
        &widget_pack,
        widget_name,
        open_options,
        is_preview,
      )
      .await
  }

  /// Opens widget from a resolved widget pack and widget name.
  pub async fn start_widget_by_pack(
    &self,
    widget_pack: &WidgetPack,
    widget_name: &str,
    open_options: &WidgetOpenOptions,
    is_preview: bool,
  ) -> anyhow::Result<()> {
    let widget_config = widget_pack
      .config
      .widgets
      .iter()
      .find(|entry| entry.name == widget_name)
      .with_context(|| {
        format!(
          "No widget named '{}' found in widget pack '{}'.",
          widget_name, widget_pack.id
        )
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
            state.pack_id == widget_pack.id
              && state.name == widget_config.name
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
              widget_pack.config_path.display()
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

      info!(
        "Creating window {} for {} from {}",
        widget_id, widget_name, widget_pack.id
      );

      let html_path =
        widget_pack.directory_path.join(&widget_config.html_path);

      if !html_path.exists() {
        bail!(
          "HTML file not found at '{}' for config '{}'.",
          widget_config.html_path.display(),
          widget_pack.config_path.display()
        )
      }

      let webview_url = WebviewUrl::External(
        create_init_url(&widget_pack.directory_path, &html_path).await?,
      );

      let mut state = WidgetState {
        id: widget_id.clone(),
        name: widget_name.to_string(),
        pack_id: widget_pack.id.clone(),
        window_handle: None,
        config: widget_config.clone(),
        html_path: html_path.canonicalize_pretty()?,
        open_options: open_options.clone(),
        is_preview,
      };

      let window = WebviewWindowBuilder::new(
        &self.app_handle,
        widget_id.clone(),
        webview_url,
      )
      .title(format!("Zebar - {} / {}", widget_pack.id, widget_name))
      .focused(widget_config.focused)
      .skip_taskbar(!widget_config.shown_in_taskbar)
      .visible_on_all_workspaces(true)
      .transparent(widget_config.transparent)
      .shadow(false)
      .decorations(false)
      .resizable(widget_config.resizable)
      .initialization_script(&self.initialization_script(&state)?)
      // Widgets from the same pack share their browser cache (i.e.
      // `localStorage`, `sessionStorage`, SW cache, etc.).
      // TODO: Add this as an ext method on the Tauri window.
      .data_directory(
        self.app_settings.webview_cache_dir.join(&widget_pack.id),
      )
      .build()?;

      // Widget coordinates might be modified when docked to an edge.
      let (size, position) = match placement.dock_to_edge.enabled {
        false => (coordinates.size, coordinates.position),
        true => self.dock_to_edge(
          &window,
          &placement.dock_to_edge,
          &coordinates,
        )?,
      };

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

      // On Windows, Tauri's `skip_taskbar` option isn't 100% reliable,
      // so we also set the window as a tool window.
      #[cfg(target_os = "windows")]
      let _ = window
        .as_ref()
        .window()
        .set_tool_window(!widget_config.shown_in_taskbar);

      // On MacOS, we need to set the window as above the menu bar for it
      // to truly be always on top.
      #[cfg(target_os = "macos")]
      {
        if widget_config.z_order == crate::config::ZOrder::TopMost {
          let _ = window.as_ref().window().set_above_menu_bar();
        }
      }

      #[cfg(target_os = "windows")]
      {
        state.window_handle = {
          let handle =
            window.hwnd().context("Failed to get window handle.")?;

          Some(handle.0 as isize)
        };
      }

      {
        let mut widget_states = self.widget_states.lock().await;
        widget_states.insert(state.id.clone(), state.clone());
      }

      self.register_window_events(&window, widget_id)?;
      self.open_tx.send(state)?;
    }

    Ok(())
  }

  /// Dock the widget window to a given edge. This might result in the
  /// window being resized or repositioned (e.g. if a window is already
  /// docked to the given edge).
  ///
  /// Returns the new window size and position.
  fn dock_to_edge(
    &self,
    window: &tauri::WebviewWindow,
    dock_config: &DockConfig,
    coords: &WidgetCoordinates,
  ) -> anyhow::Result<(PhysicalSize<i32>, PhysicalPosition<i32>)> {
    #[cfg(not(target_os = "windows"))]
    {
      return Ok((coords.size, coords.position));
    }

    #[cfg(target_os = "windows")]
    {
      // Disallow docking with a centered anchor point. Doesn't make sense.
      if coords.anchor == AnchorPoint::Center {
        return Ok((coords.size, coords.position));
      }

      let edge = dock_config.edge.unwrap_or_else(|| coords.closest_edge());

      // Offset from the monitor edge to the window.
      let offset = match edge {
        DockEdge::Top => coords.offset.y,
        DockEdge::Bottom => -coords.offset.y,
        DockEdge::Left => coords.offset.x,
        DockEdge::Right => -coords.offset.x,
      };

      // Length of the window perpendicular to the monitor edge.
      let window_length = if edge.is_horizontal() {
        coords.size.height
      } else {
        coords.size.width
      };

      // Margin to reserve *after* the window. Can be negative, but should
      // not be smaller than the size of the window.
      let window_margin = dock_config
        .window_margin
        .to_px_scaled(window_length as i32, coords.monitor.scale_factor)
        .clamp(-coords.size.height, i32::MAX);

      let monitor_length = if edge.is_horizontal() {
        coords.monitor.height
      } else {
        coords.monitor.width
      };

      // Prevent the reserved amount from exceeding 50% of the monitor
      // size. This maximum is arbitrary but should be sufficient for
      // most cases.
      let reserved_length = (offset + window_length + window_margin)
        .clamp(0, monitor_length as i32 / 2);

      let reserve_size = if edge.is_horizontal() {
        PhysicalSize::new(coords.monitor.width as i32, reserved_length)
      } else {
        PhysicalSize::new(reserved_length, coords.monitor.height as i32)
      };

      let reserve_position = match edge {
        DockEdge::Top | DockEdge::Left => {
          PhysicalPosition::new(coords.monitor.x, coords.monitor.y)
        }
        DockEdge::Bottom => PhysicalPosition::new(
          coords.monitor.x,
          coords.monitor.y + coords.monitor.height as i32
            - reserved_length,
        ),
        DockEdge::Right => PhysicalPosition::new(
          coords.monitor.x + coords.monitor.width as i32 - reserved_length,
          coords.monitor.y,
        ),
      };

      let (allocated_size, allocated_position) = window
        .as_ref()
        .window()
        .allocate_app_bar(reserve_size, reserve_position, edge)?;

      // Adjust the size to account for the window margin.
      let final_size = if edge.is_horizontal() {
        PhysicalSize::new(
          allocated_size.width,
          allocated_size.height.saturating_sub(window_margin.abs()),
        )
      } else {
        PhysicalSize::new(
          allocated_size.width.saturating_sub(window_margin.abs()),
          allocated_size.height,
        )
      };

      // Adjust position if we're docked to bottom or right edge to account
      // for the size reduction.
      let final_position = match edge {
        DockEdge::Bottom => PhysicalPosition::new(
          allocated_position.x,
          allocated_position.y
            + (allocated_size.height - final_size.height),
        ),
        DockEdge::Right => PhysicalPosition::new(
          allocated_position.x + (allocated_size.width - final_size.width),
          allocated_position.y,
        ),
        _ => allocated_position,
      };

      tracing::info!(
        "Docked widget to edge '{:?}' with size {:?} and position {:?}.",
        edge,
        final_size,
        final_position
      );

      Ok((final_size, final_position))
    }
  }

  /// Opens presets that are configured to be launched on startup.
  pub async fn startup(&self) -> anyhow::Result<()> {
    let startup_configs = self.app_settings.startup_configs().await;

    for startup_config in startup_configs {
      self
        .start_widget_by_id(
          &startup_config.pack_id,
          &startup_config.widget_name,
          &WidgetOpenOptions::Preset(startup_config.preset),
          false,
        )
        .await?;
    }

    Ok(())
  }

  fn initialization_script(
    &self,
    state: &WidgetState,
  ) -> anyhow::Result<String> {
    let state_script =
      format!("window.__ZEBAR_STATE={};", serde_json::to_string(state)?);

    let sw_script = include_str!("../resources/initialization-script.js");

    Ok(format!("{state_script}\n{sw_script}"))
  }

  /// Registers window events for a given widget.
  fn register_window_events(
    &self,
    window: &tauri::WebviewWindow,
    widget_id: String,
  ) -> anyhow::Result<()> {
    let widget_states = self.widget_states.clone();
    let close_tx = self.close_tx.clone();

    window.on_window_event(move |event| {
      if let WindowEvent::Destroyed = event {
        let widget_states = widget_states.clone();
        let close_tx = close_tx.clone();
        let widget_id = widget_id.clone();

        task::spawn(async move {
          let mut widget_states = widget_states.lock().await;

          // Remove the widget state.
          let state = widget_states.remove(&widget_id);

          // Ensure appbar space is deallocated on close.
          #[cfg(target_os = "windows")]
          {
            if let Some(window_handle) =
              state.and_then(|state| state.window_handle)
            {
              let _ = remove_app_bar(window_handle);
            }
          }

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
        offset: PhysicalPosition::new(offset_x, offset_y),
        monitor: monitor.clone(),
        anchor: placement.anchor,
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

  /// Closes all widgets of the given preset name.
  pub async fn stop_by_preset(
    &self,
    pack_id: &str,
    widget_name: &str,
    preset_name: &str,
  ) -> anyhow::Result<()> {
    let widget_states = self.states().await;

    let found_widget_states = widget_states.values().filter(|state| {
      state.pack_id == *pack_id
        && state.name == *widget_name
        && matches!(
          &state.open_options,
          WidgetOpenOptions::Preset(name) if name == preset_name
        )
    });

    for widget_state in found_widget_states {
      self.stop_by_id(&widget_state.id)?;
    }

    Ok(())
  }

  /// Stops any currently running preview widget(s).
  pub async fn stop_all_previews(&self) -> anyhow::Result<()> {
    tracing::info!("Stopping all preview widgets.");

    // Find widget states marked as previews.
    let preview_widget_ids = {
      let widget_states = self.widget_states.lock().await;
      widget_states
        .iter()
        .filter(|(_, state)| state.is_preview)
        .map(|(id, _)| id.clone())
        .collect::<Vec<_>>()
    };

    // Stop each preview widget.
    for widget_id in preview_widget_ids {
      self.stop_by_id(&widget_id)?;
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
        .inspect(|widget_state| {
          // Need to clean up any appbars prior to restarting.
          #[cfg(target_os = "windows")]
          {
            if let Some(window_handle) = widget_state.window_handle {
              let _ = remove_app_bar(window_handle);
            }
          }
        })
        .collect::<Vec<_>>()
    };

    for widget_state in &changed_states {
      info!(
        "Relaunching widget {} from {} (#{}).",
        widget_state.name, widget_state.pack_id, widget_state.id
      );

      let _ = self.stop_by_id(&widget_state.id);

      self
        .start_widget_by_id(
          &widget_state.pack_id,
          &widget_state.name,
          &widget_state.open_options,
          false,
        )
        .await?;
    }

    Ok(())
  }

  /// Relaunches widgets with the given config paths.
  pub async fn relaunch_by_name(
    &self,
    pack_id: &str,
    widget_name: &str,
  ) -> anyhow::Result<()> {
    let widget_ids = {
      self
        .widget_states
        .lock()
        .await
        .iter()
        .filter(|(_, state)| {
          state.pack_id == *pack_id && state.name == *widget_name
        })
        .map(|(id, _)| id.clone())
        .collect::<Vec<_>>()
    };

    self.relaunch_by_ids(&widget_ids).await
  }

  /// Clears the cache for all open widgets.
  pub fn clear_cache(&self) {
    for (_, window) in self.app_handle.webview_windows() {
      // Post a message to the service worker for clearing the cache.
      _ = window.eval(
        r"
        if ('serviceWorker' in navigator) {
          navigator.serviceWorker.ready.then(sw => {
            sw.active?.postMessage({ type: 'CLEAR_CACHE' });
          });
        }",
      );
    }
  }

  /// Returns widget states by their widget ID's.
  pub async fn states(&self) -> HashMap<String, WidgetState> {
    self.widget_states.lock().await.clone()
  }

  /// Returns a widget state by its widget ID.
  pub async fn state_by_id(&self, widget_id: &str) -> Option<WidgetState> {
    self.widget_states.lock().await.get(widget_id).cloned()
  }
}
