use std::{any::Any, sync::Arc};

use serde::{Deserialize, Serialize};
use sysinfo::{Disk, Disks};
use tokio::sync::Mutex;

use crate::{
  common::{to_iec_bytes, to_si_bytes},
  impl_interval_provider,
  providers::ProviderOutput,
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiskProviderConfig {
  pub refresh_interval: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskOutput {
  pub disks: Vec<DiskInner>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskInner {
  pub name: String,
  pub file_system: String,
  pub mount_point: String,
  pub total_space: DiskSizeMeasure,
  pub available_space: DiskSizeMeasure,
  pub is_removable: bool,
  pub disk_type: String,
}

pub struct DiskProvider {
  config: DiskProviderConfig,
  system: Arc<Mutex<Disks>>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskSizeMeasure {
  pub bytes: u64,
  pub si_value: f64,
  pub si_unit: String,
  pub iec_value: f64,
  pub iec_unit: String,
}

impl DiskProvider {
  pub fn new(
    config: DiskProviderConfig,
    system: Arc<Mutex<Disks>>,
  ) -> DiskProvider {
    DiskProvider { config, system }
  }

  fn refresh_interval_ms(&self) -> u64 {
    self.config.refresh_interval
  }

  async fn run_interval(&self) -> anyhow::Result<ProviderOutput> {
    let mut disks = self.system.lock().await;
    disks.refresh();

    let list: Vec<DiskInner> = disks
      .iter()
      .map(|disk| -> anyhow::Result<DiskInner> {
        Ok(DiskInner {
          name: disk.name().to_string_lossy().to_string(),
          file_system: disk.file_system().to_string_lossy().to_string(),
          mount_point: disk.mount_point().to_string_lossy().to_string(),
          total_space: Self::to_disk_size_measure(disk.total_space())?,
          available_space: Self::to_disk_size_measure(
            disk.available_space(),
          )?,
          is_removable: disk.is_removable(),
          disk_type: disk.kind().to_string(),
        })
      })
      .collect::<anyhow::Result<Vec<DiskInner>>>()?;

    let output = DiskOutput { disks: list };
    Ok(ProviderOutput::Disk(output))
  }

  fn to_disk_size_measure(bytes: u64) -> anyhow::Result<DiskSizeMeasure> {
    let (si_value, si_unit) = to_si_bytes(bytes as f64);
    let (iec_value, iec_unit) = to_iec_bytes(bytes as f64);

    Ok(DiskSizeMeasure {
      bytes,
      si_value,
      si_unit,
      iec_value,
      iec_unit,
    })
  }
}

impl_interval_provider!(DiskProvider, true);
