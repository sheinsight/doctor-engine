use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeVersionError {
  #[error("IO error: {0}")]
  IoError(#[from] std::io::Error),

  #[error("Node version file not found: {0}")]
  NodeVersionFileNotFound(String),

  #[error("Node version file is empty: {0}")]
  NodeVersionFileIsEmpty(String),
}
