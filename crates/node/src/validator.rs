use std::{
  fs,
  path::{Path, PathBuf},
};

use doctor_ext::{MultiFrom, PathExt, Validator};

use crate::error::NodeVersionError;

const FILE_NAME: &str = ".node-version";

#[derive(Debug)]
pub struct NodeVersionValidator {
  file_path: PathBuf,
}

impl NodeVersionValidator {}

impl Validator for NodeVersionValidator {
  type Error = NodeVersionError;

  fn validate(&self) -> Result<(), Self::Error> {
    if !self.file_path.exists() {
      return Err(NodeVersionError::NodeVersionFileNotFound(
        self.file_path.to_string_owned(),
      ));
    }

    let version = fs::read_to_string(&self.file_path)?;

    let version = version.trim();

    if version.is_empty() {
      return Err(NodeVersionError::NodeVersionFileIsEmpty(
        version.to_string(),
      ));
    }

    Ok(())
  }
}

impl MultiFrom for NodeVersionValidator {
  type Error = NodeVersionError;

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_validate_node_version_file_not_found() {
    let node = NodeVersionValidator::from_cwd("./examples/not-found").unwrap();
    let result = node.validate();

    assert!(result.is_err());

    if let Err(err) = result {
      assert!(matches!(err, NodeVersionError::NodeVersionFileNotFound(_)));
    }
  }

  #[test]
  fn test_validate_node_version_file_empty() {
    let node = NodeVersionValidator::from_cwd("./examples/empty_file").unwrap();
    let result = node.validate();

    assert!(result.is_err());

    if let Err(err) = result {
      assert!(matches!(err, NodeVersionError::NodeVersionFileIsEmpty(_)));
    }
  }

  #[test]
  fn test_validate_node_version_file_success() {
    let node = NodeVersionValidator::from_cwd("./examples/success").unwrap();
    let result = node.validate();

    assert!(result.is_ok());
  }
}
