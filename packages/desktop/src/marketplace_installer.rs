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
use tokio::{sync::mpsc, task};

use crate::{
  app_settings::AppSettings,
  common::read_and_parse_json,
  config::{Config, WidgetPack},
};

/// Metadata about an installed marketplace widget pack.
///
/// These are stored in `%userprofile%/.glzr/zebar/.marketplace`.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketplacePackMetadata {
  /// Unique identifier for the pack.
  pub pack_id: String,

  /// Version of the installed pack.
  pub version: String,

  /// Installation timestamp, stored as seconds since epoch.
  pub installed_at: u64,
}

impl MarketplacePackMetadata {
  pub fn new(pack_id: &str, version: &str) -> anyhow::Result<Self> {
    Ok(Self {
      pack_id: pack_id.to_string(),
      version: version.to_string(),
      installed_at: SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("Failed to get timestamp.")?
        .as_secs(),
    })
  }
}

/// Manages installation of marketplace widget packs.
#[derive(Debug)]
pub struct MarketplaceInstaller {
  /// Reference to `AppSettings`.
  app_settings: Arc<AppSettings>,

  /// Sender channel for newly installed widget packs.
  installed_tx: mpsc::Sender<WidgetPack>,
}

impl MarketplaceInstaller {
  /// Creates a new `MarketplaceInstaller` instance.
  pub fn new(
    app_settings: Arc<AppSettings>,
  ) -> (Arc<Self>, mpsc::Receiver<WidgetPack>) {
    let (installed_tx, installed_rx) = mpsc::channel(1);

    (
      Arc::new(Self {
        app_settings,
        installed_tx,
      }),
      installed_rx,
    )
  }

  /// Returns the path to the extracted pack directory.
  fn pack_download_dir(&self, pack_id: &str, version: &str) -> PathBuf {
    self
      .app_settings
      .marketplace_download_dir
      .join(format!("{}@{}", pack_id, version))
  }

  /// Returns the path to the metadata file for a pack.
  fn pack_metadata_file_path(&self, pack_id: &str) -> PathBuf {
    self
      .app_settings
      .marketplace_meta_dir
      .join(format!("{}.json", pack_id))
  }

  /// Returns a vector of `MarketplacePackMetadata` instances for all
  /// installed packs.
  pub fn installed_packs_metadata(
    &self,
  ) -> anyhow::Result<Vec<MarketplacePackMetadata>> {
    let packs_metadata =
      fs::read_dir(&self.app_settings.marketplace_meta_dir)?
        .filter_map(|entry| {
          let metadata = read_and_parse_json::<MarketplacePackMetadata>(
            &entry.ok()?.path(),
          )
          .ok()?;

          Some(metadata)
        })
        .collect();

    Ok(packs_metadata)
  }

  /// Installs a widget pack from the marketplace.
  pub async fn install(
    &self,
    pack_id: &str,
    version: &str,
    tarball_url: &str,
    is_preview: bool,
  ) -> anyhow::Result<WidgetPack> {
    let pack_dir = self.pack_download_dir(pack_id, version);

    // Download and extract the pack. Skip the download if the directory
    // already exists.
    if !pack_dir.exists() {
      self.download_and_extract(&pack_dir, tarball_url).await?;
    }

    // Create metadata.
    let metadata = MarketplacePackMetadata::new(pack_id, version)?;

    let pack = Config::read_widget_pack(
      &pack_dir.join("zpack.json"),
      Some(&metadata),
    )?;

    if !is_preview {
      fs::create_dir_all(&self.app_settings.marketplace_meta_dir)?;

      // Write metadata to file.
      fs::write(
        self.pack_metadata_file_path(pack_id),
        serde_json::to_string_pretty(&metadata)? + "\n",
      )?;

      // Broadcast the installation event.
      self.installed_tx.send(pack.clone()).await?;
    }

    tracing::info!("Installed widget pack: {}", pack_id);

    Ok(pack)
  }

  /// Deletes the metadata for an installed widget pack.
  pub fn delete_metadata(&self, pack_id: &str) -> anyhow::Result<()> {
    let metadata_path = self.pack_metadata_file_path(pack_id);

    if metadata_path.exists() {
      fs::remove_file(metadata_path)?;
    }

    Ok(())
  }

  /// Downloads and extracts a widget pack.
  async fn download_and_extract(
    &self,
    pack_dir: &PathBuf,
    tarball_url: &str,
  ) -> anyhow::Result<()> {
    tracing::info!("Downloading widget pack from {}.", tarball_url);

    // Download the tarball.
    let response = reqwest::get(tarball_url).await?;

    if response.status() != StatusCode::OK {
      bail!("Failed to download widget pack: HTTP {}", response.status());
    }

    let bytes = response.bytes().await?;

    // Create the pack directory.
    fs::create_dir_all(pack_dir)?;
    tracing::info!("Extracting widget pack to {}", pack_dir.display());

    // Extract the tarball.
    task::spawn_blocking({
      let pack_dir = pack_dir.clone();

      move || {
        let decoder = GzDecoder::new(&bytes[..]);
        let mut archive = Archive::new(decoder);
        archive.unpack(&pack_dir)
      }
    })
    .await??;

    Ok(())
  }
}
