use std::{
  fs,
  path::{Component, Path, PathBuf, Prefix},
};

use anyhow::Context;

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
  fn canonicalize_pretty(&self) -> anyhow::Result<PathBuf>;

  /// Strips the given base path. Path delimiters get normalized to forward
  /// slashes.
  ///
  /// Returns a relative path (e.g. 'subdir/file.json').
  fn to_relative(&self, base_path: &Path) -> anyhow::Result<PathBuf>;

  /// Joins the given base path.
  ///
  /// Returns an absolute path (e.g. 'C:\\Users\\John\\Desktop\\test').
  fn to_absolute(&self, base_path: &Path) -> anyhow::Result<PathBuf>;

  /// Converts the path to a unicode string.
  ///
  /// Short-hand for `.to_string_lossy().to_string()`.
  fn to_unicode_string(&self) -> String;
}

impl PathExt for PathBuf {
  fn canonicalize_pretty(&self) -> anyhow::Result<PathBuf> {
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

  fn to_relative(&self, base_path: &Path) -> anyhow::Result<PathBuf> {
    let path_to_normalize = if !self.is_absolute() {
      self.to_path_buf()
    } else {
      self
        .strip_prefix(base_path)
        .with_context(|| {
          format!("Unable to convert path to relative: {}", self.display())
        })?
        .to_path_buf()
    };

    // Convert to string and normalize delimiters to forward slashes.
    let path_str = path_to_normalize.to_string_lossy().replace('\\', "/");

    Ok(PathBuf::from(path_str))
  }

  fn to_absolute(&self, base_path: &Path) -> anyhow::Result<PathBuf> {
    let absolute_path = if self.is_absolute() {
      self
    } else {
      &base_path.join(self)
    };

    // Ensure path is canonicalized even if already absolute.
    absolute_path.canonicalize_pretty().with_context(|| {
      format!(
        "Unable to convert path to absolute: {}",
        absolute_path.display()
      )
    })
  }

  fn to_unicode_string(&self) -> String {
    self.to_string_lossy().to_string()
  }
}

impl PathExt for Path {
  fn canonicalize_pretty(&self) -> anyhow::Result<PathBuf> {
    self.to_path_buf().canonicalize_pretty()
  }

  fn to_relative(&self, base_path: &Path) -> anyhow::Result<PathBuf> {
    self.to_path_buf().to_relative(base_path)
  }

  fn to_absolute(&self, base_path: &Path) -> anyhow::Result<PathBuf> {
    self.to_path_buf().to_absolute(base_path)
  }

  fn to_unicode_string(&self) -> String {
    self.to_string_lossy().to_string()
  }
}
