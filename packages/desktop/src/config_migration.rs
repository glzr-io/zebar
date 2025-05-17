use std::{
  fs,
  path::{Path, PathBuf},
  slice::Iter,
};

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{
  app_settings::{AppSettingsValue, StartupConfig},
  common::read_and_parse_json,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum ConfigMigration {
  V3_0_0StartupConfig,
  V3_0_0WidgetConfig,
}

impl ConfigMigration {
  /// Returns an iterator over all config migrations.
  pub fn iter() -> Iter<'static, ConfigMigration> {
    static MIGRATIONS: [ConfigMigration; 2] = [
      ConfigMigration::V3_0_0StartupConfig,
      ConfigMigration::V3_0_0WidgetConfig,
    ];

    MIGRATIONS.iter()
  }
}

/// Migrates config files to the latest version.
pub fn apply_config_migrations(config_dir: &Path) -> anyhow::Result<()> {
  let migration_file = config_dir.join(".migrations.json");
  let migrations = migrations_to_apply(&migration_file);

  for migration in migrations {
    match migration {
      ConfigMigration::V3_0_0StartupConfig => {
        migrate_startup_config(config_dir)?;
      }
      ConfigMigration::V3_0_0WidgetConfig => {
        migrate_widget_config(config_dir)?;
      }
    }
  }

  Ok(())
}

/// Returns the migrations that need to be applied to the config files.
fn migrations_to_apply(migration_file: &Path) -> Vec<ConfigMigration> {
  if !migration_file.exists() {
    return vec![
      ConfigMigration::V3_0_0StartupConfig,
      ConfigMigration::V3_0_0WidgetConfig,
    ];
  }

  let migrations =
    read_and_parse_json::<Vec<ConfigMigration>>(migration_file)
      .unwrap_or_default();

  // TODO: Should be the inverse of the applied migrations.

  migrations
}

/// Migrates the startup config to the latest version.
fn migrate_startup_config(config_dir: &Path) -> anyhow::Result<()> {
  let settings_file = config_dir.join("settings.json");

  // Read the settings file.
  let settings_content = fs::read_to_string(&settings_file)
    .context("Failed to read settings.json")?;

  let settings_json =
    serde_json::from_str::<serde_json::Value>(&settings_content)
      .context("Failed to parse settings.json")?;

  // Extract and migrate startup configs if they exist.
  let new_startup_configs = settings_json
    .get("startupConfigs")
    .and_then(|configs| {
      serde_json::from_value::<Vec<LegacyStartupConfig>>(configs.clone())
        .ok()
    })
    .map_or_else(Vec::new, |configs| {
      configs.into_iter().map(Into::into).collect()
    });

  // Create new settings with updated schema.
  let new_settings = AppSettingsValue {
    schema: Some(format!(
        "https://github.com/glzr-io/zebar/raw/v{}/resources/settings-schema.json",
      crate::app_settings::VERSION_NUMBER
    )),
    startup_configs: new_startup_configs,
  };

  // Write the migrated settings back to the file.
  fs::write(
    &settings_file,
    serde_json::to_string_pretty(&new_settings)? + "\n",
  )
  .context("Failed to write migrated settings.")?;

  Ok(())
}

/// Migrates the widget config to the latest version.
fn migrate_widget_config(config_dir: &Path) -> anyhow::Result<()> {
  todo!()
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum LegacyStartupConfig {
  /// String format from v2.3.0 and earlier.
  String(String),

  /// Object format from v2.7.0 and earlier.
  Object { path: PathBuf, preset: String },

  /// Current format from v3.0.0 and later.
  Current(StartupConfig),
}

/// Converts a v2 startup config to a v3 startup config.
impl From<LegacyStartupConfig> for StartupConfig {
  fn from(value: LegacyStartupConfig) -> Self {
    match value {
      LegacyStartupConfig::String(s) => {
        let (pack, widget) = parse_path(&PathBuf::from(s));

        StartupConfig {
          pack,
          widget,
          preset: "default".to_string(),
        }
      }
      LegacyStartupConfig::Object { path, preset } => {
        let (pack, widget) = parse_path(&path);

        StartupConfig {
          pack,
          widget,
          preset,
        }
      }
      LegacyStartupConfig::Current(config) => config,
    }
  }
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
