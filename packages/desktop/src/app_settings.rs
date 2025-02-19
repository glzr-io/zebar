use std::{
  collections::HashMap,
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
  /// Relative path to widget configs to launch on startup.
  pub path: PathBuf,

  /// Preset name within the widget config.
  pub preset: String,
}

// Deserializer that handles `StartupConfig` objects and string format from
// v2.3.0 and earlier.
impl<'de> Deserialize<'de> for StartupConfig {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrObject {
      String(String),
      Object { path: PathBuf, preset: String },
    }

    let value = StringOrObject::deserialize(deserializer)?;

    Ok(match value {
      StringOrObject::String(s) => StartupConfig {
        path: PathBuf::from(s),
        preset: "default".to_string(),
      },
      StringOrObject::Object { path, preset } => {
        StartupConfig { path, preset }
      }
    })
  }
}

/// Represents templates that can be initialized from the `templates/`
/// directory.
#[derive(Debug)]
pub enum TemplateResource {
  /// Template for creating a new widget pack.
  Pack,

  /// Template for creating a new widget with specified frontend
  /// framework.
  Widget(FrontendTemplate),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FrontendTemplate {
  ReactBuildless,
  SolidTypescript,
}

#[derive(Debug)]
pub struct AppSettings {
  /// Handle to the Tauri application.
  app_handle: AppHandle,

  /// Directory where config files are stored.
  pub config_dir: PathBuf,

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

    let settings = Self::read_settings_or_init(&config_dir)?;
    let (settings_change_tx, _settings_change_rx) = broadcast::channel(16);

    Ok(Self {
      app_handle: app_handle.clone(),
      config_dir: config_dir.to_absolute()?,
      value: Arc::new(Mutex::new(settings)),
      _settings_change_rx,
      settings_change_tx,
    })
  }

  /// Re-evaluates app settings and broadcasts the change.
  pub async fn reload(&self) -> anyhow::Result<()> {
    let new_settings = Self::read_settings_or_init(&self.config_dir)?;

    {
      let mut settings = self.value.lock().await;
      *settings = new_settings.clone();
    }

    self.settings_change_tx.send(new_settings)?;

    Ok(())
  }

  /// Reads the app settings file or initializes it with the template.
  ///
  /// Returns the parsed `AppSettingsValue`.
  fn read_settings_or_init(
    dir: &Path,
  ) -> anyhow::Result<AppSettingsValue> {
    let settings = Self::read_settings(dir)?;

    match settings {
      Some(settings) => Ok(settings),
      None => {
        Self::create_default(dir)?;

        Self::read_settings(dir)?
          .context("Failed to create settings config.")
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
  /// `settings.json` is initialized with either `starter/vanilla` or
  /// `starter/with-glazewm` as startup config.
  fn create_default(config_dir: &Path) -> anyhow::Result<()> {
    tracing::info!("Initializing app settings from default.",);

    let default_settings = AppSettingsValue {
      schema: Some("https://github.com/glzr-io/zebar/raw/v2.4.0/resources/settings-schema.json".into()),
      startup_configs: vec![StartupConfig {
        path: match is_app_installed("glazewm") {
          true => "starter/with-glazewm.zebar.json".into(),
          false => "starter/vanilla.zebar.json".into(),
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

  /// Returns the widget configs to open on startup.
  pub async fn startup_configs_by_path(
    &self,
  ) -> anyhow::Result<HashMap<PathBuf, StartupConfig>> {
    self
      .startup_configs()
      .await
      .into_iter()
      .map(|config| {
        self
          .to_absolute_path(&config.path)
          .map(|abs_path| (abs_path, config))
      })
      .collect()
  }

  /// Adds the given config to be launched on startup.
  ///
  /// Config path can be either absolute or relative.
  pub async fn add_startup_config(
    &self,
    config_path: &Path,
    preset_name: &str,
  ) -> anyhow::Result<()> {
    let mut new_settings = { self.value.lock().await.clone() };

    let startup_config = StartupConfig {
      path: self.to_relative_path(config_path),
      preset: preset_name.to_string(),
    };

    if new_settings.startup_configs.contains(&startup_config) {
      return Ok(());
    }

    new_settings.startup_configs.push(startup_config);
    self.write_settings(new_settings).await
  }

  /// Removes the given config from being launched on startup.
  ///
  /// Config path can be either absolute or relative.
  pub async fn remove_startup_config(
    &self,
    config_path: &Path,
    preset_name: &str,
  ) -> anyhow::Result<()> {
    let mut new_settings = { self.value.lock().await.clone() };
    let rel_path = self.to_relative_path(config_path);

    new_settings.startup_configs.retain(|config| {
      config.path != rel_path || config.preset != preset_name
    });

    self.write_settings(new_settings).await
  }

  /// Joins the given path with the config directory path.
  ///
  /// Returns an absolute path.
  pub fn to_absolute_path(
    &self,
    config_path: &PathBuf,
  ) -> anyhow::Result<PathBuf> {
    match config_path.is_absolute() {
      false => self.config_dir.join(config_path).to_absolute(),
      // Ensure path is canonicalized even if already absolute.
      true => config_path.to_absolute(),
    }
  }

  /// Strips the config directory path from the given path.
  ///
  /// Returns a relative path.
  pub fn to_relative_path(&self, config_path: &Path) -> PathBuf {
    config_path
      .strip_prefix(&self.config_dir)
      .unwrap_or(config_path)
      .into()
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
    template: TemplateResource,
    dest_dir: &Path,
    context: &HashMap<&str, String>,
  ) -> anyhow::Result<()> {
    // Determine source template path based on template type.
    let template_path = match template {
      TemplateResource::Pack => "pack-template",
      TemplateResource::Widget(frontend) => match frontend {
        FrontendTemplate::ReactBuildless => {
          "widget-templates/react-buildless"
        }
        FrontendTemplate::SolidTypescript => "widget-templates/solid-ts",
      },
    };

    // Resolve the full path to template directory.
    let template_dir = self
      .app_handle
      .path()
      .resolve(
        format!("../../templates/{}", template_path),
        BaseDirectory::Resource,
      )
      .with_context(|| {
        format!("Unable to resolve {} template resource.", template_path)
      })?;

    tracing::info!(
      "Copying template from {} to {}",
      template_dir.display(),
      dest_dir.display()
    );

    // Copy all template files.
    copy_dir_all(&template_dir, dest_dir, false)?;

    let context = tera::Context::from_serialize(context)?;

    // Run Tera template engine on all files with a `.tera` extension.
    visit_deep(dest_dir, &|entry| {
      if let Some(file_name) = entry.file_name().to_str() {
        if file_name.ends_with(".tera") {
          let path = entry.path();

          if let Ok(contents) = fs::read_to_string(&path) {
            // Render the template using Tera.
            if let Ok(result) =
              tera::Tera::one_off(&contents, &context, true)
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
