use std::process::Command;

use anyhow::{Context, Result};
use regex::Regex;

#[derive(Debug)]
pub struct WifiHotstop {
  pub ssid: Option<String>,
  pub signal_strength: Option<u32>,
}

/// Runs the netsh command and parses the output to get the primary
/// interface's SSID and signal strength.
//
/// Unclear if the primary interface is always the default interface
/// returned by the netdev/defaultnet crate - requires more testing.
pub fn default_gateway_wifi() -> Result<WifiHotstop> {
  if cfg!(not(target_os = "windows")) {
    return Ok(WifiHotstop {
      ssid: None,
      signal_strength: None,
    });
  }

  let ssid_match = Regex::new(r"(?m)^\s*SSID\s*:\s*(.*?)\r?$").unwrap();

  let signal_match =
    Regex::new(r"(?m)^\s*Signal\s*:\s*(.*?)\r?$").unwrap();

  let signal_strip = Regex::new(r"(\d+)%").unwrap();

  let output = Command::new("netsh")
    .args(&["wlan", "show", "interfaces"])
    .output()
    .context("Could not run netsh.")?;

  let output = String::from_utf8_lossy(&output.stdout);

  let ssid = ssid_match
    .captures(&output)
    .map(|m| m.get(1).unwrap().as_str().to_string());
  let signal_str: Option<&str> = signal_match
    .captures(&output)
    .map(|m| m.get(1).unwrap().as_str());
  let signal: Option<u32> = signal_str
    .and_then(|s| signal_strip.captures(s))
    .map(|m| m.get(1).unwrap().as_str().parse().unwrap());

  return Ok(WifiHotstop {
    ssid,
    signal_strength: signal,
  });
}
