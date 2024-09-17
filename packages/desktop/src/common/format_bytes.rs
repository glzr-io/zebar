const SI_UNITS: [&'static str; 9] =
  ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

const IEC_UNITS: [&'static str; 9] =
  ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];

/// Converts a byte value to its SI (decimal) representation.
///
/// Returns a tuple of the value and the SI unit as a string.
pub fn to_si_bytes(bytes: f64) -> (f64, String) {
  if bytes < 1. && bytes > -1. {
    return (bytes, "B".into());
  }

  let exponent = std::cmp::min(
    (bytes.abs().log10() / 3.).floor() as i32,
    (SI_UNITS.len() - 1) as i32,
  );

  (
    bytes / 1000f64.powi(exponent),
    SI_UNITS[exponent as usize].into(),
  )
}

/// Converts a byte value to its IEC (binary) representation.
///
/// Returns a tuple of the value and the IEC unit as a string.
pub fn to_iec_bytes(bytes: f64) -> (f64, String) {
  if bytes <= 1. && bytes >= -1. {
    return (bytes, "B".into());
  }

  let exponent = std::cmp::min(
    (bytes.abs().log2() / 10.).floor() as i32,
    (IEC_UNITS.len() - 1) as i32,
  );

  (
    bytes / 1024f64.powi(exponent),
    IEC_UNITS[exponent as usize].into(),
  )
}
