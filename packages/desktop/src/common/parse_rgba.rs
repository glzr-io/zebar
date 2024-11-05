#[derive(Debug)]
pub enum ColorError {
  InvalidFormat,
  InvalidValue,
}

pub fn parse_rgba(
  color_str: &str,
) -> Result<(u8, u8, u8, u8), ColorError> {
  let content = color_str
    .trim_start_matches("rgba(")
    .trim_end_matches(")")
    .trim();

  let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();

  if parts.len() != 4 {
    return Err(ColorError::InvalidFormat);
  }

  let r = parts[0].parse().map_err(|_| ColorError::InvalidValue)?;
  let g = parts[1].parse().map_err(|_| ColorError::InvalidValue)?;
  let b = parts[2].parse().map_err(|_| ColorError::InvalidValue)?;

  let a = if parts[3].ends_with('%') {
    let percent = parts[3]
      .trim_end_matches('%')
      .parse::<f32>()
      .map_err(|_| ColorError::InvalidValue)?;
    if percent < 0.0 || percent > 100.0 {
      return Err(ColorError::InvalidValue);
    }
    (percent / 100.0 * 255.0) as u8
  } else {
    let alpha = parts[3]
      .parse::<f32>()
      .map_err(|_| ColorError::InvalidValue)?;
    if alpha < 0.0 || alpha > 1.0 {
      return Err(ColorError::InvalidValue);
    }
    (alpha * 255.0) as u8
  };

  Ok((r, g, b, a))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_rgba() {
    // Test valid inputs
    assert_eq!(
      parse_rgba("rgba(255, 0, 0, 1.0)").unwrap(),
      (255, 0, 0, 255)
    );
    assert_eq!(
      parse_rgba("rgba(255, 0, 0, 0.5)").unwrap(),
      (255, 0, 0, 127)
    );
    assert_eq!(
      parse_rgba("rgba(255, 0, 0, 100%)").unwrap(),
      (255, 0, 0, 255)
    );
    assert_eq!(
      parse_rgba("rgba(255, 0, 0, 50%)").unwrap(),
      (255, 0, 0, 127)
    );

    // Test invalid inputs
    assert!(parse_rgba("rgb(255, 0, 0)").is_err());
    assert!(parse_rgba("rgba(300, 0, 0, 1.0)").is_err());
    assert!(parse_rgba("rgba(255, 0, 0, 1.5)").is_err());
    assert!(parse_rgba("rgba(255, 0, 0, 101%)").is_err());
    assert!(parse_rgba("rgba(255, 0, 0)").is_err());
    assert!(parse_rgba("rgba(255, 0, 0, invalid)").is_err());
  }
}
