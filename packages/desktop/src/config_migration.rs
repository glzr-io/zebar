use std::{
  fs,
  path::{Path, PathBuf},
  slice::Iter,
};

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{
  app_settings::{AppSettingsValue, StartupConfig},
  common::{has_extension, read_and_parse_json},
};

/// Migrations that can be applied to the config files.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
  // TODO: Should be stored in data dir instead.
  let migration_path = config_dir.join(".migrations.json");

  let mut applied_migrations =
    read_and_parse_json::<Vec<ConfigMigration>>(&migration_path)
      .unwrap_or_default();

  // Get migrations that have not been applied yet.
  let migrations_to_apply = ConfigMigration::iter()
    .filter(|migration| !applied_migrations.contains(migration))
    .collect::<Vec<_>>();

  for migration in migrations_to_apply {
    match migration {
      ConfigMigration::V3_0_0StartupConfig => {
        migrate_startup_config(config_dir)?;
      }
      ConfigMigration::V3_0_0WidgetConfig => {
        migrate_widget_config(config_dir)?;
      }
    }

    applied_migrations.push(migration.clone());

    // Update the migration file.
    fs::write(
      &migration_path,
      serde_json::to_string_pretty(&applied_migrations)? + "\n",
    )
    .context("Failed to update migration file.")?;
  }

  Ok(())
}

/// Migrates the startup config to the latest version.
fn migrate_startup_config(config_dir: &Path) -> anyhow::Result<()> {
  let settings_path = config_dir.join("settings.json");

  let settings_json =
    read_and_parse_json::<serde_json::Value>(&settings_path)
      .context("Failed to parse settings.json")?;

  // Extract and migrate startup configs if they exist.
  let new_startup_configs = settings_json
    .get("startupConfigs")
    .and_then(|configs| {
      serde_json::from_value::<Vec<StartupConfigFormat>>(configs.clone())
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
    &settings_path,
    serde_json::to_string_pretty(&new_settings)? + "\n",
  )
  .context("Failed to write migrated settings.")?;

  Ok(())
}

/// Migrates the widget config to the latest version.
fn migrate_widget_config(config_dir: &Path) -> anyhow::Result<()> {
  let pack_dirs = fs::read_dir(config_dir)
    .with_context(|| {
      format!("Failed to read directory: {}", config_dir.display())
    })?
    .filter_map(|entry| Some(entry.ok()?.path()))
    .filter(|path| path.is_dir());

  for pack_dir in pack_dirs {
    // Collect all the found `*.zebar.json` files in this subdirectory.
    let widget_configs = match find_widget_configs(&pack_dir) {
      Ok(configs) if !configs.is_empty() => configs,
      // Skip if no widget configs to migrate.
      Ok(_) => continue,
      // Skip if failed to read the directory.
      Err(err) => {
        tracing::warn!(
          "Failed to read directory '{}': {}",
          pack_dir.display(),
          err
        );
        continue;
      }
    };

    let pack_name = pack_dir
      .file_name()
      .and_then(|name| name.to_str())
      .unwrap_or("unknown")
      .to_string();

    // Create a new pack config.
    let mut pack_config = serde_json::json!({
      "$schema": format!(
        "https://github.com/glzr-io/zebar/raw/v{}/resources/pack-schema.json",
        crate::app_settings::VERSION_NUMBER
      ),
      "name": pack_name,
      "description": "",
      "tags": [],
      "previewImages": [],
      "repositoryUrl": "",
      "widgets": []
    });

    // Process each widget config file, adding it to the pack config.
    for config_path in widget_configs {
      // Extract widget name from filename.
      let widget_name = config_path
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown")
        .trim_end_matches(".zebar")
        .to_string();

      // Read the legacy widget config.
      match read_and_parse_json::<serde_json::Value>(&config_path) {
        Ok(mut legacy_config) => {
          if let Some(legacy_obj) = legacy_config.as_object_mut() {
            // Add new required `name` field.
            legacy_obj.insert(
              "name".to_string(),
              serde_json::Value::String(widget_name),
            );

            // Add new required `includeFiles` field.
            legacy_obj
              .insert("includeFiles".to_string(), serde_json::json!([]));

            // Add the migrated widget config to the pack.
            if let Some(widgets) = pack_config["widgets"].as_array_mut() {
              widgets.push(legacy_config);
            }

            tracing::info!(
              "Migrated widget config: {}",
              config_path.display()
            );

            // Remove the legacy widget config file.
            let _ = fs::remove_file(config_path);
          } else {
            tracing::warn!(
              "Widget config is not an object: {}",
              config_path.display()
            );
          }
        }
        Err(err) => {
          tracing::warn!(
            "Failed to parse widget config at '{}': {}",
            config_path.display(),
            err
          );
        }
      }
    }

    let pack_config_path = pack_dir.join("zpack.json");

    // Write the new pack config file.
    fs::write(
      &pack_config_path,
      serde_json::to_string_pretty(&pack_config)? + "\n",
    )
    .with_context(|| {
      format!(
        "Failed to write pack config at: {}",
        pack_config_path.display()
      )
    })?;

    tracing::info!(
      "Created pack config at: {}",
      pack_config_path.display()
    );
  }

  Ok(())
}

/// Finds all legacy widget config files within a given directory.
fn find_widget_configs(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
  Ok(
    fs::read_dir(dir)?
      .filter_map(Result::ok)
      .map(|entry| entry.path())
      .filter(|path| path.is_file() && has_extension(path, ".zebar.json"))
      .collect(),
  )
}

/// Legacy and current structure for startup configs.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum StartupConfigFormat {
  /// String format from v2.3.0 and earlier.
  String(String),

  /// Object format from v2.7.0 and earlier.
  Object { path: PathBuf, preset: String },

  /// Current format from v3.0.0 and later.
  Current(StartupConfig),
}

