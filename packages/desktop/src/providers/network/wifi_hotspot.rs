use std::{os::windows::process::CommandExt, process::Command};

use anyhow::Context;
use regex::Regex;
#[cfg(target_os = "windows")]
use windows::Win32::System::Threading::CREATE_NO_WINDOW;

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
pub fn default_gateway_wifi() -> anyhow::Result<WifiHotstop> {
  #[cfg(not(target_os = "windows"))]
  {
    Ok(WifiHotstop {
      ssid: None,
      signal_strength: None,
    })
  }
  #[cfg(target_os = "windows")]
  {
    let ssid_match = Regex::new(r"(?m)^\s*SSID\s*:\s*(.*?)\r?$").unwrap();

    let signal_match =
      Regex::new(r"(?m)^\s*Signal\s*:\s*(.*?)\r?$").unwrap();

    let signal_strip = Regex::new(r"(\d+)%").unwrap();

    let output = Command::new("netsh")
      .args(&["wlan", "show", "interfaces"])
      .creation_flags(CREATE_NO_WINDOW.0)
      .output()
      .context("Could not run netsh.")?;

    let output = String::from_utf8_lossy(&output.stdout);

    let ssid = ssid_match
      .captures(&output)
      .context("Failed to parse WiFi hotspot SSID.")?
      .get(1)
      .map(|s| s.as_str().to_string());

    let signal_str = signal_match
      .captures(&output)
      .context("Failed to parse WiFi hotspot signal strength.")?
      .get(1)
      .map(|s| s.as_str());

    let signal = signal_str
      .and_then(|s| signal_strip.captures(s))
      .context("Failed to parse WiFi hotspot signal strength.")?
      .get(1)
      .and_then(|s| s.as_str().parse().ok());

    Ok(WifiHotstop {
      ssid,
      signal_strength: signal,
    })
  }
}
