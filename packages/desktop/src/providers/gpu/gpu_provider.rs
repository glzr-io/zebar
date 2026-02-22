use nvml_wrapper::Nvml;
use serde::{Deserialize, Serialize};

use crate::{
  common::SyncInterval,
  providers::{
    CommonProviderState, Provider, ProviderInputMsg, RuntimeType,
  },
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GpuProviderConfig {
  pub refresh_interval: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GpuOutput {
  pub name: String,
  pub temperature: Option<f32>,
  pub usage: Option<f32>,
  pub memory_used: Option<u64>,
  pub memory_total: Option<u64>,
}

pub struct GpuProvider {
  config: GpuProviderConfig,
  common: CommonProviderState,
}

impl GpuProvider {
  pub fn new(
    config: GpuProviderConfig,
    common: CommonProviderState,
  ) -> GpuProvider {
    GpuProvider { config, common }
  }

  fn run_interval(&self) -> anyhow::Result<GpuOutput> {
    let nvml = Nvml::init()?;
    let device = nvml.device_by_index(0)?;

    let name = device.name()?;

    let temperature = device
      .temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
      .ok()
      .map(|t| t as f32);

    let usage = device
      .utilization_rates()
      .ok()
      .map(|u| u.gpu as f32);

    let memory_info = device.memory_info().ok();
    let memory_used = memory_info.as_ref().map(|m| m.used);
    let memory_total = memory_info.as_ref().map(|m| m.total);

    Ok(GpuOutput {
      name,
      temperature,
      usage,
      memory_used,
      memory_total,
    })
  }
}

impl Provider for GpuProvider {
  fn runtime_type(&self) -> RuntimeType {
    RuntimeType::Sync
  }

  fn start_sync(&mut self) {
    let mut interval = SyncInterval::new(self.config.refresh_interval);

    loop {
      crossbeam::select! {
        recv(interval.tick()) -> _ => {
          let output = self.run_interval();
          self.common.emitter.emit_output(output);
        }
        recv(self.common.input.sync_rx) -> input => {
          if let Ok(ProviderInputMsg::Stop) = input {
            break;
          }
        }
      }
    }
  }
}