/// Converts a v2 startup config to a v3 startup config.
impl From<StartupConfigFormat> for StartupConfig {
  fn from(value: StartupConfigFormat) -> Self {
    match value {
      StartupConfigFormat::String(s) => {
        let (pack, widget) = parse_legacy_path(&PathBuf::from(s));

        StartupConfig {
          pack,
          widget,
          preset: "default".to_string(),
        }
      }
      StartupConfigFormat::Object { path, preset } => {
        let (pack, widget) = parse_legacy_path(&path);

        StartupConfig {
          pack,
          widget,
          preset,
        }
      }
      StartupConfigFormat::Current(config) => config,
    }
  }
}

/// Parses a path to a pack and widget name.
fn parse_legacy_path(path: &Path) -> (String, String) {
  let path = path.to_string_lossy();
  let path = path
    .trim_start_matches(['.', '/', '\\'])
    .trim_end_matches(".zebar.json");

  // Split the path into pack and widget name.
  // TODO: Transform pack ID if necessary. It might include special
  // symbols or spaces.
  path.split_once(['/', '\\']).map_or(
    (path.to_string(), String::new()),
    |(pack_dir, widget_name)| {
      (pack_dir.to_string(), widget_name.to_string())
    },
  )
}

/// Sanitizes a pack/widget name to match the schema requirements:
/// - 2-24 characters.
/// - Only lowercase letters, numbers, hyphens, and underscores.
/// - Must start with a letter or number.
fn sanitize_name(name: String) -> String {
  // Convert to lowercase.
  let name = name.to_lowercase();

  // Replace invalid characters with underscores.
  let sanitized = name
    .chars()
    .map(|c| {
      if c.is_ascii_lowercase()
        || c.is_ascii_digit()
        || c == '-'
        || c == '_'
      {
        c
      } else {
        '_'
      }
    })
    .collect::<String>();

  // Ensure it starts with a letter or number.
  let sanitized = if !(sanitized.is_empty()
    || sanitized.chars().next().unwrap().is_ascii_lowercase()
    || sanitized.chars().next().unwrap().is_ascii_digit())
  {
    format!("x{}", sanitized)
  } else {
    sanitized
  };

  // Ensure minimum length.
  let sanitized = if sanitized.len() < 2 {
    format!("{}_", sanitized)
  } else {
    sanitized
  };

  // Truncate if too long.
  if sanitized.len() > 24 {
    sanitized[0..24].to_string()
  } else {
    sanitized
  }
}
