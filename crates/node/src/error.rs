use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeVersionValidatorError {
  #[error("{0} {1}")]
  IoError(String, #[source] std::io::Error),

  #[error("{0} Node version file not found")]
  NodeVersionFileNotFound(String),

  #[error("{0} Node version file is empty")]
  NodeVersionFileIsEmpty(String),
}
