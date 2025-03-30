use std::{
  fs::{self},
  path::{Path, PathBuf},
  sync::Arc,
};

use anyhow::Context;
use serde::{Deserialize, Deserializer, Serialize};
use tauri::{path::BaseDirectory, AppHandle, Manager};
use tokio::sync::{broadcast, Mutex};

use crate::common::{
  copy_dir_all, read_and_parse_json, visit_deep, PathExt,
};

pub const VERSION_NUMBER: &str = env!("VERSION_NUMBER");

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettingsValue {
  /// JSON schema URL to validate the settings file.
  #[serde(rename = "$schema")]
  schema: Option<String>,

  /// Widget configs to be launched on startup.
  pub startup_configs: Vec<StartupConfig>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartupConfig {
  /// ID of the widget pack to launch on startup.
  pub pack: String,

  /// Name of the widget within the widget pack to launch on startup.
  pub widget: String,

  /// Preset name within the widget config.
  pub preset: String,
}

// Deserializer for `StartupConfig` that handles the different formats
// from v2.3.0 to v3.0.0.
impl<'de> Deserialize<'de> for StartupConfig {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrObject {
      // String format from v2.3.0 and earlier.
      String(String),
      // Object format from v2.7.0 and earlier.
      V2Object {
        path: PathBuf,
        preset: String,
      },
      // Object format from v3.0.0 onwards.
      V3Object {
        pack: String,
        widget: String,
        preset: String,
      },
    }

    fn parse_path(path: &Path) -> (String, String) {
      let path = path.to_string_lossy();
      let path = path
        .trim_start_matches('.')
        .trim_start_matches('/')
        .trim_start_matches('\\')
        .trim_end_matches(".zebar.json");

      // TODO: Transform pack ID if necessary. It might include special
      // symbols or spaces.
      path.split_once(['/', '\\']).map_or(
        (path.to_string(), String::new()),
        |(pack_dir, widget_name)| {
          (pack_dir.to_string(), widget_name.to_string())
        },
      )
    }

    let value = StringOrObject::deserialize(deserializer)?;

    Ok(match value {
      StringOrObject::String(s) => {
        let (pack, widget) = parse_path(&PathBuf::from(s));

        StartupConfig {
          pack,
          widget,
          preset: "default".to_string(),
        }
      }
      StringOrObject::V2Object { path, preset } => {
        let (pack, widget) = parse_path(&path);

        StartupConfig {
          pack,
          widget,
          preset,
        }
      }
      StringOrObject::V3Object {
        pack,
        widget,
        preset,
      } => StartupConfig {
        pack,
        widget,
        preset,
      },
    })
  }
}

#[derive(Debug)]
pub struct AppSettings {
  /// Handle to the Tauri application.
  app_handle: AppHandle,

  /// Indicates if the settings file was created during initialization.
  pub is_first_run: bool,

  /// Directory where config files are stored.
  pub config_dir: PathBuf,

  /// Directory where webview cache files are stored.
  pub webview_cache_dir: PathBuf,

  /// Directory where marketplace metadata files are stored.
  pub marketplace_meta_dir: PathBuf,

  /// Directory where downloaded marketplace widget packs are stored.
  pub marketplace_download_dir: PathBuf,

  /// Parsed app settings value.
  pub value: Arc<Mutex<AppSettingsValue>>,

  _settings_change_rx: broadcast::Receiver<AppSettingsValue>,

  pub settings_change_tx: broadcast::Sender<AppSettingsValue>,
}

