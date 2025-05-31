use std::str::FromStr;

use anyhow::{bail, Context};
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq)]
pub struct LengthValue {
  pub amount: f32,
  pub unit: LengthUnit,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LengthUnit {
  Percentage,
  Pixel,
}

impl LengthValue {
  pub fn to_px(&self, total_px: i32) -> i32 {
    match self.unit {
      LengthUnit::Percentage => {
        (self.amount / 100. * total_px as f32) as i32
      }
      LengthUnit::Pixel => self.amount as i32,
    }
  }

  pub fn to_px_scaled(&self, total_px: i32, scale_factor: f32) -> i32 {
    match self.unit {
      LengthUnit::Percentage => self.to_px(total_px),
      LengthUnit::Pixel => (scale_factor * self.amount) as i32,
    }
  }
}

impl FromStr for LengthValue {
  type Err = anyhow::Error;

  /// Parses a string containing a number followed by a unit (`px`, `%`).
  /// Allows for negative and fractional numbers.
  ///
  /// Example:
  /// ```
  /// LengthValue::from_str("100px") // { amount: 100.0, unit: LengthUnit::Pixel }
  /// LengthValue::from_str("50.5%") // { amount: 50.5, unit: LengthUnit::Percentage }
  /// ```
  fn from_str(unparsed: &str) -> anyhow::Result<Self> {
    let units_regex = Regex::new(r"([+-]?\d+(?:\.\d+)?)(%|px)?")?;

    let err_msg = format!(
      "Not a valid length value '{}'. Must be of format '10px' or '10%'.",
      unparsed
    );

    let captures = units_regex
      .captures(unparsed)
      .context(err_msg.to_string())?;

    let unit_str = captures.get(2).map_or("", |m| m.as_str());
    let unit = match unit_str {
      "px" | "" => LengthUnit::Pixel,
      "%" => LengthUnit::Percentage,
      _ => bail!(err_msg),
    };

    let amount = captures
      .get(1)
      .and_then(|amount_str| f32::from_str(amount_str.into()).ok())
      .context(err_msg.to_string())?;

    Ok(LengthValue { amount, unit })
  }
}

impl Serialize for LengthValue {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let s = match self.unit {
      LengthUnit::Percentage => format!("{}%", self.amount),
      LengthUnit::Pixel => format!("{}px", self.amount),
    };

    serializer.serialize_str(&s)
  }
}

impl<'de> Deserialize<'de> for LengthValue {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    LengthValue::from_str(&s).map_err(serde::de::Error::custom)
  }
}

impl Default for LengthValue {
  fn default() -> Self {
    Self {
      amount: 0.,
      unit: LengthUnit::Pixel,
    }
  }
}
