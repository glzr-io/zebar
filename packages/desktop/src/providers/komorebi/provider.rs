use std::{
  io::{BufRead, BufReader},
  sync::Arc,
  time::Duration,
};

use async_trait::async_trait;
use komorebi_client::{Container, Monitor, Window, Workspace};
use tokio::{
  sync::mpsc::Sender,
  task::{self, AbortHandle},
};
use tracing::debug;

use crate::providers::{
  komorebi::KomorebiVariables,
  manager::{ProviderOutput, VariablesResult},
  provider::Provider,
  variables::ProviderVariables,
};

use super::{
  KomorebiContainer, KomorebiLayout, KomorebiLayoutFlip, KomorebiMonitor,
  KomorebiProviderConfig, KomorebiWindow, KomorebiWorkspace,
};

const SOCKET_NAME: &str = "zebar.sock";

pub struct KomorebiProvider {
  pub config: Arc<KomorebiProviderConfig>,
  abort_handle: Option<AbortHandle>,
}

impl KomorebiProvider {
  pub fn new(config: KomorebiProviderConfig) -> KomorebiProvider {
    KomorebiProvider {
      config: Arc::new(config),
      abort_handle: None,
    }
  }

  fn transform_response(
    state: komorebi_client::State,
  ) -> KomorebiVariables {
    let all_monitors = state
      .monitors
      .elements()
      .into_iter()
      .map(Self::transform_monitor)
      .collect();

    KomorebiVariables {
      all_monitors,
      focused_monitor_index: state.monitors.focused_idx(),
    }
  }

  fn transform_monitor(monitor: &Monitor) -> KomorebiMonitor {
    KomorebiMonitor {
      id: monitor.id(),
      name: monitor.name().to_string(),
      device_id: monitor.device_id().clone(),
      focused_workspace_index: monitor.focused_workspace_idx(),
      size: *monitor.size(),
      work_area_size: *monitor.work_area_size(),
      work_area_offset: monitor.work_area_offset(),
      workspaces: monitor
        .workspaces()
        .into_iter()
        .map(Self::transform_workspace)
        .collect(),
    }
  }

  fn transform_workspace(workspace: &Workspace) -> KomorebiWorkspace {
    KomorebiWorkspace {
      container_padding: workspace.container_padding(),
      floating_windows: workspace
        .floating_windows()
        .into_iter()
        .map(Self::transform_window)
        .collect(),
      focused_container_index: workspace.focused_container_idx(),
      latest_layout: (*workspace.latest_layout()).clone(),
      layout: KomorebiLayout::from((*workspace.layout()).clone()),
      layout_flip: workspace.layout_flip().map(KomorebiLayoutFlip::from),
      name: workspace.name().clone(),
      maximized_window: workspace
        .maximized_window()
        .map(|w| Self::transform_window(&w)),
      monocle_container: workspace
        .monocle_container()
        .as_ref()
        .map(|c| Self::transform_container(&c)),
      tiling_containers: workspace
        .containers()
        .into_iter()
        .map(Self::transform_container)
        .collect(),
      workspace_padding: workspace.workspace_padding(),
    }
  }

  fn transform_container(container: &Container) -> KomorebiContainer {
    KomorebiContainer {
      id: container.id().to_string(),
      windows: container
        .windows()
        .iter()
        .map(Self::transform_window)
        .collect(),
    }
  }

  fn transform_window(window: &Window) -> KomorebiWindow {
    KomorebiWindow {
      class: window.class().ok(),
      exe: window.exe().ok(),
      hwnd: window.hwnd().0 as u64,
      title: window.title().ok(),
    }
  }
}

#[async_trait]
impl Provider for KomorebiProvider {
  // State should always be up to date.
  fn min_refresh_interval(&self) -> Duration {
    Duration::MAX
  }

  async fn on_start(
    &mut self,
    config_hash: String,
    emit_output_tx: Sender<ProviderOutput>,
  ) {
    let task_handle = task::spawn(async move {
      let socket = komorebi_client::subscribe(SOCKET_NAME).unwrap();
      debug!("Connected to Komorebi socket.");

      for incoming in socket.incoming() {
        debug!("Incoming Komorebi socket message.");

        match incoming {
          Ok(data) => {
            let reader = BufReader::new(data.try_clone().unwrap());

            for line in reader.lines().flatten() {
              if let Ok(notification) = serde_json::from_str::<
                komorebi_client::Notification,
              >(&line)
              {
                // Transform and emit the incoming Komorebi state.
                _ = emit_output_tx
                  .send(ProviderOutput {
                    config_hash: config_hash.clone(),
                    variables: VariablesResult::Data(
                      ProviderVariables::Komorebi(
                        Self::transform_response(notification.state),
                      ),
                    ),
                  })
                  .await;
              }
            }
          }
          Err(error) => {
            _ = emit_output_tx
              .send(ProviderOutput {
                config_hash: config_hash.clone(),
                variables: VariablesResult::Error(error.to_string()),
              })
              .await;
          }
        }
      }
    });

    self.abort_handle = Some(task_handle.abort_handle());
    _ = task_handle.await;
  }

  async fn on_refresh(
    &mut self,
    _config_hash: String,
    _emit_output_tx: Sender<ProviderOutput>,
  ) {
    // No-op.
  }

  async fn on_stop(&mut self) {
    if let Some(handle) = &self.abort_handle {
      handle.abort();
    }
  }
}
