use std::{
  io::{BufRead, BufReader},
  sync::Arc,
  time::Duration,
};

use async_trait::async_trait;
use komorebi_client::{Container, Monitor, Window, Workspace};
use tokio::{sync::mpsc::Sender, task::AbortHandle};
use tracing::debug;

use super::{
  KomorebiContainer, KomorebiLayout, KomorebiLayoutFlip, KomorebiMonitor,
  KomorebiProviderConfig, KomorebiWindow, KomorebiWorkspace,
};
use crate::providers::{
  komorebi::KomorebiOutput, provider::Provider,
  provider_ref::ProviderResult, variables::ProviderOutput,
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

  fn transform_response(state: komorebi_client::State) -> KomorebiOutput {
    let all_monitors = state
      .monitors
      .elements()
      .into_iter()
      .map(Self::transform_monitor)
      .collect();

    KomorebiOutput {
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
  async fn on_start(
    &mut self,
    config_hash: &str,
    emit_result_tx: Sender<ProviderResult>,
  ) {
    let config_hash = config_hash.to_string();

    let socket = komorebi_client::subscribe(SOCKET_NAME).unwrap();
    debug!("Connected to Komorebi socket.");

    for incoming in socket.incoming() {
      debug!("Incoming Komorebi socket message.");

      match incoming {
        Ok(data) => {
          let reader = BufReader::new(data.try_clone().unwrap());

          for line in reader.lines().flatten() {
            if let Ok(notification) =
              serde_json::from_str::<komorebi_client::Notification>(&line)
            {
              // Transform and emit the incoming Komorebi state.
              _ = emit_result_tx
                .send(ProviderOutput {
                  config_hash: config_hash.clone(),
                  variables: OutputResult::Data(ProviderOutput::Komorebi(
                    Self::transform_response(notification.state),
                  )),
                })
                .await;
            }
          }
        }
        Err(error) => {
          _ = emit_result_tx
            .send(ProviderOutput {
              config_hash: config_hash.to_string(),
              variables: OutputResult::Error(error.to_string()),
            })
            .await;
        }
      }
    }
  }
}