impl AppSettings {
  /// Creates a new `AppSettings` instance.
  pub fn new(
    app_handle: &AppHandle,
    config_dir_override: Option<PathBuf>,
  ) -> anyhow::Result<Self> {
    let config_dir = match config_dir_override {
      Some(dir) => dir,
      None => app_handle
        .path()
        .resolve(".glzr/zebar", BaseDirectory::Home)
        .context("Unable to get home directory.")?,
    };

    let webview_cache_dir = app_handle
      .path()
      .resolve("zebar/webview-cache", BaseDirectory::Data)
      .context("Unable to resolve app data directory.")?;

    let marketplace_meta_dir = config_dir.join(".marketplace");

    let marketplace_download_dir = app_handle
      .path()
      .resolve("zebar/downloads", BaseDirectory::Data)
      .context("Unable to resolve app data directory.")?;

    for dir in [
      &config_dir,
      &webview_cache_dir,
      &marketplace_meta_dir,
      &marketplace_download_dir,
    ] {
      fs::create_dir_all(dir)?;
    }

    let (settings, is_first_run) =
      Self::read_settings_or_init(&config_dir)?;

    let (settings_change_tx, _settings_change_rx) = broadcast::channel(16);

    Ok(Self {
      app_handle: app_handle.clone(),
      is_first_run,
      config_dir: config_dir.canonicalize_pretty()?,
      webview_cache_dir: webview_cache_dir.canonicalize_pretty()?,
      marketplace_meta_dir: marketplace_meta_dir.canonicalize_pretty()?,
      marketplace_download_dir: marketplace_download_dir
        .canonicalize_pretty()?,
      value: Arc::new(Mutex::new(settings)),
      _settings_change_rx,
      settings_change_tx,
    })
  }

  /// Re-evaluates app settings and broadcasts the change.
  pub async fn reload(&self) -> anyhow::Result<()> {
    let (new_settings, _) = Self::read_settings_or_init(&self.config_dir)?;

    {
      let mut settings = self.value.lock().await;
      *settings = new_settings.clone();
    }

    self.settings_change_tx.send(new_settings)?;

    Ok(())
  }

  /// Reads the app settings file or initializes it with the template.
  ///
  /// Returns the parsed `AppSettingsValue` and a boolean indicating if
  /// the settings file was created.
  fn read_settings_or_init(
    dir: &Path,
  ) -> anyhow::Result<(AppSettingsValue, bool)> {
    let settings = Self::read_settings(dir)?;

    match settings {
      Some(settings) => Ok((settings, false)),
      None => {
        Self::create_default(dir)?;

        Ok((
          Self::read_settings(dir)?
            .context("Failed to create settings config.")?,
          true,
        ))
      }
    }
  }

  /// Reads the app settings file.
  ///
  /// Returns the parsed `AppSettingsValue` if found.
  fn read_settings(
    dir: &Path,
  ) -> anyhow::Result<Option<AppSettingsValue>> {
    let settings_path = dir.join("settings.json");

    match settings_path.exists() {
      false => Ok(None),
      true => read_and_parse_json(&settings_path),
    }
  }

  /// Writes to the app settings file.
  async fn write_settings(
    &self,
    new_settings: AppSettingsValue,
  ) -> anyhow::Result<()> {
    let settings_path = self.config_dir.join("settings.json");

    fs::write(
      &settings_path,
      serde_json::to_string_pretty(&new_settings)? + "\n",
    )?;

    let mut settings = self.value.lock().await;
    *settings = new_settings.clone();

    self.settings_change_tx.send(new_settings)?;

    Ok(())
  }

  /// Initializes app settings to the given path.
  ///
  /// `settings.json` is initialized with either `vanilla` or
  /// `with-glazewm` from the `glzr-io/starter` widget pack as
  /// startup config.
  fn create_default(config_dir: &Path) -> anyhow::Result<()> {
    tracing::info!("Initializing app settings from default.",);

    let default_settings = AppSettingsValue {
      schema: Some(format!(
        "https://github.com/glzr-io/zebar/raw/v{}/resources/settings-schema.json",
        VERSION_NUMBER
      )),
      startup_configs: vec![StartupConfig {
        pack: "glzr-io.starter".into(),
        widget: match is_app_installed("glazewm") {
          true => "with-glazewm".into(),
          false => "vanilla".into(),
        },
        preset: "default".into(),
      }],
    };

    let settings_path = config_dir.join("settings.json");

    fs::write(
      &settings_path,
      serde_json::to_string_pretty(&default_settings)? + "\n",
    )?;

    Ok(())
  }

