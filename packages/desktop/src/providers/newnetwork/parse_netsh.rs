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
  pub signal: Option<u32>,
  pub connected: bool,
}

pub fn get_primary_interface_ssid_and_strength(
) -> anyhow::Result<NetshInfo> {
  let ssid_match = Regex::new(r"(?m)^\s*SSID\s*:\s*(.*?)\r?$").unwrap();

  let signal_match =
    Regex::new(r"(?m)^\s*Signal\s*:\s*(.*?)\r?$").unwrap();

  let signal_strip = Regex::new(r"(\d+)%").unwrap();

  let connected_match =
    Regex::new(r"(?m)^\s*State\s*:\s*(.*?)\r?$").unwrap();

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
  let signal_str: Option<&str> = signal_match
    .captures(&output)
    .map(|m| m.get(1).unwrap().as_str());
  let signal: Option<u32> = signal_str
    .and_then(|s| signal_strip.captures(s))
    .map(|m| m.get(1).unwrap().as_str().parse().unwrap());
  let connected: bool = connected_match
    .captures(&output)
    .map(|m| m.get(1).unwrap().as_str().to_string()) == Some("connected".to_string());
  
  return Ok(NetshInfo { ssid, signal, connected });
}
