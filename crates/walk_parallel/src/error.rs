use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum WalkError {
  #[error("Handler error: {path} - {error}")]
  HandlerError { path: PathBuf, error: String },

  #[error("Glob error: {0}")]
  GlobError(#[from] wax::BuildError),

  #[error("IO error: {0}")]
  IoError(#[from] std::io::Error),
}
