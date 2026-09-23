use std::{fs, path::PathBuf};

use schemars::{generate::SchemaSettings, JsonSchema};
use serde_json::{json, Value};

use crate::{
  app_settings::AppSettingsValue,
  common::LengthValue,
  widget_pack::{MonitorSelection, WidgetPackConfig},
};

fn schema<T: JsonSchema>() -> Value {
  serde_json::to_value(
    SchemaSettings::draft07()
      .into_generator()
      .into_root_schema_for::<T>(),
  )
  .unwrap()
}

fn schemas() -> [(&'static str, Value); 2] {
  [
    ("settings-schema.json", schema::<AppSettingsValue>()),
    ("zpack-schema.json", schema::<WidgetPackConfig>()),
  ]
}

fn resources() -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../resources")
}

#[test]
#[ignore = "Run explicitly to regenerate the committed schemas"]
fn generate() {
  for (name, schema) in schemas() {
    fs::write(
      resources().join(name),
      serde_json::to_string_pretty(&schema).unwrap() + "\n",
    )
    .unwrap();
  }
}

#[test]
fn committed_schemas_match_rust_types() {
  for (name, expected) in schemas() {
    let actual: Value = serde_json::from_str(
      &fs::read_to_string(resources().join(name)).unwrap(),
    )
    .unwrap();
    assert_eq!(
      actual, expected,
      "Regenerate {name}: pnpm schema:generate"
    );
  }
}

#[test]
fn monitor_selection_matches_serde() {
  let validator =
    jsonschema::validator_for(&schema::<MonitorSelection>()).unwrap();
  for value in [
    json!({"type": "all"}),
    json!({"type": "primary"}),
    json!({"type": "secondary"}),
    json!({"type": "index", "match": 0}),
    json!({"type": "name", "match": "DISPLAY1"}),
  ] {
    let parsed: MonitorSelection =
      serde_json::from_value(value.clone()).unwrap();
    assert!(validator.is_valid(&value));
    assert!(validator.is_valid(&serde_json::to_value(parsed).unwrap()));
  }
  for value in [
    json!({"type": "name"}),
    json!({"type": "name", "match": 1}),
    json!({"type": "index", "match": -1}),
    json!({"type": "unknown"}),
  ] {
    assert!(
      serde_json::from_value::<MonitorSelection>(value.clone()).is_err()
    );
    assert!(!validator.is_valid(&value));
  }
}

#[test]
fn lengths_use_the_serialized_string_representation() {
  let validator =
    jsonschema::validator_for(&schema::<LengthValue>()).unwrap();
  for text in ["100px", "50%", "-12.5px", "+2.5%", "10"] {
    let length: LengthValue = serde_json::from_value(json!(text)).unwrap();
    let serialized = serde_json::to_value(length).unwrap();
    assert!(serialized.is_string());
    assert!(validator.is_valid(&serialized));
  }
  for value in [json!(100), json!({"amount": 100, "unit": "pixel"})] {
    assert!(!validator.is_valid(&value));
    assert!(serde_json::from_value::<LengthValue>(value).is_err());
  }
}

#[test]
fn starter_pack_and_settings_examples_validate() {
  let pack: Value = serde_json::from_str(
    &fs::read_to_string(resources().join("starter/zpack.json")).unwrap(),
  )
  .unwrap();
  serde_json::from_value::<WidgetPackConfig>(pack.clone()).unwrap();
  assert!(jsonschema::is_valid(&schema::<WidgetPackConfig>(), &pack));

  let minimal_pack = json!({"name": "example", "version": "1.0.0"});
  serde_json::from_value::<WidgetPackConfig>(minimal_pack.clone())
    .unwrap();
  assert!(jsonschema::is_valid(
    &schema::<WidgetPackConfig>(),
    &minimal_pack,
  ));

  let settings = schema::<AppSettingsValue>();
  for example in settings["examples"].as_array().unwrap() {
    serde_json::from_value::<AppSettingsValue>(example.clone()).unwrap();
    assert!(jsonschema::is_valid(&settings, example));
  }
}
