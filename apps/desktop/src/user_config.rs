use std::{
  fs,
  io::{Read, Write},
  path::PathBuf,
};

use anyhow::{Context, Result};
use tauri::{api::path::home_dir, AppHandle};

/// Reads the config file at `/.glazer/zanbar.yaml` in the user's home
/// directory.
pub fn read_file(app_handle: AppHandle) -> Result<String> {
  let config_path = home_dir()
    .context("Unable to get home directory.")?
    .join(".glazer")
    .join("zanbar.yaml");

  // Create new config file from sample if it doesn't exist.
  if !config_path.exists() {
    create_from_sample(app_handle, &config_path)?;
  }

  let config_string = fs::read(&config_path)
    .context("Unable to read config file.")?
    .iter()
    .map(|&c| c as char)
    .collect::<String>();

  Ok(config_string)
}

fn create_from_sample(
  app_handle: AppHandle,
  config_path: &PathBuf,
) -> Result<()> {
  let sample_path = app_handle
    .path_resolver()
    .resolve_resource("sample-config.yaml")
    .context("Failed to resolve sample config.")?;

  // let resource_path = app_handle
  //   .path_resolver()
  //   .resolve_resource("sample-config.yaml")
  //   .expect("Failed to resolve resource.");

  println!("resource_path: {:?}", sample_path);
  let mut file =
    fs::File::open(&sample_path).context("Unable to read sample config.")?;

  // Read the contents of the source file
  let mut contents = Vec::new();
  file.read_to_end(&mut contents)?;

  // let reader = std::io::BufReader::new(file);

  // let destination_path = "path/to/destination/file.txt";

  // // Open the source file
  // // let mut source_file = fs::File::open(&resource_path)?;

  // // Create the destination file
  // let mut destination_file =
  //   fs::File::create(&destination_path).map_err(|err| err.to_string())?;

  // destination_file.
  // .map_err(|err| err.to_string())?;

  let parent_dir = config_path.parent().context("Invalid config directory.")?;

  std::fs::create_dir_all(&parent_dir).with_context(|| {
    format!("Unable to create directory {}.", &config_path.display())
  })?;

  // Create the destination file
  let mut config_file = fs::File::create(&config_path)?;
  config_file.write_all(&contents)?;

  // fs::copy(&sample_path, &config_path).with_context(|| {
  //   format!(
  //     "Unable to copy file from {} to {}.",
  //     &sample_path.display(),
  //     &config_path.display()
  //   )
  // })?;

  Ok(())
}
