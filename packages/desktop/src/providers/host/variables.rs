use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HostOutput {
  pub hostname: Option<String>,
  pub os_name: Option<String>,
  pub os_version: Option<String>,
  pub friendly_os_version: Option<String>,
  pub boot_time: u64,
  pub uptime: u64,
}
