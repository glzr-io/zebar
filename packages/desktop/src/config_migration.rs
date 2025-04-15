use serde::{Deserialize, Serialize};

use crate::{app_settings::AppSettings, common::read_and_parse_json};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum ConfigMigration {
  V3_0_0StartupConfig,
  V3_0_0WidgetConfig,
}

/// Returns the migrations that need to be applied to the config files.
pub fn migrations_to_apply(
  app_settings: &AppSettings,
) -> Vec<ConfigMigration> {
  let migration_file = app_settings.config_dir.join(".migrations.json");

  if !migration_file.exists() {
    return vec![
      ConfigMigration::V3_0_0StartupConfig,
      ConfigMigration::V3_0_0WidgetConfig,
    ];
  }

  let migrations =
    read_and_parse_json::<Vec<ConfigMigration>>(&migration_file)
      .unwrap_or_default();

  migrations
}

/// Migrates config files to the latest version.
pub fn apply_config_migrations(
  app_settings: &AppSettings,
) -> anyhow::Result<()> {
  let migrations = migrations_to_apply(app_settings);

  for migration in migrations {
    match migration {
      ConfigMigration::V3_0_0StartupConfig => {
        migrate_v3_0_0_startup_config(app_settings)
      }
      ConfigMigration::V3_0_0WidgetConfig => {
        migrate_v3_0_0_widget_config(app_settings)
      }
    }
  }

  Ok(())
}