  /// Returns the widget configs to open on startup.
  pub async fn startup_configs(&self) -> Vec<StartupConfig> {
    self.value.lock().await.startup_configs.clone()
  }

  /// Adds the given config to be launched on startup.
  pub async fn add_startup_config(
    &self,
    pack_id: &str,
    widget_name: &str,
    preset_name: &str,
  ) -> anyhow::Result<()> {
    let mut new_settings = { self.value.lock().await.clone() };

    let startup_config = StartupConfig {
      pack: pack_id.to_string(),
      widget: widget_name.to_string(),
      preset: preset_name.to_string(),
    };

    if new_settings.startup_configs.contains(&startup_config) {
      return Ok(());
    }

    new_settings.startup_configs.push(startup_config);
    self.write_settings(new_settings).await
  }

  /// Removes the given config from being launched on startup.
  pub async fn remove_startup_config(
    &self,
    pack_id: &str,
    widget_name: &str,
    preset_name: &str,
  ) -> anyhow::Result<()> {
    let mut new_settings = { self.value.lock().await.clone() };

    new_settings.startup_configs.retain(|config| {
      config.pack != pack_id
        && config.widget != widget_name
        && config.preset != preset_name
    });

    self.write_settings(new_settings).await
  }

  /// Opens the config directory in the OS-dependent file explorer.
  pub fn open_config_dir(&self) -> anyhow::Result<()> {
    #[cfg(target_os = "windows")]
    {
      std::process::Command::new("explorer")
        .arg(self.config_dir.clone())
        .spawn()?;
    }

    #[cfg(target_os = "macos")]
    {
      std::process::Command::new("open")
        .arg(self.config_dir.clone())
        .arg("-R")
        .spawn()?;
    }

    #[cfg(target_os = "linux")]
    {
      std::process::Command::new("xdg-open")
        .arg(self.config_dir.clone())
        .spawn()?;
    }

    Ok(())
  }

  /// Copies and processes a template to the destination directory.
  pub fn init_template(
    &self,
    template_path: &Path,
    dest_dir: &Path,
    context: &tera::Context,
  ) -> anyhow::Result<()> {
    // Resolve the full path to template directory.
    let template_dir = self
      .app_handle
      .path()
      .resolve(
        Path::new("../../resources/templates").join(template_path),
        BaseDirectory::Resource,
      )
      .with_context(|| {
        format!(
          "Unable to resolve {} template resource.",
          template_path.display()
        )
      })?;

    tracing::info!(
      "Copying template from {} to {}",
      template_dir.display(),
      dest_dir.display()
    );

    // Copy all template files.
    copy_dir_all(&template_dir, dest_dir, false)?;

    // Run Tera template engine on all files with a `.tera` extension.
    visit_deep(dest_dir, &mut |entry| {
      if let Some(file_name) = entry.file_name().to_str() {
        if file_name.ends_with(".tera") {
          let path = entry.path();

          if let Ok(contents) = fs::read_to_string(&path) {
            // Render the template using Tera.
            if let Ok(result) =
              tera::Tera::one_off(&contents, context, true)
            {
              let _ = fs::write(&path, result);
            }

            // Remove `.tera` extension from processed files.
            let file_name = file_name.replace(".tera", "");
            let _ = fs::rename(&path, path.with_file_name(file_name));
          }
        }
      }
    })
  }
}

/// Checks if an application is installed and available in the system PATH.
///
/// Returns `true` if the application is found in PATH, `false` otherwise.
fn is_app_installed(app_name: &str) -> bool {
  #[cfg(target_os = "windows")]
  {
    std::process::Command::new("where")
      .arg(app_name)
      .output()
      .map(|output| output.status.success())
      .unwrap_or(false)
  }

  #[cfg(any(target_os = "macos", target_os = "linux"))]
  {
    std::process::Command::new("which")
      .arg(app_name)
      .output()
      .map(|output| output.status.success())
      .unwrap_or(false)
  }
}
