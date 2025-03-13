use std::{
  fs::{self, File},
  io::Read,
  path::{Path, PathBuf},
};

use anyhow::Context;
use flate2::{write::GzEncoder, Compression};
use ignore::WalkBuilder;
use reqwest::{multipart::Form, StatusCode};
use serde::Deserialize;
use tar::Builder;

use crate::{
  cli::PublishArgs,
  common::read_and_parse_json,
  config::{Config, WidgetPackConfig},
};

#[derive(Debug, Deserialize)]
struct UploadResponse {
  created_id: String,
}

/// Publishes a widget pack to the Zebar marketplace.
pub async fn publish_widget_pack(
  args: &PublishArgs,
) -> anyhow::Result<String> {
  let pack_config_path = if args.pack_config.is_dir() {
    args.pack_config.join("zpack.json")
  } else {
    args.pack_config.to_path_buf()
  };

  if !pack_config_path.exists() {
    anyhow::bail!(
      "Widget pack config not found at '{}'.",
      pack_config_path.display()
    );
  }

  let pack_config = Config::read_widget_pack(&pack_config_path)?;

  // Create the tarball of the widget pack.
  let tarball_path = create_tarball(
    &pack_config.directory_path,
    &pack_config.config.exclude_files,
  )?;

  // Upload to marketplace.
  let response =
    upload_to_marketplace(&pack_config.config.name, args, &tarball_path)
      .await?;

  // Clean up temporary tarball.
  if let Err(err) = fs::remove_file(&tarball_path) {
    eprintln!("Failed to clean up temporary tarball file: {}", err);
  }

  Ok(format!(
    "Widget pack '{}' with version {} successfully published!",
    response.created_id, args.version
  ))
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
  name: &str,
  args: &PublishArgs,
  tarball_path: &Path,
) -> anyhow::Result<UploadResponse> {
  // Create multipart form.
  let mut form = Form::new()
    .text("name", name.to_string())
    .text("version", args.version.to_string())
    .file("tarball", tarball_path)
    .await?;

  if let Some(commit_sha) = &args.commit_sha {
    form = form.text("commitSha", commit_sha.to_string());
  }

  if let Some(release_notes) = &args.release_notes {
    form = form.text("releaseNotes", release_notes.to_string());
  }

  if let Some(release_url) = &args.release_url {
    form = form.text("releaseUrl", release_url.to_string());
  }

  // Construct the full API endpoint URL.
  let upload_url = format!(
    "{}/api/trpc/widgetPack.publishVersion",
    args.api_url.trim_end_matches('/')
  );

  // Send the request.
  let client = reqwest::Client::new();
  let response = client
    .post(upload_url)
    .header("X-API-TOKEN", args.token.clone())
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
      anyhow::bail!(
        "Failed to upload widget pack: {} - {}",
        status,
        error_text
      )
    }
  }
}
