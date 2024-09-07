use std::{
  fs,
  path::{Component, Path, PathBuf, Prefix},
};

pub trait PathExt
where
  Self: AsRef<Path>,
{
  /// Like `std::fs::canonicalize()`, but on Windows it outputs paths
  /// without UNC.
  ///
  /// Example:
  /// ```
  /// let path = PathBuf::from("\\?\C:\\Users\\John\\Desktop\\test");
  /// path.canonicalize_pretty().unwrap(); // "C:\\Users\\John\\Desktop\\test"
  /// ```
  fn canonicalize_pretty(&self) -> anyhow::Result<String>;

  /// Converts the path to a unicode string.
  ///
  /// Short-hand for `.to_string_lossy().to_string()`.
  fn to_unicode_string(&self) -> String;
}

impl PathExt for PathBuf {
  fn canonicalize_pretty(&self) -> anyhow::Result<String> {
    let canonicalized = fs::canonicalize(self)?;
    let canonicalized_str = canonicalized.to_unicode_string();

    #[cfg(not(windows))]
    {
      Ok(canonicalized_str)
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

      let stripped_str = if should_strip_unc {
        canonicalized_str
          .get(4..)
          .map(Into::into)
          .unwrap_or(canonicalized_str)
      } else {
        canonicalized_str
      };

      Ok(stripped_str)
    }
  }

  fn to_unicode_string(&self) -> String {
    self.to_string_lossy().to_string()
  }
}
