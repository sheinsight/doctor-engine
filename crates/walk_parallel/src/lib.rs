use error::WalkError;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use walk_patterns::WalkPatterns;
use wax::Glob;
pub mod error;
pub mod walk_patterns;

pub const JS_FILE_EXTENSIONS: &[&str] = &["js", "jsx", "cjs", "mjs"];

pub const TS_FILE_EXTENSIONS: &[&str] = &["ts", "tsx", "cts", "mts"];

pub const DEFAULT_PATTERNS: &[&str] = &["**/*.{js,jsx,ts,tsx,cjs,mjs,cts,mts}"];

pub struct WalkParallel<'a> {
  cwd: &'a Path,
  patterns: WalkPatterns<'a>,
}

impl<'a> WalkParallel<'a> {
  pub fn new<P: AsRef<Path> + 'a>(cwd: &'a P) -> Self {
    Self {
      cwd: cwd.as_ref(),
      patterns: WalkPatterns::default(),
    }
  }

  pub fn with_patterns(mut self, patterns: WalkPatterns<'a>) -> Self {
    self.patterns = patterns;
    self
  }

  pub fn walk<F, R>(&self, f: F) -> Result<Vec<Result<R, WalkError>>, WalkError>
  where
    F: Fn(PathBuf) -> Result<R, WalkError> + Send + Sync,
    R: Send + Sync,
  {
    let glob = Glob::new(self.patterns.walk)?;
    let x = self.patterns.ignore.into_iter().copied();
    let entries = glob.walk(self.cwd).not(x)?;

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
