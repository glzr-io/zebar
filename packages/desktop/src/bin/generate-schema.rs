use std::{fs, path::PathBuf};

use schemars::{generate::SchemaSettings, JsonSchema};

#[allow(dead_code)]
#[path = "../config_types.rs"]
mod config_types;

fn write_schema<T: JsonSchema>(name: &str) -> anyhow::Result<()> {
  let schema = SchemaSettings::draft07()
    .into_generator()
    .into_root_schema_for::<T>();
  let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .join("../../resources")
    .join(name);
  fs::write(
    path,
    serde_json::to_string_pretty(&serde_json::to_value(schema)?)? + "\n",
  )?;
  Ok(())
}

fn main() -> anyhow::Result<()> {
  write_schema::<config_types::AppSettingsValue>("settings-schema.json")?;
  write_schema::<config_types::WidgetPackConfig>("zpack-schema.json")
}
