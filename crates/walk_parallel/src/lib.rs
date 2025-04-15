use rayon::prelude::*;
use std::path::{Path, PathBuf};
use typed_builder::TypedBuilder;
use wax::Glob;
mod error;
mod walk_patterns;
pub use error::WalkError;
pub use walk_patterns::WalkPatterns;

#[derive(Debug, Clone, TypedBuilder)]
pub struct WalkParallel<'a> {
  cwd: &'a Path,
  #[builder(default = WalkPatterns::default())]
  patterns: WalkPatterns,
}

impl<'a> WalkParallel<'a> {
  pub fn walk<F, R>(&self, f: F) -> Result<Vec<Result<R, WalkError>>, WalkError>
  where
    F: Fn(PathBuf) -> Result<R, WalkError> + Send + Sync,
    R: Send + Sync,
  {
    let glob = Glob::new(&self.patterns.walk)?;
    let ignore = self
      .patterns
      .ignore
      .iter()
      .map(|s| s.as_str())
      .collect::<Vec<_>>();
    let entries = glob.walk(self.cwd).not(ignore)?;

    let res = entries
      .par_bridge()
      .filter_map(Result::ok)
      .map(|entry| entry.path().to_owned())
      .filter(|path| path.is_file())
      .map(f)
      .collect::<Vec<Result<R, WalkError>>>();

    Ok(res)
  }
}
