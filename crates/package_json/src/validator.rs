use std::{fs::read_to_string, path::Path};

use doctor_ext::{PathExt, Validator};
use typed_builder::TypedBuilder;

use crate::{
  error::{
    IoErr, MissingNameErr, MissingPackageManagerErr, MissingPrivateErr, NoMatchedPrivateErr,
    PackageJsonValidatorError, ParseErr,
  },
  package_json::PackageJson,
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
    Option<Box<dyn Fn(&PackageJson) -> Result<(), PackageJsonValidatorError> + 'a>>,
}

impl<'a, P> PackageJsonValidator<'a, P>
where
  P: AsRef<Path>,
{
  fn validate_package_manager(
    &self,
    package_json: &PackageJson,
  ) -> Result<(), PackageJsonValidatorError> {
    if let Some(validate_package_manager) = &self.with_validate_package_manager {
      let path = self.config_path.as_ref();

      let Some(_) = &package_json.package_manager else {
        return MissingPackageManagerErr::builder()
          .config_path(path.to_string_owned())
          .build()
          .into();
      };

      match validate_package_manager {
        ValidatePackageManager::Exist => Ok(()),
      }?;
    }

    Ok(())
  }

  fn validate_private(&self, package_json: &PackageJson) -> Result<(), PackageJsonValidatorError> {
    if let Some(validate_private) = &self.with_validate_private {
      let path = self.config_path.as_ref();

      let Some(actual) = package_json.private else {
        return MissingPrivateErr::builder()
          .config_path(path.to_string_owned())
          .build()
          .into();
      };

      match validate_private {
        ValidatePrivate::Exist => Ok(()),
        ValidatePrivate::True if actual == true => Ok(()),
        ValidatePrivate::False if actual == false => Ok(()),
        _ => {
          let expect = match validate_private {
            ValidatePrivate::Exist => actual,
            ValidatePrivate::True => true,
            ValidatePrivate::False => false,
          };
          return NoMatchedPrivateErr::builder()
            .config_path(path.to_string_owned())
            .expect(expect)
            .actual(actual)
            .build()
            .into();
        }
      }?;
    }

    Ok(())
  }

  fn validate_name(&self, package_json: &PackageJson) -> Result<(), PackageJsonValidatorError> {
    if let Some(validate_name) = &self.with_validate_name {
      let path = self.config_path.as_ref();

      let Some(_) = &package_json.name else {
        return MissingNameErr::builder()
          .config_path(path.to_string_owned())
          .build()
          .into();
      };

      match validate_name {
        ValidateName::Exist => Ok(()),
      }?;
    }

    Ok(())
  }

  fn validate_additional_validation(
    &self,
    package_json: &PackageJson,
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

  fn validate(&self) -> Result<(), Self::Error> {
    let path = self.config_path.as_ref();

    let content = read_to_string(&path).map_err(|e| {
      IoErr::builder()
        .path(path.to_string_owned())
        .source(e)
        .build()
        .into()
    })?;

    let package_json = serde_json::from_str::<PackageJson>(&content).map_err(|e| {
      ParseErr::builder()
        .path(path.to_string_owned())
        .source(e)
        .build()
        .into()
    })?;

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
      Err(PackageJsonValidatorError::MissingNameErr(_))
    ))
  }

  #[test]
  fn test_validate_private() {
    let result = PackageJsonValidator::builder()
      .config_path("fixtures/no_private.json")
      .with_validate_private(ValidatePrivate::Exist)
      .build()
      .validate();

    assert!(matches!(
      result,
      Err(PackageJsonValidatorError::MissingPrivateErr(_))
    ))
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
      Err(PackageJsonValidatorError::MissingPackageManagerErr(_))
    ))
  }

  #[test]
  fn test_validate_package_json_additional_validation() {
    let path = "fixtures/no_package_manager.json";
    let result = PackageJsonValidator::builder()
      .config_path(path)
      .with_additional_validation(Box::new(|_package_json| {
        MissingNameErr::builder()
          .config_path(path.to_string())
          .build()
          .into()
      }))
      .build()
      .validate();

    assert!(matches!(
      result,
      Err(PackageJsonValidatorError::MissingNameErr(_))
    ))
  }
}
