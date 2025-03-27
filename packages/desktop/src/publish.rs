use std::{
  fs::{self, File},
  path::{Path, PathBuf},
};

use flate2::{write::GzEncoder, Compression};
use ignore::{overrides::OverrideBuilder, WalkBuilder};
use reqwest::{multipart::Form, StatusCode};
use serde::Deserialize;

use crate::{
  cli::PublishArgs,
  common::visit_deep,
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

  let mut included_files = vec![pack.config_path.clone()];

  let glob_set = {
    let mut builder = WalkBuilder::new(pack.directory_path.clone());

    builder
      .standard_filters(false)
      .hidden(true)
      .overrides(OverrideBuilder::new("!**").build()?);

    for widget in &pack.config.widgets {
      for pattern in widget.include_files.split('\n') {
        // builder.add(Glob::new(pattern)?);
        builder.overrides(OverrideBuilder::new(pattern).build()?);
      }
    }

    builder.build()
  };

  for entry in glob_set {
    match entry {
      Ok(entry) => {
        let path = entry.path();
        if path.is_file() {
          included_files.push(path.to_path_buf());
        }
      }
      Err(err) => println!("ERROR: {}", err),
    }

    // let path = entry?.path();
    // // let relative_path =
    // // path.strip_prefix(&pack.directory_path).unwrap();
    // if path.is_file() {
    //   included_files.push(path.to_path_buf());
    // }
  }

  // visit_deep(&pack.directory_path, &mut |entry| {
  //   let path = entry.path();
  //   let relative_path =
  // path.strip_prefix(&pack.directory_path).unwrap();   // TODO: If the
  // path is a directory, we need to add all files in the   // directory
  // to the tarball.   if entry.path().is_file() &&
  // glob_set.is_match(relative_path) {     included_files.push(path.
  // to_path_buf());   }
  // })?;

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
