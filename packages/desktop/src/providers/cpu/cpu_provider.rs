use serde::{Deserialize, Serialize};
use sysinfo::Components;

use crate::{
  common::SyncInterval,
  providers::{
    CommonProviderState, Provider, ProviderInputMsg, RuntimeType,
  },
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CpuProviderConfig {
  pub refresh_interval: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuOutput {
  pub frequency: u64,
  pub usage: f32,
  pub logical_core_count: usize,
  pub physical_core_count: usize,
  pub vendor: String,
  pub temperature: Option<f32>,
}

pub struct CpuProvider {
  config: CpuProviderConfig,
  common: CommonProviderState,
}

impl CpuProvider {
  pub fn new(
    config: CpuProviderConfig,
    common: CommonProviderState,
  ) -> CpuProvider {
    CpuProvider { config, common }
  }

  fn run_interval(&self) -> anyhow::Result<CpuOutput> {
    let mut sysinfo = self.common.sysinfo.blocking_lock();
    sysinfo.refresh_cpu();

    let temperature = Self::get_temperature();

    Ok(CpuOutput {
      usage: sysinfo.global_cpu_info().cpu_usage(),
      frequency: sysinfo.global_cpu_info().frequency(),
      logical_core_count: sysinfo.cpus().len(),
      physical_core_count: sysinfo
        .physical_core_count()
        .unwrap_or(sysinfo.cpus().len()),
      vendor: sysinfo.global_cpu_info().vendor_id().into(),
      temperature,
    })
  }

  fn get_temperature() -> Option<f32> {
    // Try platform-specific methods first, fall back to sysinfo.
    #[cfg(windows)]
    if let Some(temp) = Self::get_temperature_wmi() {
      return Some(temp);
    }

    // Fallback: sysinfo Components (works on Linux/macOS, rarely on Windows).
    let mut components = Components::new_with_refreshed_list();
    components.iter_mut().find_map(|c| {
      let label = c.label().to_lowercase();
      if label.contains("cpu")
        || label.contains("tctl")
        || label.contains("coretemp")
      {
        Some(c.temperature())
      } else {
        None
      }
    })
  }

  /// Query LibreHardwareMonitor or OpenHardwareMonitor via WMI.
  /// These tools expose sensor data through custom WMI namespaces.
  #[cfg(windows)]
  fn get_temperature_wmi() -> Option<f32> {
    use serde::Deserialize as _;

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "PascalCase")]
    struct WmiSensor {
      sensor_type: String,
      name: String,
      value: f32,
    }

    // Try LibreHardwareMonitor first, then OpenHardwareMonitor.
    for namespace in [
      "root\\LibreHardwareMonitor",
      "root\\OpenHardwareMonitor",
    ] {
      let con = wmi::COMLibrary::new()
        .and_then(|lib| {
          wmi::WMIConnection::with_namespace_path(namespace, lib)
        })
        .ok()?;

      let sensors: Vec<WmiSensor> = con
        .raw_query(
          "SELECT SensorType, Name, Value FROM Sensor \
           WHERE SensorType = 'Temperature'",
        )
        .ok()
        .unwrap_or_default();

      // Find CPU Package or CPU average temperature.
      let cpu_temp = sensors.iter().find_map(|s| {
        let name = s.name.to_lowercase();
        if s.sensor_type == "Temperature"
          && (name.contains("cpu package")
            || name.contains("core (tctl")
            || name.contains("cpu (tctl"))
        {
          Some(s.value)
        } else {
          None
        }
      });

      // If no package temp, try any CPU core temperature.
      let temp = cpu_temp.or_else(|| {
        sensors.iter().find_map(|s| {
          let name = s.name.to_lowercase();
          if s.sensor_type == "Temperature"
            && (name.starts_with("cpu core")
              || name.starts_with("core #"))
          {
            Some(s.value)
          } else {
            None
          }
        })
      });

      if temp.is_some() {
        return temp;
      }
    }

    None
  }
}

impl Provider for CpuProvider {
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
