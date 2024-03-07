use anyhow::Context;
use regex::Regex;

// Runs the netsh command and parses the output
// to get the primary interface's SSID and signal strength.
//
// Unclear if the primary interface is always the default interface
// returned by the netdev/defaultnet crate - requires more testing.

#[derive(Debug)]
pub struct NetshInfo {
  pub ssid: Option<String>,
  pub signal: Option<String>,
}

pub fn get_primary_interface_ssid_and_strength(
) -> anyhow::Result<NetshInfo> {
  let ssid_match = Regex::new(r"(?m)^\s*SSID\s*:\s*(.*?)\r?$").unwrap();

  let signal_match =
    Regex::new(r"(?m)^\s*Signal\s*:\s*(.*?)\r?$").unwrap();

  use std::os::windows::process::CommandExt;
  let output = std::process::Command::new("netsh")
    .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
    .args(&["wlan", "show", "interfaces"])
    .output()
    .context("could not run netsh")?;
  let output = String::from_utf8_lossy(&output.stdout);
  let ssid = ssid_match
    .captures(&output)
    .map(|m| m.get(1).unwrap().as_str().to_string());
  let signal: Option<String> = signal_match
    .captures(&output)
    .map(|m| m.get(1).unwrap().as_str().to_string());
  return Ok(NetshInfo { ssid, signal });
}
