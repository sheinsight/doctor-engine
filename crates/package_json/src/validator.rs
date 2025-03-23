use std::path::{Path, PathBuf};

use doctor_ext::{MultiFrom, PathExt, Validator};

use crate::{error::PackageJsonValidatorError, package_json::PackageJson};

const FILE_NAME: &str = "package.json";

/// PackageJsonValidator
///
/// # Example
///
/// ```rust
/// use doctor_package_json::validator::PackageJsonValidator;
/// use doctor_ext::MultiFrom;
/// use doctor_ext::Validator;
///
/// let validator = PackageJsonValidator::from_cwd("fixtures/no_name")
///   .unwrap()
///   .with_validate_name();
/// assert!(validator.validate().is_err());
///
/// let validator = PackageJsonValidator::from_cwd("fixtures/no_private")
///   .unwrap()
///   .with_validate_private_value(true);
/// assert!(validator.validate().is_err());
///
/// let validator = PackageJsonValidator::from_cwd("fixtures/no_package_manager")
///   .unwrap()
///   .with_validate_package_manager();
/// assert!(validator.validate().is_err());
/// ```
#[derive(Debug)]
pub struct PackageJsonValidator {
  file_path: PathBuf,
  package_json: PackageJson,
  with_validate_name: bool,
  with_validate_package_manager: bool,
  with_validate_private_value: Option<bool>,
}

impl PackageJsonValidator {
  /// Validate name
  ///
  /// # Example
  ///
  /// ```rust
  /// use doctor_package_json::validator::PackageJsonValidator;
  /// use doctor_ext::MultiFrom;
  /// use doctor_ext::Validator;
  ///
  /// let validator = PackageJsonValidator::from_cwd("fixtures/no_name")
  ///   .unwrap()
  ///   .with_validate_name();
  /// assert!(validator.validate().is_err());
  /// ```
  pub fn with_validate_name(mut self) -> Self {
    self.with_validate_name = true;
    self
  }

  /// Validate private value
  ///
  /// # Example
  ///
  /// ```rust
  /// use doctor_package_json::validator::PackageJsonValidator;
  /// use doctor_ext::MultiFrom;
  /// use doctor_ext::Validator;
  ///
  /// let validator = PackageJsonValidator::from_cwd("fixtures/no_private")
  ///   .unwrap()
  ///   .with_validate_private_value(true);
  /// assert!(validator.validate().is_err());
  /// ```
  pub fn with_validate_private_value(mut self, expect_value: bool) -> Self {
    self.with_validate_private_value = Some(expect_value);
    self
  }

  /// Validate package manager
  ///
  /// # Example
  ///
  /// ```rust
  /// use doctor_package_json::validator::PackageJsonValidator;
  /// use doctor_ext::MultiFrom;
  /// use doctor_ext::Validator;
  ///
  /// let validator = PackageJsonValidator::from_cwd("fixtures/no_package_manager")
  ///   .unwrap()
  ///   .with_validate_package_manager();
  /// assert!(validator.validate().is_err());
  /// ```
  pub fn with_validate_package_manager(mut self) -> Self {
    self.with_validate_package_manager = true;
    self
  }
}

impl Validator for PackageJsonValidator {
  type Error = PackageJsonValidatorError;

  /// Validate package.json
  ///
  /// # Example
  ///
  /// ```rust
  /// use doctor_package_json::validator::PackageJsonValidator;
  /// use doctor_ext::MultiFrom;
  /// use doctor_ext::Validator;
  ///
  /// let validator = PackageJsonValidator::from_cwd("fixtures/no_name")
  ///   .unwrap()
  ///   .with_validate_name();
  /// assert!(validator.validate().is_err());
  fn validate(&self) -> Result<(), Self::Error> {
    if self.with_validate_name {
      if self.package_json.name.is_none() {
        return Err(PackageJsonValidatorError::NoNameError(
          self.file_path.to_string_owned(),
        ));
      }
    }

    if self.with_validate_package_manager {
      if self.package_json.package_manager.is_none() {
        return Err(PackageJsonValidatorError::NoPackageManagerError(
          self.file_path.to_string_owned(),
        ));
      }
    }

    if let Some(expect_value) = self.with_validate_private_value {
      if let Some(actual_value) = self.package_json.private {
        if actual_value != expect_value {
          return Err(PackageJsonValidatorError::NoMatchedPrivateError {
            path: self.file_path.to_string_owned(),
            expect_value,
            actual_value,
          });
        }
      } else {
        return Err(PackageJsonValidatorError::NoPrivateError(
          self.file_path.to_string_owned(),
        ));
      }
    }

    Ok(())
  }
}

impl MultiFrom for PackageJsonValidator {
  type Error = PackageJsonValidatorError;

  /// Create PackageJsonValidator from file
  ///
  /// # Example
  ///
  /// ```rust
  /// use doctor_package_json::validator::PackageJsonValidator;
  /// use doctor_ext::MultiFrom;
  /// use doctor_ext::Validator;
  /// use std::path::Path;
  ///
  /// let validator = PackageJsonValidator::from_file(Path::new("fixtures/no_name/package.json"))
  ///   .unwrap();
  /// assert!(!validator.validate().is_err());
  /// ```
  fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Self::Error> {
    let package_json = PackageJson::from_file(&path)?;
    let file_path = path.as_ref().to_path_buf();

    Ok(Self {
      file_path,
      package_json,
      with_validate_name: false,
      with_validate_package_manager: false,
      with_validate_private_value: None,
    })
  }

  /// Create PackageJsonValidator from current working directory
  ///
  /// # Example
  ///
  /// ```rust
  /// use doctor_package_json::validator::PackageJsonValidator;
  /// use doctor_ext::MultiFrom;
  /// use doctor_ext::Validator;
  /// use std::path::Path;
  ///
  /// let validator = PackageJsonValidator::from_cwd(Path::new("fixtures/no_name"))
  ///   .unwrap();
  /// assert!(!validator.validate().is_err());
  /// ```
  fn from_cwd<P: AsRef<Path>>(cwd: P) -> Result<Self, Self::Error> {
    let path = cwd.as_ref().join(FILE_NAME);
    let package_json = PackageJson::from_file(&path)?;
    let file_path = path.to_path_buf();

    Ok(Self {
      file_path,
      package_json,
      with_validate_name: false,
      with_validate_package_manager: false,
      with_validate_private_value: None,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_validate_name() {
    let validator = PackageJsonValidator::from_cwd("fixtures/no_name")
      .unwrap()
      .with_validate_name();
    let result = validator.validate();
    assert!(result.is_err());
  }

  #[test]
  fn test_validate_private() {
    let validator = PackageJsonValidator::from_cwd("fixtures/no_private")
      .unwrap()
      .with_validate_private_value(true);
    let result = validator.validate();
    assert!(result.is_err());
  }

  #[test]
  fn test_validate_package_manager() {
    let validator = PackageJsonValidator::from_cwd("fixtures/no_package_manager")
      .unwrap()
      .with_validate_package_manager();
    let result = validator.validate();
    assert!(result.is_err());
  }
}
