use std::path::Path;

use doctor_ext::Validator;
use package_json_parser::PackageJsonParser;
use typed_builder::TypedBuilder;

use crate::error::{
  MissingNameError, MissingPackageManagerError, MissingPrivateError, MustBeTrueError,
  PackageJsonValidatorError,
};

#[derive(Debug)]
pub enum ValidateName {
  Exist,
  // Value(String),
}

#[derive(Debug)]
pub enum ValidatePackageManager {
  Exist,
  // Npm,
  // Pnpm,
  // Yarn,
}

#[derive(Debug)]
pub enum ValidatePrivate {
  Exist,
  True,
  False,
}

/// validate package.json file
///
/// # Example
///
/// ```rust
/// use doctor_package_json::validator::PackageJsonValidator;
/// use std::path::Path;
/// use doctor_ext::Validator;
///
/// let validator = PackageJsonValidator::builder()
///   .config_path("./fixtures/package.json")
///   .build();
///
/// let result = validator.validate();
///
/// assert!(result.is_ok());
/// ```
///
/// # Validate name
///
/// ```rust
/// use doctor_package_json::validator::{PackageJsonValidator,ValidateName};
/// use doctor_package_json::error::PackageJsonValidatorError;
/// use std::path::Path;
/// use doctor_ext::Validator;
///
/// let validator = PackageJsonValidator::builder()
///   .config_path("./fixtures/no_name.json")
///   .with_validate_name(ValidateName::Exist)
///   .build();
///
/// let result = validator.validate();
///
/// assert!(matches!(result, Err(PackageJsonValidatorError::MissingNameError(_))));
/// ```
///
/// # Validate private
///
/// ```rust
/// use doctor_package_json::validator::{PackageJsonValidator,ValidatePrivate};
/// use doctor_package_json::error::PackageJsonValidatorError;
/// use std::path::Path;
/// use doctor_ext::Validator;
///
/// let validator = PackageJsonValidator::builder()
///   .config_path("./fixtures/no_private.json")
///   .with_validate_private(ValidatePrivate::Exist)
///   .build();
///
/// let result = validator.validate();
///
/// assert!(matches!(result, Err(PackageJsonValidatorError::MissingPrivateError(_))));
/// ```
///
/// # Validate package manager
///
/// ```rust
/// use doctor_package_json::validator::{PackageJsonValidator,ValidatePackageManager};
/// use doctor_package_json::error::PackageJsonValidatorError;
/// use std::path::Path;
/// use doctor_ext::Validator;
///
/// let validator = PackageJsonValidator::builder()
///   .config_path("./fixtures/no_package_manager.json")
///   .with_validate_package_manager(ValidatePackageManager::Exist)
///   .build();
///
/// let result = validator.validate();
///
/// assert!(matches!(result, Err(PackageJsonValidatorError::MissingPackageManagerError(_))));
/// ```
#[derive(TypedBuilder)]
pub struct PackageJsonValidator<'a, P>
where
  P: AsRef<Path>,
{
  config_path: P,

  #[builder(default = None, setter(strip_option))]
  with_validate_name: Option<ValidateName>,

  #[builder(default = None, setter(strip_option))]
  with_validate_private: Option<ValidatePrivate>,

  #[builder(default = None, setter(strip_option))]
  with_validate_package_manager: Option<ValidatePackageManager>,

  #[builder(default = None, setter(strip_option))]
  with_additional_validation:
    Option<Box<dyn Fn(&PackageJsonParser) -> Result<(), PackageJsonValidatorError> + 'a>>,
}

