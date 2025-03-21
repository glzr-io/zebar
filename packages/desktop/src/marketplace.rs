use std::{
  fs::{self},
  path::PathBuf,
  sync::Arc,
  time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{bail, Context};
use flate2::read::GzDecoder;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tar::Archive;
use tauri::{path::BaseDirectory, AppHandle, Manager};
use tokio::task;

use crate::{
  app_settings::AppSettings,
  config::Config,
  widget_factory::{WidgetFactory, WidgetOpenOptions},
};

/// Metadata about an installed marketplace widget pack.
///
/// These are stored in `%userprofile%/.glzr/zebar/.marketplace`.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketplacePackMetadata {
  /// Unique identifier for the pack.
  pub pack_id: String,

  /// Version of the installed pack.
  pub version: String,

  /// URL to the tarball.
  pub tarball_url: String,

  /// Path to the extracted pack directory.
  pub directory_path: String,

  /// Installation timestamp, stored as seconds since epoch.
  pub installed_at: u64,
}

/// Manages marketplace widget packs.
pub struct MarketplaceManager {
  /// Handle to the Tauri application.
  app_handle: AppHandle,

  /// Reference to `AppSettings`.
  app_settings: Arc<AppSettings>,

  /// Reference to `Config`.
  config: Arc<Config>,

  /// Reference to `WidgetFactory`.
  widget_factory: Arc<WidgetFactory>,
}

impl MarketplaceManager {
  /// Creates a new `MarketplaceManager` instance.
  pub fn new(
    app_handle: AppHandle,
    app_settings: Arc<AppSettings>,
    config: Arc<Config>,
    widget_factory: Arc<WidgetFactory>,
  ) -> Self {
    Self {
      app_handle,
      app_settings,
      config,
      widget_factory,
    }
  }

  /// Returns the path to the marketplace metadata directory.
  fn metadata_dir(&self) -> anyhow::Result<PathBuf> {
    let metadata_dir = self.app_settings.config_dir.join(".marketplace");

    if !metadata_dir.exists() {
      fs::create_dir_all(&metadata_dir)?;
    }

    Ok(metadata_dir)
  }

  /// Returns the path to the downloads directory.
  fn downloads_dir(&self) -> anyhow::Result<PathBuf> {
    let downloads_dir = self
      .app_handle
      .path()
      .resolve("downloads", BaseDirectory::AppData)
      .context("Unable to resolve app data directory.")?;

    if !downloads_dir.exists() {
      fs::create_dir_all(&downloads_dir)?;
    }

    Ok(downloads_dir)
  }

  /// Returns the path to the extracted pack directory.
  fn pack_dir(
    &self,
    pack_id: &str,
    version: &str,
    is_preview: bool,
  ) -> anyhow::Result<PathBuf> {
    let suffix = if is_preview { "-preview" } else { "" };
    let dir_name = format!("{}@{}{}", pack_id, version, suffix);

    Ok(self.downloads_dir()?.join(dir_name))
  }

  /// Returns the path to the metadata file for a pack.
  fn metadata_file(&self, pack_id: &str) -> anyhow::Result<PathBuf> {
    Ok(self.metadata_dir()?.join(format!("{}.json", pack_id)))
  }

  /// Downloads and extracts a widget pack.
  async fn download_and_extract(
    &self,
    pack_id: &str,
    version: &str,
    tarball_url: &str,
    is_preview: bool,
  ) -> anyhow::Result<PathBuf> {
    let pack_dir = self.pack_dir(pack_id, version, is_preview)?;

    // Skip download if the directory already exists.
    if pack_dir.exists() {
      tracing::info!(
        "Pack directory already exists: {}",
        pack_dir.display()
      );

      return Ok(pack_dir);
    }

    tracing::info!("Downloading widget pack from {}", tarball_url);

    // Download the tarball.
    let response = reqwest::get(tarball_url).await?;

    if response.status() != StatusCode::OK {
      bail!("Failed to download widget pack: HTTP {}", response.status());
    }

    let bytes = response.bytes().await?;

    // Create the pack directory.
    fs::create_dir_all(&pack_dir)?;

    tracing::info!("Extracting widget pack to {}", pack_dir.display());

    // Extract the tarball.
    let pack_dir = task::spawn_blocking({
      let pack_dir = pack_dir.clone();

      move || {
        let decoder = GzDecoder::new(&bytes[..]);
        let mut archive = Archive::new(decoder);

        archive.unpack(&pack_dir)?;

        anyhow::Ok(pack_dir)
      }
    })
    .await??;

    Ok(pack_dir)
  }

