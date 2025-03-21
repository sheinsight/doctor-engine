use std::path::{Path, PathBuf};

use config::{Config, File, FileFormat};
use doctor_ext::{MultiFrom, PathExt, Validator};

use crate::error::NpmrcError;

const FILE_NAME: &str = ".npmrc";

/// NpmrcValidator
///
/// # Example
///
/// ```rust
/// use doctor_npmrc::validator::NpmrcValidator;
/// use doctor_ext::MultiFrom;
/// use doctor_ext::Validator;
///
/// let validator = NpmrcValidator::from_cwd("fixtures/success")
///   .unwrap()
///   .with_validate_registry("https://test.npmjs.org/");
/// assert!(validator.validate().is_ok());
/// ```
#[derive(Debug)]
pub struct NpmrcValidator {
  file_path: PathBuf,
  expected_registry: Option<String>,
}

impl NpmrcValidator {
  /// Validate registry
  ///
  /// # Example
  ///
  /// ```rust
  /// use doctor_npmrc::validator::NpmrcValidator;
  /// use doctor_ext::MultiFrom;
  /// use doctor_ext::Validator;
  ///
  /// let validator = NpmrcValidator::from_cwd("fixtures/success")
  ///   .unwrap()
  ///   .with_validate_registry("https://test.npmjs.org/");
  /// assert!(validator.validate().is_ok());
  /// ```
  pub fn with_validate_registry(mut self, expected_registry: &str) -> Self {
    self.expected_registry = Some(expected_registry.to_owned());
    self
  }
}

impl MultiFrom for NpmrcValidator {
  type Error = NpmrcError;

  /// Create NpmrcValidator from file
  ///
  /// # Example
  ///
  /// ```rust
  /// use doctor_npmrc::validator::NpmrcValidator;
  /// use doctor_ext::MultiFrom;
  /// use std::path::Path;
  /// use doctor_ext::Validator;
  ///
  /// let validator = NpmrcValidator::from_file(Path::new("fixtures/success/.npmrc"))
  ///   .unwrap();
  /// assert!(validator.validate().is_ok());
  /// ```
  fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Self::Error> {
    Ok(Self {
      file_path: path.as_ref().to_path_buf(),
      expected_registry: None,
    })
  }

  /// Create NpmrcValidator from current working directory
  ///
  /// # Example
  ///
  /// ```rust
  /// use doctor_npmrc::validator::NpmrcValidator;
  /// use doctor_ext::MultiFrom;
  /// use std::path::Path;
  /// use doctor_ext::Validator;
  ///
  /// let validator = NpmrcValidator::from_cwd(Path::new("fixtures/success"))
  ///   .unwrap();
  /// assert!(validator.validate().is_ok());
  /// ```
  fn from_cwd<P: AsRef<Path>>(cwd: P) -> Result<Self, Self::Error> {
    let file_path = cwd.as_ref().join(FILE_NAME);
    Ok(Self {
      file_path,
      expected_registry: None,
    })
  }
}

impl Validator for NpmrcValidator {
  type Error = NpmrcError;

  /// Validate npmrc
  ///
  /// # Example
  ///
  /// ```rust
  /// use doctor_npmrc::validator::NpmrcValidator;
  /// use doctor_ext::MultiFrom;
  /// use doctor_ext::Validator;
  ///
  /// let validator = NpmrcValidator::from_cwd("fixtures/success")
  ///   .unwrap()
  ///   .with_validate_registry("https://test.npmjs.org/");
  /// assert!(validator.validate().is_ok());
  /// ```
  fn validate(&self) -> Result<(), Self::Error> {
    if !self.file_path.exists() {
      return Err(NpmrcError::NpmrcFileNotFound(
        self.file_path.to_string_owned(),
      ));
    }

    let source = File::from(self.file_path.as_path()).format(FileFormat::Ini);

    let config = Config::builder()
      .add_source(source)
      .build()
      .map_err(|e| NpmrcError::BuildConfigError(e))?;

    let registry = config
      .get::<String>("registry")
      .map_err(|_| NpmrcError::RegistryNotFound)?;

    if let Some(expected) = &self.expected_registry {
      if registry.is_empty() {
        return Err(NpmrcError::RegistryValueIsEmpty);
      }

      if registry != *expected {
        return Err(NpmrcError::RegistryValueMatchedFailed(
          expected.to_owned(),
          registry,
        ));
      }
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_validate_registry() {
    let npmrc = NpmrcValidator::from_cwd("fixtures/success")
      .unwrap()
      .with_validate_registry("https://test.npmjs.org/");
    let result = npmrc.validate();
    assert!(result.is_ok());
  }

  #[test]
  fn test_validate_registry_error() {
    let npmrc = NpmrcValidator::from_cwd("fixtures/not_found_registry")
      .unwrap()
      .with_validate_registry("https://test.npmjs.org/");
    let result = npmrc.validate();
    assert!(result.is_err());
  }

  #[test]
  fn test_validate_registry_error_registry_value_is_empty() {
    let npmrc = NpmrcValidator::from_cwd("fixtures/registry_value_is_empty")
      .unwrap()
      .with_validate_registry("https://test.npmjs.org/");
    let result = npmrc.validate();
    assert!(result.is_err());
  }
}
