use std::{
  collections::HashSet,
  path::{Path, PathBuf},
};

use anyhow::Result;
use globset::{Glob, GlobSet, GlobSetBuilder};

/// Creates a `GlobSet` from a collection of glob patterns.
fn create_glob_set(patterns: &[String]) -> Result<GlobSet> {
  let mut builder = GlobSetBuilder::new();

  for pattern in patterns {
    builder.add(Glob::new(pattern)?);
  }

  Ok(builder.build()?)
}

/// Checks if a path matches any of the provided glob patterns.
///
/// Returns `true` if the path matches any of the patterns, otherwise
/// returns `false`.
pub fn is_match(path: &Path, patterns: &[String]) -> Result<bool> {
  if patterns.is_empty() {
    return Ok(false);
  }

  let glob_set = create_glob_set(patterns)?;
  Ok(glob_set.is_match(path))
}

/// Collects all file paths in a directory that match the given glob
/// patterns.
///
/// Returns a `HashSet` of matching file paths.
pub fn matched_paths(
  base_dir: &Path,
  patterns: &[String],
) -> Result<HashSet<PathBuf>> {
  let mut matched_files = HashSet::new();

  if patterns.is_empty() {
    return Ok(matched_files);
  }

  let glob_set = create_glob_set(patterns)?;

  crate::common::visit_deep(base_dir, &mut |entry| {
    let path = entry.path();

    if let Ok(relative_path) = path.strip_prefix(base_dir) {
      if path.is_file() && glob_set.is_match(relative_path) {
        matched_files.insert(path.to_path_buf());
      }
    }
  })?;

  Ok(matched_files)
}
