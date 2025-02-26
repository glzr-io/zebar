use std::{
  fs::{self, File},
  io::{self, Read, Write},
  path::{Path, PathBuf},
};

use anyhow::{bail, Context};
use flate2::{write::GzEncoder, Compression};
use ignore::{DirEntry, WalkBuilder};
use reqwest::{multipart::Form, StatusCode};
use serde::{Deserialize, Serialize};
use tar::Builder;
use tracing::{info, warn};

use crate::{
  cli::PublishArgs, common::read_and_parse_json, config::WidgetPackConfig,
};

#[derive(Debug, Deserialize)]
struct UploadResponse {
  widget_pack_id: String,
  message: String,
}

/// Publishes a widget pack to the Zebar marketplace.
pub async fn publish_widget_pack(
  args: &PublishArgs,
) -> anyhow::Result<String> {
  // Read and parse the widget pack config.
  let pack_config_path = &args.pack_config;
  if !pack_config_path.exists() {
    bail!(
      "Widget pack config not found at '{}'.",
      pack_config_path.display()
    );
  }

  let pack_config: WidgetPackConfig =
    read_and_parse_json(pack_config_path)?;
  let pack_dir = pack_config_path
    .parent()
    .context("Failed to get parent directory of pack config")?;

  // Create tarball
  let tarball_path = create_tarball(pack_dir, &pack_config.exclude_files)?;

  // Upload to marketplace
  let response = upload_to_marketplace(
    &args.token,
    &args.version,
    args.commit_sha.as_deref(),
    args.release_notes.as_deref(),
    args.release_url.as_deref(),
    pack_config_path,
    &tarball_path,
  )
  .await?;

  // Clean up temporary tarball
  if let Err(e) = fs::remove_file(&tarball_path) {
    warn!("Failed to remove temporary tarball: {}", e);
  }

  Ok(response.message)
}

/// Creates a tarball of the widget pack, excluding files based on the
/// exclude pattern.
fn create_tarball(
  pack_dir: &Path,
  exclude_pattern: &str,
) -> anyhow::Result<PathBuf> {
  let temp_dir = std::env::temp_dir();
  let tarball_path = temp_dir.join("zebar-pack.tar.gz");

  // Create tar.gz file
  let file = File::create(&tarball_path)?;
  let encoder = GzEncoder::new(file, Compression::default());
  let mut builder = Builder::new(encoder);

  // Build a list of files to include
  let mut included_files = Vec::new();
  let walker = WalkBuilder::new(pack_dir)
    .hidden(false) // Include hidden files
    .standard_filters(false) // Don't use standard ignore filters
    .add_custom_ignore_filename(".zebarignore") // Optional: support custom ignore file
    .build();

  // Parse the exclude pattern into a gitignore-style matcher
  let matcher = ignore::gitignore::GitignoreBuilder::new(pack_dir)
    .add_line(None, exclude_pattern)?
    .build()?;

  for entry in walker {
    let entry = entry?;
    let path = entry.path();

    // Skip the directory itself
    if path == pack_dir {
      continue;
    }

    // Check if the file should be excluded
    let relative_path = path.strip_prefix(pack_dir)?;
    if !matcher
      .matched(relative_path, entry.file_type().unwrap().is_dir())
      .is_ignore()
    {
      included_files.push(entry);
    }
  }

  // Print the list of included files
  println!("Files to be included in the widget pack:");
  for entry in &included_files {
    println!("  {}", entry.path().strip_prefix(pack_dir)?.display());
  }

  // Add files to the tarball
  for entry in included_files {
    let path = entry.path();
    let relative_path = path.strip_prefix(pack_dir)?;

    if path.is_file() {
      let mut file = File::open(path)?;
      let mut contents = Vec::new();
      file.read_to_end(&mut contents)?;

      let mut header = tar::Header::new_gnu();
      header.set_size(contents.len() as u64);
      header.set_mode(0o644);
      header.set_mtime(
        entry
          .metadata()?
          .modified()?
          .duration_since(std::time::UNIX_EPOCH)?
          .as_secs(),
      );

      builder.append_data(&mut header, relative_path, &contents[..])?;
    } else if path.is_dir() {
      builder.append_dir(relative_path, path)?;
    }
  }

  builder.finish()?;

  Ok(tarball_path)
}

/// Uploads the widget pack to the Zebar marketplace.
async fn upload_to_marketplace(
  token: &str,
  version: &str,
  commit_sha: Option<&str>,
  release_notes: Option<&str>,
  release_url: Option<&str>,
  pack_config_path: &Path,
  tarball_path: &Path,
) -> anyhow::Result<UploadResponse> {
  // Read the pack config file
  let config_content = fs::read_to_string(pack_config_path)?;

  // Create multipart form
  let mut form = Form::new()
    .text("version", version.to_string())
    .text("config", config_content)
    .file("tarball", tarball_path)
    .await?;

  // Add optional fields
  if let Some(commit_sha) = commit_sha {
    form = form.text("commitSha", commit_sha.to_string());
  }

  if let Some(release_notes) = release_notes {
    form = form.text("releaseNotes", release_notes.to_string());
  }

  if let Some(release_url) = release_url {
    form = form.text("releaseUrl", release_url.to_string());
  }

  // Send the request.
  let client = reqwest::Client::new();
  let response = client
    .post("https://api.glzr.io/marketplace/widget-packs")
    .header("Authorization", format!("Bearer {}", token))
    .multipart(form)
    .send()
    .await?;

  match response.status() {
    StatusCode::OK | StatusCode::CREATED => {
      let upload_response = response.json::<UploadResponse>().await?;
      Ok(upload_response)
    }
    status => {
      let error_text = response.text().await?;
      bail!("Failed to upload widget pack: {} - {}", status, error_text)
    }
  }
}
