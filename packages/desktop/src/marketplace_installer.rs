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
use tokio::{sync::mpsc, task};

use crate::app_settings::AppSettings;

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

/// Manages installation of marketplace widget packs.
pub struct MarketplaceInstaller {
  /// Handle to the Tauri application.
  app_handle: AppHandle,

  /// Reference to `AppSettings`.
  app_settings: Arc<AppSettings>,

  /// Sender channel for marketplace events.
  installed_tx: mpsc::Sender<MarketplacePackMetadata>,
}

impl MarketplaceInstaller {
  /// Creates a new `MarketplaceInstaller` instance.
  pub fn new(
    app_handle: &AppHandle,
    app_settings: Arc<AppSettings>,
  ) -> (Arc<Self>, mpsc::Receiver<MarketplacePackMetadata>) {
    let (installed_tx, installed_rx) = mpsc::channel(1);

    (
      Arc::new(Self {
        app_handle: app_handle.clone(),
        app_settings,
        installed_tx,
      }),
      installed_rx,
    )
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
  ) -> anyhow::Result<PathBuf> {
    Ok(
      self
        .downloads_dir()?
        .join(format!("{}@{}", pack_id, version)),
    )
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
  ) -> anyhow::Result<PathBuf> {
    let pack_dir = self.pack_dir(pack_id, version)?;

    // Skip download if the directory already exists.
    if pack_dir.exists() {
      tracing::info!(
        "Pack directory already exists: {}",
        pack_dir.display()
      );

      return Ok(pack_dir);
    }

    tracing::info!("Downloading widget pack from {}.", tarball_url);

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
    let pack_dir = task::spawn_blocking(move || {
      let decoder = GzDecoder::new(&bytes[..]);
      let mut archive = Archive::new(decoder);

      archive.unpack(&pack_dir)?;
      anyhow::Ok(pack_dir)
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
    is_preview: bool,
  ) -> anyhow::Result<()> {
    // Download and extract the pack.
    let pack_dir = self
      .download_and_extract(pack_id, version, tarball_url)
      .await?;

    // Create metadata.
    let metadata = MarketplacePackMetadata {
      pack_id: pack_id.to_string(),
      version: version.to_string(),
      tarball_url: tarball_url.to_string(),
      directory_path: pack_dir.to_string_lossy().to_string(),
      installed_at: SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("Failed to get timestamp.")?
        .as_secs(),
    };

    // Write metadata to file.
    if !is_preview {
      fs::write(
        &self.metadata_file(pack_id)?,
        serde_json::to_string_pretty(&metadata)? + "\n",
      )?;
    }

    tracing::info!("Installed widget pack: {}", pack_id);

    // Broadcast the installation event.
    self.installed_tx.send(metadata).await?;

    Ok(())
  }
}
