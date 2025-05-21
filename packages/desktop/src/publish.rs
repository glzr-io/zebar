use std::{
  collections::HashSet,
  fs::{self, File},
  path::{Path, PathBuf},
};

use flate2::{write::GzEncoder, Compression};
use reqwest::multipart::Form;
use serde::Deserialize;

use crate::{
  cli::PublishArgs,
  common::{glob_util, PathExt},
  widget_pack::{WidgetPack, WidgetPackManager},
};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum UploadResponse {
  Error { error: UploadResponseError },
  Success { result: UploadResponseSuccess },
}

#[derive(Debug, Deserialize)]
struct UploadResponseError {
  json: UploadResponseErrorJson,
}

#[derive(Debug, Deserialize)]
struct UploadResponseErrorJson {
  message: String,
}

#[derive(Debug, Deserialize)]
struct UploadResponseSuccess {
  data: UploadResponseSuccessData,
}

#[derive(Debug, Deserialize)]
struct UploadResponseSuccessData {
  json: UploadResponseSuccessJson,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UploadResponseSuccessJson {
  widget_pack: UploadResponseSuccessWidgetPack,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UploadResponseSuccessWidgetPack {
  published_id: String,
  latest_version: String,
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
  let pack = WidgetPackManager::read_widget_pack(&pack_config_path, None)?;
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
    response.published_id, response.latest_version
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

  let mut included_files = HashSet::new();
  included_files.insert(pack.config_path.clone());

  // Include preview images.
  for preview_image in &pack.config.preview_images {
    included_files.insert(
      PathBuf::from(preview_image).to_absolute(&pack.directory_path)?,
    );
  }

  let target_files = [
    "readme.md",
    "readme",
    "license.md",
    "license.txt",
    "license",
  ];

  // Include readme and license files if they exist.
  for entry in
    std::fs::read_dir(&pack.directory_path)?.filter_map(Result::ok)
  {
    let path = entry.path();

    if path.is_file() {
      if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
        // Check case-insensitive match against our target files.
        if target_files.contains(&file_name.to_lowercase().as_str()) {
          included_files.insert(path.clone());
        }
      }
    }
  }

  // Include all files that match the widgets' include patterns.
  included_files.extend(glob_util::matched_paths(
    &pack.directory_path,
    &pack.include_files(),
  )?);

  let encoder = GzEncoder::new(tarball_file, Compression::default());
  let mut builder = tar::Builder::new(encoder);

  println!("Files to be included in the widget pack:");

  // Add files to the tarball, printing them as they are added.
  for entry in &included_files {
    let relative_path = entry.strip_prefix(&pack.directory_path)?;
    println!("  {}", relative_path.display());
    builder.append_file(relative_path, &mut File::open(entry)?)?;
  }

  builder.finish()?;

  Ok(tarball_path)
}

/// Uploads the widget pack to the Zebar marketplace.
async fn upload_to_marketplace(
  pack_config_path: &Path,
  tarball_path: &Path,
  args: &PublishArgs,
) -> anyhow::Result<UploadResponseSuccessWidgetPack> {
  println!("Uploading widget pack to marketplace...");

  // Create multipart form.
  let mut form = Form::new()
    .file("packConfig", pack_config_path)
    .await?
    .file("tarball", tarball_path)
    .await?;

  if let Some(version_override) = &args.version_override {
    form = form.text("versionOverride", version_override.to_string());
  }

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
    "{}/v1/trpc/widgetPack.publishVersion",
    args.api_url.trim_end_matches('/')
  );

  // Send the request.
  let client = reqwest::Client::new();
  let response = client
    .post(upload_url)
    .header("X-API-TOKEN", args.token.clone())
    .multipart(form)
    .send()
    .await?
    .json::<UploadResponse>()
    .await?;

  match response {
    UploadResponse::Success { result } => Ok(result.data.json.widget_pack),
    UploadResponse::Error { error } => {
      anyhow::bail!(
        "Failed to upload widget pack: {}",
        error.json.message
      );
    }
  }
}
