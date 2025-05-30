use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum WalkError {
  #[error("Handler error: {path} - {error}")]
  HandlerError { path: PathBuf, error: String },

  #[error("IO error: {0}")]
  IoError(#[from] std::io::Error),

  #[error("Unknown error: {0}")]
  Unknown(String),
}
