use std::{
  fs::{self, File},
  path::{Path, PathBuf},
};

use flate2::{write::GzEncoder, Compression};
use reqwest::{multipart::Form, StatusCode};
use serde::Deserialize;

use crate::{
  cli::PublishArgs,
  config::{Config, WidgetPack},
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

  // Create the tarball of the widget pack.
  let pack = Config::read_widget_pack(&pack_config_path, None)?;
  let tarball_path = create_tarball(&pack)?;

  // Upload to marketplace.
  let response =
    upload_to_marketplace(&pack_config_path, &tarball_path, args).await?;

  // Clean up temporary tarball.
  if let Err(err) = fs::remove_file(&tarball_path) {
    eprintln!("Failed to clean up temporary tarball file: {}", err);
  }

  Ok(format!(
    "Widget pack '{}' with version {} successfully published!",
    response.created_id, args.version
  ))
}

/// Creates a tarball of the widget pack.
///
/// The tarball includes all files in the widget directories that match the
/// include pattern.
fn create_tarball(pack: &WidgetPack) -> anyhow::Result<PathBuf> {
  // Create temporary .tar.gz file.
  let tarball_path = std::env::temp_dir().join("zebar-pack.tar.gz");
  let tarball_file = File::create(&tarball_path)?;

  let mut included_files = Vec::new();
  included_files.push(pack.config_path.clone());

  for widget in &pack.config.widgets {
    for entry in glob::glob(&widget.include_files)? {
      let entry = entry?;
      included_files.push(entry);
    }
  }

  let encoder = GzEncoder::new(tarball_file, Compression::default());
  let mut builder = tar::Builder::new(encoder);

  println!("Files to be included in the widget pack:");

  // Add files to the tarball, printing them as they are added.
  for entry in &included_files {
    println!("  {}", entry.strip_prefix(&pack.directory_path)?.display());
    builder.append_path(entry)?;
  }

  builder.finish()?;

  Ok(tarball_path)
}

/// Uploads the widget pack to the Zebar marketplace.
async fn upload_to_marketplace(
  pack_config_path: &Path,
  tarball_path: &Path,
  args: &PublishArgs,
) -> anyhow::Result<UploadResponse> {
  // Create multipart form.
  let mut form = Form::new()
    .text("version", args.version.to_string())
    .file("packConfig", pack_config_path)
    .await?
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
