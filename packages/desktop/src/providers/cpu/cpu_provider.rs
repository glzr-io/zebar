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

/// Cached resources for temperature readings to avoid
/// re-initializing expensive system handles every poll.
struct TempCache {
  components: Components,
  #[cfg(windows)]
  wmi_connection: Option<wmi::WMIConnection>,
}

impl TempCache {
  fn new() -> Self {
    let components = Components::new_with_refreshed_list();

    #[cfg(windows)]
    return TempCache {
      components,
      wmi_connection: Self::init_wmi(),
    };

    #[cfg(not(windows))]
    TempCache { components }
  }

  #[cfg(windows)]
  fn init_wmi() -> Option<wmi::WMIConnection> {
    for namespace in [
      "root\\LibreHardwareMonitor",
      "root\\OpenHardwareMonitor",
    ] {
      if let Ok(con) =
        wmi::COMLibrary::new().and_then(|lib| {
          wmi::WMIConnection::with_namespace_path(
            namespace, lib,
          )
        })
      {
        return Some(con);
      }
    }
    None
  }
}

impl CpuProvider {
  pub fn new(
    config: CpuProviderConfig,
    common: CommonProviderState,
  ) -> CpuProvider {
    CpuProvider { config, common }
  }

  fn run_interval(
    &self,
    temp_cache: &mut TempCache,
  ) -> anyhow::Result<CpuOutput> {
    let mut sysinfo = self.common.sysinfo.blocking_lock();
    sysinfo.refresh_cpu();

    let temperature =
      Self::get_temperature(temp_cache);

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

  fn get_temperature(
    cache: &mut TempCache,
  ) -> Option<f32> {
    // Try platform-specific methods first, fall back to
    // sysinfo.
    #[cfg(windows)]
    {
      if let Some(temp) =
        Self::get_temperature_lhm_http()
      {
        return Some(temp);
      }
      if let Some(ref con) = cache.wmi_connection {
        if let Some(temp) =
          Self::get_temperature_wmi(con)
        {
          return Some(temp);
        }
      }
    }

    // Fallback: sysinfo Components (works on
    // Linux/macOS, rarely on Windows). Refresh cached
    // components instead of re-creating them.
    for c in cache.components.iter_mut() {
      c.refresh();
    }
    cache.components.iter().find_map(|c| {
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

  /// Query LibreHardwareMonitor's HTTP web server API.
  /// LHM v0.9.5+ has broken WMI support, so this is the primary
  /// method. Requires LHM running with Remote Web Server enabled
  /// (Options > Remote Web Server > Run).
  #[cfg(windows)]
  fn get_temperature_lhm_http() -> Option<f32> {
    use std::io::{BufRead, BufReader, Write};
    use std::net::TcpStream;
    use std::time::Duration;

    let mut stream =
      TcpStream::connect_timeout(
        &"127.0.0.1:8085".parse().ok()?,
        Duration::from_millis(100),
      )
      .ok()?;
    stream.set_read_timeout(Some(Duration::from_millis(500))).ok()?;
    stream
      .write_all(b"GET /data.json HTTP/1.0\r\nHost: localhost\r\n\r\n")
      .ok()?;

    let reader = BufReader::new(stream);
    let mut body = String::new();
    let mut in_body = false;
    for line in reader.lines() {
      let line = line.ok()?;
      if in_body {
        body.push_str(&line);
      } else if line.is_empty() {
        in_body = true;
      }
    }

    let json: serde_json::Value = serde_json::from_str(&body).ok()?;
    Self::find_cpu_temp_in_lhm_json(&json)
  }

  /// Recursively search the LHM JSON tree for a CPU temperature node.
  #[cfg(windows)]
  fn find_cpu_temp_in_lhm_json(
    node: &serde_json::Value,
  ) -> Option<f32> {
    let text = node.get("Text")?.as_str()?;
    let children = node.get("Children")?.as_array()?;

    // Look for CPU hardware nodes (e.g. "AMD Ryzen ...", "Intel Core ...").
    let text_lower = text.to_lowercase();
    let is_cpu_hardware = text_lower.contains("ryzen")
      || text_lower.contains("intel")
      || text_lower.contains("core")
      || text_lower.starts_with("cpu");

    if is_cpu_hardware {
      // Find the "Temperatures" child group under this CPU.
      for child in children {
        let child_text =
          child.get("Text").and_then(|t| t.as_str()).unwrap_or("");
        if child_text == "Temperatures" {
          if let Some(temps) =
            child.get("Children").and_then(|c| c.as_array())
          {
            // Prefer "Core (Tctl/Tdie)" or "CPU Package", then any temp.
            let preferred = temps.iter().find_map(|t| {
              let name =
                t.get("Text").and_then(|n| n.as_str()).unwrap_or("");
              let name_lower = name.to_lowercase();
              if name_lower.contains("tctl")
                || name_lower.contains("cpu package")
              {
                Self::parse_lhm_temp_value(t)
              } else {
                None
              }
            });
            if preferred.is_some() {
              return preferred;
            }
            // Fall back to first temperature with a valid reading.
            let fallback =
              temps.iter().find_map(|t| Self::parse_lhm_temp_value(t));
            if fallback.is_some() {
              return fallback;
            }
          }
        }
      }
    }

    // Recurse into children.
    for child in children {
      if let Some(temp) = Self::find_cpu_temp_in_lhm_json(child) {
        return Some(temp);
      }
    }

    None
  }

  /// Parse a temperature value string like "48.4 °C" into f32.
  #[cfg(windows)]
  fn parse_lhm_temp_value(node: &serde_json::Value) -> Option<f32> {
    let value_str = node.get("Value")?.as_str()?;
    value_str
      .replace(',', ".")
      .split(|c: char| !c.is_ascii_digit() && c != '.' && c != '-')
      .next()?
      .parse::<f32>()
      .ok()
      .filter(|&v| v > 0.0 && v < 150.0)
  }

  /// Query LibreHardwareMonitor or OpenHardwareMonitor via WMI.
  /// These tools expose sensor data through custom WMI namespaces.
  /// Note: WMI is broken in LHM v0.9.5+, but kept as a fallback
  /// for older versions and OpenHardwareMonitor.
  #[cfg(windows)]
  fn get_temperature_wmi(
    con: &wmi::WMIConnection,
  ) -> Option<f32> {
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "PascalCase")]
    struct WmiSensor {
      sensor_type: String,
      name: String,
      value: f32,
    }

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
    cpu_temp.or_else(|| {
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
    })
  }
}

impl Provider for CpuProvider {
  fn runtime_type(&self) -> RuntimeType {
    RuntimeType::Sync
  }

  fn start_sync(&mut self) {
    let mut temp_cache = TempCache::new();
    let mut interval =
      SyncInterval::new(self.config.refresh_interval);

    loop {
      crossbeam::select! {
        recv(interval.tick()) -> _ => {
          let output =
            self.run_interval(&mut temp_cache);
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