impl<'a, P> PackageJsonValidator<'a, P>
where
  P: AsRef<Path>,
{
  fn validate_package_manager(
    &self,
    package_json: &PackageJsonParser,
  ) -> Result<(), PackageJsonValidatorError> {
    if let Some(validate_package_manager) = &self.with_validate_package_manager {
      let Some(_) = &package_json.package_manager else {
        return MissingPackageManagerError::new(package_json);
      };

      match validate_package_manager {
        ValidatePackageManager::Exist => (),
      };
    }

    Ok(())
  }

  fn validate_private(
    &self,
    package_json: &PackageJsonParser,
  ) -> Result<(), PackageJsonValidatorError> {
    if let Some(validate_private) = &self.with_validate_private {
      let Some(actual) = package_json.private else {
        return MissingPrivateError::new(package_json);
      };

      match validate_private {
        ValidatePrivate::Exist => (),
        ValidatePrivate::True if actual == true => (),
        ValidatePrivate::False if actual == false => (),
        _ => {
          return MustBeTrueError::new(package_json);
        }
      };
    }

    Ok(())
  }

  fn validate_name(
    &self,
    package_json: &PackageJsonParser,
  ) -> Result<(), PackageJsonValidatorError> {
    if let Some(validate_name) = &self.with_validate_name {
      let Some(_) = &package_json.name else {
        return MissingNameError::new(package_json);
      };

      match validate_name {
        ValidateName::Exist => (),
      };
    }

    Ok(())
  }

  fn validate_additional_validation(
    &self,
    package_json: &PackageJsonParser,
  ) -> Result<(), PackageJsonValidatorError> {
    if let Some(with_additional_validation) = &self.with_additional_validation {
      with_additional_validation(package_json)?;
    }

    Ok(())
  }
}
impl<'a, P> Validator for PackageJsonValidator<'a, P>
where
  P: AsRef<Path>,
{
  type Error = PackageJsonValidatorError;

  /// validate package.json file
  ///
  /// # Example
  ///
  /// ```rust
  /// use doctor_package_json::validator::PackageJsonValidator;
  /// use std::path::Path;
  /// use doctor_ext::Validator;
  ///
  /// let validator = PackageJsonValidator::builder()
  ///   .config_path("./fixtures/package.json")
  ///   .build();
  ///
  /// let result = validator.validate();
  ///
  /// assert!(result.is_ok());
  /// ```
  fn validate(&self) -> Result<(), Self::Error> {
    let path = self.config_path.as_ref();

    // let package_json = package_json_parser::PackageJsonParser::parse(path).map_err(|e| {
    //   ParseErr::builder()
    //     .path(path.to_string_owned())
    //     .source(e)
    //     .build()
    //     .into()
    // })?;

    let package_json = package_json_parser::PackageJsonParser::parse(path).unwrap();

    self.validate_name(&package_json)?;

    self.validate_private(&package_json)?;

    self.validate_package_manager(&package_json)?;

    self.validate_additional_validation(&package_json)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_validate_name() {
    let result = PackageJsonValidator::builder()
      .config_path("fixtures/no_name.json")
      .with_validate_name(ValidateName::Exist)
      .build()
      .validate();

    assert!(matches!(
      result,
      Err(PackageJsonValidatorError::MissingNameError(_))
    ));

    if let Err(e) = result {
      eprintln!("{:?}", miette::Report::new(e));
    }
  }

  #[test]
  fn test_validate_private() {
    let result = PackageJsonValidator::builder()
      .config_path("fixtures/no_private.json")
      .with_validate_private(ValidatePrivate::True)
      .build()
      .validate();

    assert!(matches!(
      result,
      Err(PackageJsonValidatorError::MissingPrivateError(_))
    ));

    if let Err(e) = result {
      eprintln!("{:?}", miette::Report::new(e));
    }
  }

  #[test]
  fn test_validate_package_manager() {
    let result = PackageJsonValidator::builder()
      .config_path("fixtures/no_package_manager.json")
      .with_validate_package_manager(ValidatePackageManager::Exist)
      .build()
      .validate();

    assert!(matches!(
      result,
      Err(PackageJsonValidatorError::MissingPackageManagerError(_))
    ));

    if let Err(e) = result {
      eprintln!("{:?}", miette::Report::new(e));
    }
  }

  #[test]
  fn test_validate_package_json_additional_validation() {
    let path = "fixtures/no_package_manager.json";
    let result = PackageJsonValidator::builder()
      .config_path(path)
      .with_additional_validation(Box::new(|package_json| {
        MissingPackageManagerError::new(package_json)
      }))
      .build()
      .validate();

    assert!(matches!(
      result,
      Err(PackageJsonValidatorError::MissingPackageManagerError(_))
    ));

    if let Err(e) = result {
      eprintln!("{:?}", miette::Report::new(e));
    }
  }

  #[test]
  fn should_fail_when_private_is_false() {
    let result = PackageJsonValidator::builder()
      .config_path("fixtures/private_false.json")
      .with_validate_private(ValidatePrivate::True)
      .build()
      .validate();

    assert!(matches!(
      result,
      Err(PackageJsonValidatorError::MustBeTrueError(_))
    ));

    if let Err(e) = result {
      eprintln!("{:?}", miette::Report::new(e));
    }
  }
}
