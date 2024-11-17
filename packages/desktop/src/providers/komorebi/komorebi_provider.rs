use std::{
  io::{BufReader, Read},
  time::Duration,
};

use anyhow::Context;
use async_trait::async_trait;
use komorebi_client::{
  Container, Monitor, SocketMessage, Window, Workspace,
};
use serde::{Deserialize, Serialize};
use tracing::debug;

use super::{
  KomorebiContainer, KomorebiLayout, KomorebiLayoutFlip, KomorebiMonitor,
  KomorebiWindow, KomorebiWorkspace,
};
use crate::providers::{
  CommonProviderState, Provider, ProviderOutput, RuntimeType,
};

const SOCKET_NAME: &str = "zebar.sock";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KomorebiProviderConfig {}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KomorebiOutput {
  pub all_monitors: Vec<KomorebiMonitor>,
  pub focused_monitor_index: usize,
}

pub struct KomorebiProvider {
  common: CommonProviderState,
}

impl KomorebiProvider {
  pub fn new(
    _config: KomorebiProviderConfig,
    common: CommonProviderState,
  ) -> KomorebiProvider {
    KomorebiProvider { common }
  }

  fn create_socket(&mut self) -> anyhow::Result<()> {
    let socket = komorebi_client::subscribe(SOCKET_NAME)
      .context("Failed to initialize Komorebi socket.")?;

    debug!("Connected to Komorebi socket.");

    for incoming in socket.incoming() {
      debug!("Incoming Komorebi socket message.");

      match incoming {
        Ok(stream) => {
          let mut buffer = Vec::new();
          let mut reader = BufReader::new(stream);

          // Shutdown has been sent.
          if matches!(reader.read_to_end(&mut buffer), Ok(0)) {
            debug!("Komorebi shutdown.");

            // Attempt to reconnect to Komorebi.
            while komorebi_client::send_message(
              &SocketMessage::AddSubscriberSocket(SOCKET_NAME.to_string()),
            )
            .is_err()
            {
              debug!("Attempting to reconnect to Komorebi.");
              std::thread::sleep(Duration::from_secs(15));
            }
          }

          // Transform and emit the incoming Komorebi state.
          if let Ok(notification) =
            serde_json::from_str::<komorebi_client::Notification>(
              &String::from_utf8(buffer).unwrap(),
            )
          {
            self.common.emit_result_tx.send(
              Ok(ProviderOutput::Komorebi(Self::transform_response(
                notification.state,
              )))
              .into(),
            );
          }
        }
        Err(_) => self.common.emit_result_tx.send(
          Err(anyhow::anyhow!("Failed to read Komorebi stream.")).into(),
        ),
      }
    }

    Ok(())
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
  fn runtime_type(&self) -> RuntimeType {
    RuntimeType::Sync
  }

  fn start_sync(&mut self) {
    if let Err(err) = self.create_socket() {
      self.common.emit_result_tx.try_send(Err(err).into());
    }
  }
}
