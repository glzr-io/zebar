use std::{
  fs,
  path::{Component, Path, PathBuf, Prefix},
};

pub trait PathExt
where
  Self: AsRef<Path>,
{
  /// Like `std::fs::canonicalize()`, but on Windows it outputs paths
  /// without UNC prefix. Similar to the `dunce::canonicalize` crate fn.
  ///
  /// Example:
  /// ```
  /// let path = PathBuf::from("\\?\C:\\Users\\John\\Desktop\\test");
  /// path.canonicalize_pretty().unwrap(); // "C:\\Users\\John\\Desktop\\test"
  /// ```
  fn to_absolute(&self) -> anyhow::Result<PathBuf>;

  /// Converts the path to a unicode string.
  ///
  /// Short-hand for `.to_string_lossy().to_string()`.
  fn to_unicode_string(&self) -> String;
}

impl PathExt for PathBuf {
  fn to_absolute(&self) -> anyhow::Result<PathBuf> {
    let canonicalized = fs::canonicalize(self)?;

    #[cfg(not(windows))]
    {
      Ok(canonicalized)
    }
    #[cfg(windows)]
    {
      let should_strip_unc = match canonicalized.components().next() {
        Some(Component::Prefix(prefix)) => match prefix.kind() {
          Prefix::VerbatimDisk(_) => true,
          _ => false,
        },
        _ => false,
      };

      let formatted = match should_strip_unc {
        true => canonicalized
          .to_str()
          .and_then(|path| path.get(4..))
          .map_or(canonicalized.clone(), PathBuf::from),
        false => canonicalized,
      };

      Ok(formatted)
    }
  }

  fn to_unicode_string(&self) -> String {
    self.to_string_lossy().to_string()
  }
}

impl PathExt for Path {
  fn to_absolute(&self) -> anyhow::Result<PathBuf> {
    self.to_path_buf().to_absolute()
  }

  fn to_unicode_string(&self) -> String {
    self.to_string_lossy().to_string()
  }
}
