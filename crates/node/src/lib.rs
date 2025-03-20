use std::{
  fs,
  path::{Path, PathBuf},
};

use doctor_ext::MultiFrom;

pub mod err;

const FILE_NAME: &str = ".node-version";

#[derive(Debug)]
pub struct Node {
  file_path: PathBuf,
}

impl MultiFrom for Node {
  type Error = err::NodeError;

  fn from_cwd<P: AsRef<Path>>(cwd: P) -> Result<Self, Self::Error> {
    let file_path = cwd.as_ref().join(FILE_NAME);
    Ok(Self { file_path })
  }

  fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Self::Error> {
    Ok(Self {
      file_path: path.as_ref().to_path_buf(),
    })
  }
}

impl Node {
  pub fn validate_node_version(self) -> Result<Self, err::NodeError> {
    if !self.file_path.exists() {
      return Err(err::NodeError::NodeVersionFileNotFound(
        self.file_path.to_string_lossy().to_string(),
      ));
    }

    let version = fs::read_to_string(&self.file_path)?;

    let version = version.trim();

    if version.is_empty() {
      return Err(err::NodeError::NodeVersionFileIsEmpty(version.to_string()));
    }

    Ok(self)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_validate_node_version_file_not_found() {
    let node = Node::from_cwd("./examples/not-found").unwrap();
    let result = node.validate_node_version();

    assert!(result.is_err());

    if let Err(err) = result {
      assert!(matches!(err, err::NodeError::NodeVersionFileNotFound(_)));
    }
  }

  #[test]
  fn test_validate_node_version_file_empty() {
    let node = Node::from_cwd("./examples/empty_file").unwrap();
    let result = node.validate_node_version();

    assert!(result.is_err());

    if let Err(err) = result {
      assert!(matches!(err, err::NodeError::NodeVersionFileIsEmpty(_)));
    }
  }

  #[test]
  fn test_validate_node_version_file_success() {
    let node = Node::from_cwd("./examples/success").unwrap();
    let result = node.validate_node_version();

    assert!(result.is_ok());
  }
}
