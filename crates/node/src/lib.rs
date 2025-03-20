use std::{fs, path::Path};

use doctor_ext::PathExt;

pub mod err;

const FILE_NAME: &str = ".node-version";

#[derive(Debug)]
pub struct Node<P: AsRef<Path>> {
  pub cwd: P,
}

impl<P: AsRef<Path>> Node<P> {
  pub fn new(cwd: P) -> Self {
    Self { cwd }
  }

  pub fn validate_node_version(self) -> Result<Self, err::NodeError> {
    let cwd = self.cwd.as_ref();

    let node_version_file = cwd.join(FILE_NAME);

    if !node_version_file.exists() {
      return Err(err::NodeError::NodeVersionFileNotFound(
        node_version_file.to_string_owned(),
      ));
    }

    let version = fs::read_to_string(node_version_file).map_err(|e| err::NodeError::IoError(e))?;

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
    let node = Node::new("./examples/not-found");
    let result = node.validate_node_version();

    assert!(result.is_err());

    if let Err(err) = result {
      assert!(matches!(err, err::NodeError::NodeVersionFileNotFound(_)));
    }
  }
}