  /// Installs a widget pack from the marketplace.
  pub async fn install_widget_pack(
    &self,
    pack_id: &str,
    version: &str,
    tarball_url: &str,
  ) -> anyhow::Result<()> {
    // Download and extract the pack
    let pack_dir = self
      .download_and_extract(pack_id, version, tarball_url, false)
      .await?;

    // Create metadata
    let metadata = MarketplacePackMetadata {
      pack_id: pack_id.to_string(),
      version: version.to_string(),
      tarball_url: tarball_url.to_string(),
      directory_path: pack_dir.to_string_lossy().to_string(),
      installed_at: SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs(),
    };

    // Write metadata to file
    let metadata_file = self.metadata_file(pack_id)?;
    fs::write(
      &metadata_file,
      serde_json::to_string_pretty(&metadata)? + "\n",
    )?;

    tracing::info!("Installed widget pack: {}", pack_id);

    // Reload configs to include the new pack
    self.config.reload().await?;

    Ok(())
  }

  /// Previews a widget pack from the marketplace.
  pub async fn preview_widget_pack(
    &self,
    pack_id: &str,
    version: &str,
    tarball_url: &str,
  ) -> anyhow::Result<()> {
    // Stop any existing preview
    self.stop_preview_widget_pack(Some(pack_id)).await?;

    // Download and extract the pack
    let pack_dir = self
      .download_and_extract(pack_id, version, tarball_url, true)
      .await?;

    // Find the pack config file
    let pack_config_path = pack_dir.join("zpack.json");

    if !pack_config_path.exists() {
      bail!("Widget pack config not found in downloaded pack");
    }

    // Read the pack config
    let pack = Config::read_widget_pack(&pack_config_path)?;

    // Start all widgets in the pack
    for widget in &pack.config.widgets {
      // Use the first preset or skip if no presets
      if let Some(preset) = widget.presets.first() {
        self
          .widget_factory
          .start_widget(
            &pack.id,
            &widget.name,
            &WidgetOpenOptions::Preset(preset.name.clone()),
          )
          .await?;
      }
    }

    tracing::info!("Started preview for widget pack: {}", pack_id);

    Ok(())
  }

  /// Stops the preview of a widget pack.
  pub async fn stop_preview_widget_pack(
    &self,
    pack_id: Option<&str>,
  ) -> anyhow::Result<()> {
    if let Some(pack_id) = pack_id {
      // Get all widget states
      let widget_states = self.widget_factory.states().await;

      // Find widgets from the specified pack
      for state in widget_states.values() {
        if state.pack_id == pack_id {
          self.widget_factory.stop_by_id(&state.id)?;
        }
      }

      // Clean up the preview directory
      let downloads_dir = self.downloads_dir()?;

      if let Ok(entries) = fs::read_dir(&downloads_dir) {
        for entry in entries.filter_map(Result::ok) {
          let path = entry.path();

          if let Some(file_name) = path.file_name() {
            let file_name = file_name.to_string_lossy();

            if file_name.starts_with(&format!("{}@", pack_id))
              && file_name.ends_with("-preview")
            {
              if let Err(err) = fs::remove_dir_all(&path) {
                tracing::error!(
                  "Failed to remove preview directory: {}",
                  err
                );
              }
            }
          }
        }
      }

      tracing::info!("Stopped preview for widget pack: {}", pack_id);
    }

    Ok(())
  }
}
