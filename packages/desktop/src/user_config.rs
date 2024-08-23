use std::{
  fs::{self, DirEntry},
  path::PathBuf,
};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use tauri::{path::BaseDirectory, AppHandle, Manager};

use crate::util::LengthValue;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowConfig {
  /// Entry point HTML file.
  html_path: String,

  /// Default options for when the window is opened.
  launch_options: WindowLaunchOptions,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowLaunchOptions {
  /// Whether to show the window above/below all others.
  z_order: WindowZOrder,

  /// Whether the window should be shown in the taskbar.
  shown_in_taskbar: bool,

  /// Whether the window should have resize handles.
  resizable: bool,

  /// Whether the window frame should be transparent.
  transparent: bool,

  /// Where to place the window.
  placement: Vec<WindowPlacement>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowZOrder {
  AlwaysOnBottom,
  AlwaysOnTop,
  Normal,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowPlacement {
  /// The monitor index to place the window on.
  monitor: u32,

  /// Anchor-point of the window.
  anchor: WindowAnchor,

  /// Offset from the anchor-point.
  offset_x: LengthValue,

  /// Offset from the anchor-point.
  offset_y: LengthValue,

  /// Width of the window in % or physical pixels.
  width: LengthValue,

  /// Height of the window in % or physical pixels.
  height: LengthValue,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowAnchor {
  TopLeft,
  TopCenter,
  TopRight,
  CenterLeft,
  Center,
  CenterCenter,
  CenterRight,
  BottomLeft,
  BottomCenter,
  BottomRight,
}

#[derive(Debug)]
pub struct Config {
  /// Directory where config files are stored.
  pub config_dir: PathBuf,

  /// Config entries.
  pub config_entries: Vec<ConfigEntry>,
}

#[derive(Debug)]
pub struct ConfigEntry {
  path: PathBuf,
  config: WindowConfig,
}

impl Config {
  /// Creates a new `Config` instance.
  pub fn new(app_handle: &AppHandle) -> anyhow::Result<Self> {
    let config_dir = app_handle
      .path()
      .resolve(".glzr/zebar", BaseDirectory::Home)
      .context("Unable to get home directory.")?;

    Ok(Self {
      config_dir,
      config_entries: Vec::new(),
    })
  }

  /// Reads the config files within the config directory.
  pub fn read(&mut self) -> anyhow::Result<()> {
    self.config_entries = Self::aggregate_configs(&self.config_dir)?;
    Ok(())
  }

  fn aggregate_configs(dir: &PathBuf) -> anyhow::Result<Vec<ConfigEntry>> {
    let mut configs = Vec::new();

    for entry in fs::read_dir(dir).with_context(|| {
      format!("Failed to read directory: {}", dir.display())
    })? {
      let entry = entry?;
      let path = entry.path();

      if path.is_dir() {
        // Recursively call aggregate_configs on subdirectories
        configs.extend(Self::aggregate_configs(&path)?);
      } else if path.extension().and_then(|ext| ext.to_str())
        == Some("json")
      {
        if let Ok(config) = Self::process_file(&path) {
          configs.push(ConfigEntry {
            config,
            path: path.clone(),
          });
        }
      }
    }

    Ok(configs)
  }

  fn is_json_file(entry: &DirEntry) -> anyhow::Result<bool> {
    let file_type = entry.file_type()?;
    Ok(
      file_type.is_file()
        && entry.path().extension().and_then(|ext| ext.to_str())
          == Some("json"),
    )
  }

  // fn traverse_directory(
  //   dir: &PathBuf,
  //   config_entries: &mut Vec<ConfigEntry>,
  // ) -> anyhow::Result<()> {
  //   for entry in fs::read_dir(dir)? {
  //     let entry = entry?;
  //     let path = entry.path();
  //     if path.is_dir() {
  //       Self::traverse_directory(&path, config_entries)?;
  //     } else if Self::is_json_file(&entry)? {
  //       if let Ok(config_entry) = Self::process_file(entry) {
  //         config_entries.push(config_entry);
  //       }
  //     }
  //   }
  //   Ok(())
  // }

  fn process_file(entry: &PathBuf) -> anyhow::Result<WindowConfig> {
    let content = fs::read_to_string(entry)?;
    let config = serde_json::from_str(&content)?;
    Ok(config)
  }

  /// Initialize config at the given path from the sample config resource.
  fn create_from_sample(
    config_path: &PathBuf,
    app_handle: &AppHandle,
  ) -> anyhow::Result<()> {
    let sample_path = app_handle
      .path()
      .resolve("resources/sample-config.yaml", BaseDirectory::Resource)
      .context("Unable to resolve sample config resource.")?;

    let sample_script = app_handle
      .path()
      .resolve("resources/script.js", BaseDirectory::Resource)
      .context("Unable to resolve sample script resource.")?;

    let dest_dir =
      config_path.parent().context("Invalid config directory.")?;

    // Create the destination directory.
    std::fs::create_dir_all(&dest_dir).with_context(|| {
      format!("Unable to create directory {}.", &config_path.display())
    })?;

    // Copy over sample config.
    let config_path = dest_dir.join("config.yaml");
    fs::copy(&sample_path, &config_path).with_context(|| {
      format!("Unable to write to {}.", config_path.display())
    })?;

    // Copy over sample script.
    let script_path = dest_dir.join("script.js");
    fs::copy(&sample_script, &script_path).with_context(|| {
      format!("Unable to write to {}.", script_path.display())
    })?;

    Ok(())
  }

  pub fn open_config_dir(app_handle: &AppHandle) -> anyhow::Result<()> {
    let dir_path = app_handle
      .path()
      .resolve(".glzr/zebar", BaseDirectory::Home)
      .context("Unable to get home directory.")?
      .canonicalize()?;

    #[cfg(target_os = "windows")]
    {
      std::process::Command::new("explorer")
        .arg(dir_path)
        .spawn()?;
    }

    #[cfg(target_os = "macos")]
    {
      std::process::Command::new("open")
        .arg(dir_path)
        .arg("-R")
        .spawn()?;
    }

    #[cfg(target_os = "linux")]
    {
      std::process::Command::new("xdg-open")
        .arg(dir_path)
        .spawn()?;
    }

    Ok(())
  }
}
