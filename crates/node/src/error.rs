use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeVersionValidatorError {
  #[error("{0} {1}")]
  IoError(String, #[source] std::io::Error),

  #[error("{} Node version file not found",.0.path)]
  NodeVersionFileNotFound(NodeVersionFileNotFound),

  #[error("{} Node version file is empty",.0.path)]
  NodeVersionFileIsEmpty(NodeVersionFileIsEmpty),

  #[error("Invalid node version: {}",.0.version)]
  InvalidNodeVersion(InvalidNodeVersion),
}

#[derive(Debug)]
pub struct NodeVersionFileNotFound {
  pub path: String,
}

impl NodeVersionFileNotFound {
  pub fn with_path(path: String) -> Self {
    Self { path }
  }
}

impl From<NodeVersionFileNotFound> for NodeVersionValidatorError {
  fn from(value: NodeVersionFileNotFound) -> Self {
    NodeVersionValidatorError::NodeVersionFileNotFound(value)
  }
}

#[derive(Debug)]
pub struct NodeVersionFileIsEmpty {
  pub path: String,
}

impl NodeVersionFileIsEmpty {
  pub fn with_path(path: String) -> Self {
    Self { path }
  }
}

impl From<NodeVersionFileIsEmpty> for NodeVersionValidatorError {
  fn from(value: NodeVersionFileIsEmpty) -> Self {
    NodeVersionValidatorError::NodeVersionFileIsEmpty(value)
  }
}

#[derive(Debug)]
pub struct InvalidNodeVersion {
  pub version: String,
}

impl InvalidNodeVersion {
  pub fn with_version(version: String) -> Self {
    Self { version }
  }
}

impl From<InvalidNodeVersion> for NodeVersionValidatorError {
  fn from(value: InvalidNodeVersion) -> Self {
    NodeVersionValidatorError::InvalidNodeVersion(value)
  }
}
