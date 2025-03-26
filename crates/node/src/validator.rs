use std::{
  fs,
  path::{Path, PathBuf},
};

use doctor_ext::{MultiFrom, PathExt, Validator};
use lazy_regex::regex_is_match;

use crate::error::{
  InvalidNodeVersion, NodeVersionFileIsEmpty, NodeVersionFileNotFound, NodeVersionValidatorError,
};

const FILE_NAME: &str = ".node-version";

/// NodeVersionValidator
///
/// # Example
///
/// ```rust
/// use doctor_node::validator::NodeVersionValidator;
/// use doctor_ext::MultiFrom;
/// use doctor_ext::Validator;
///
/// let validator = NodeVersionValidator::from_cwd("fixtures/success").unwrap();
/// assert!(validator.validate().is_ok());
/// ```
#[derive(Debug)]
pub struct NodeVersionValidator {
  file_path: PathBuf,
}

impl NodeVersionValidator {}

impl Validator for NodeVersionValidator {
  type Error = NodeVersionValidatorError;

  /// Validate node version
  ///
  /// # Example
  ///
  /// ```rust
  /// use doctor_node::validator::NodeVersionValidator;
  /// use doctor_ext::MultiFrom;
  /// use doctor_ext::Validator;
  ///
  /// let validator = NodeVersionValidator::from_cwd("fixtures/success").unwrap();
  /// assert!(validator.validate().is_ok());
  /// ```
  fn validate(&self) -> Result<(), Self::Error> {
    if !self.file_path.exists() {
      return Err(NodeVersionFileNotFound::with_path(self.file_path.to_string_owned()).into());
    }

    let version = fs::read_to_string(&self.file_path)
      .map_err(|e| NodeVersionValidatorError::IoError(self.file_path.to_string_owned(), e))?;

    let version = version.trim();

    if version.is_empty() {
      return Err(NodeVersionFileIsEmpty::with_path(self.file_path.to_string_owned()).into());
    }

    let is_valid = regex_is_match!(r#"^\d+\.\d+\.\d+$"#, version);

    if !is_valid {
      return Err(InvalidNodeVersion::with_version(version.to_string()).into());
    }

    Ok(())
  }
}

impl MultiFrom for NodeVersionValidator {
  type Error = NodeVersionValidatorError;

  /// Create NodeVersionValidator from current working directory
  ///
  /// # Example
  ///
  /// ```rust
  /// use doctor_node::validator::NodeVersionValidator;
  /// use doctor_ext::MultiFrom;
  /// use std::path::Path;
  /// use doctor_ext::Validator;
  ///
  /// let validator = NodeVersionValidator::from_cwd(Path::new("fixtures/success"))
  ///   .unwrap();
  /// assert!(validator.validate().is_ok());
  /// ```
  fn from_cwd<P: AsRef<Path>>(cwd: P) -> Result<Self, Self::Error> {
    let file_path = cwd.as_ref().join(FILE_NAME);
    Ok(Self { file_path })
  }

  /// Create NodeVersionValidator from file
  ///
  /// # Example
  ///
  /// ```rust
  /// use doctor_node::validator::NodeVersionValidator;
  /// use doctor_ext::MultiFrom;
  /// use std::path::Path;
  /// use doctor_ext::Validator;
  ///
  /// let validator = NodeVersionValidator::from_file(Path::new("fixtures/success/.node-version"))
  ///   .unwrap();
  /// assert!(validator.validate().is_ok());
  /// ```
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
  fn test_validate_node_version_file_invalid() {
    let node = NodeVersionValidator::from_cwd("./fixtures/invalid").unwrap();
    let result = node.validate();

    assert!(result.is_err());

    assert!(matches!(
      result,
      Err(NodeVersionValidatorError::InvalidNodeVersion(_))
    ));
  }

  #[test]
  fn test_validate_node_version_file_not_found() {
    let node = NodeVersionValidator::from_cwd("./fixtures/not-found").unwrap();
    let result = node.validate();

    assert!(result.is_err());

    if let Err(err) = result {
      assert!(matches!(
        err,
        NodeVersionValidatorError::NodeVersionFileNotFound(_)
      ));
    }
  }

  #[test]
  fn test_validate_node_version_file_empty() {
    let node = NodeVersionValidator::from_cwd("./fixtures/empty_file").unwrap();
    let result = node.validate();

    assert!(result.is_err());

    if let Err(err) = result {
      assert!(matches!(
        err,
        NodeVersionValidatorError::NodeVersionFileIsEmpty(_)
      ));
    }
  }

  #[test]
  fn test_validate_node_version_file_success() {
    let node = NodeVersionValidator::from_cwd("./fixtures/success").unwrap();
    let result = node.validate();

    assert!(result.is_ok());
  }
}
