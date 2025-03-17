use thiserror::Error;

#[derive(Debug, Error)]
pub enum LintError {
  #[error("IO error: {0}")]
  Io(#[from] std::io::Error),

  #[error("Other error: {0}")]
  Other(String),

  #[error("Failed to build config: {0}")]
  FailedToBuildConfig(String),
}
