use std::{fs::read_to_string, path::Path};

use doctor_ext::{PathExt, Validator};
use lazy_regex::regex;
use typed_builder::TypedBuilder;

use crate::error::{
  InvalidErr, NodeVersionValidatorError, NotFoundErr, UNknowErr, VersionRequirementNotMet,
};

/// validate node version file
///
/// # Example
///
/// ```rust
/// use doctor_node::validator::NodeVersionValidator;
/// use std::path::Path;
/// use doctor_ext::Validator;
///
/// let validator = NodeVersionValidator::builder()
///   .config_path("./fixtures/.success")
///   .build();
///
/// let result = validator.validate();
///
/// assert!(result.is_ok());
/// ```
#[derive(TypedBuilder)]
pub struct NodeVersionValidator<'a, P>
where
  P: AsRef<Path>,
{
  config_path: P,

  #[builder(default = None, setter(strip_option))]
  with_valid_range: Option<Vec<String>>,

  #[builder(default = None, setter(strip_option))]
  with_additional_validation:
    Option<Box<dyn Fn(String) -> Result<(), NodeVersionValidatorError> + 'a>>,
}

impl<'a, P> NodeVersionValidator<'a, P>
where
  P: AsRef<Path>,
{
  fn validate_additional_validation(&self, version: &str) -> Result<(), NodeVersionValidatorError> {
    if let Some(with_additional_validation) = &self.with_additional_validation {
      with_additional_validation(version.to_string())?;
    }

    let Ok(version) = node_semver::Version::parse(version) else {
      return InvalidErr::builder()
        .config_path(self.config_path.as_ref().to_string_owned())
        .version(version.to_string())
        .build()
        .into();
    };

    if let Some(with_valid_range) = &self.with_valid_range {
      for range in with_valid_range {
        if let Ok(range) = node_semver::Range::parse(range) {
          if range.satisfies(&version) {
            return Ok(());
          } else {
            return VersionRequirementNotMet::builder()
              .config_path(self.config_path.as_ref().to_string_owned())
              .version(version.to_string())
              .build()
              .into();
          }
        }
      }
    }

    Ok(())
  }
}

impl<'a, P> Validator for NodeVersionValidator<'a, P>
where
  P: AsRef<Path>,
{
  type Error = NodeVersionValidatorError;

  /// validate node version file
  ///
  /// # Example
  ///
  /// ```rust
  /// use doctor_node::validator::NodeVersionValidator;
  /// use std::path::Path;
  /// use doctor_ext::Validator;
  ///
  /// let validator = NodeVersionValidator::builder()
  ///   .config_path("./fixtures/.success")
  ///   .build();
  ///
  /// let result = validator.validate();
  ///
  /// assert!(result.is_ok());
  /// ```
  fn validate(&self) -> Result<(), Self::Error> {
    let path = self.config_path.as_ref();

    if !path.exists() {
      return NotFoundErr::builder()
        .config_path(path.to_string_owned())
        .build()
        .into();
    }

    let version =
      read_to_string(path).map_err(|e| UNknowErr::builder().source(Box::new(e)).build().into())?;

    let version = version.trim();

    let r = regex!(r#"^\d+\.\d+\.\d+$"#);

    if !r.is_match(&version) {
      return InvalidErr::builder()
        .config_path(path.to_string_owned())
        .version(version.to_string())
        .build()
        .into();
    }

    self.validate_additional_validation(&version)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_validate_node_version_file_invalid() {
    let res = NodeVersionValidator::builder()
      .config_path("./fixtures/.invalid")
      .build()
      .validate();

    assert!(matches!(res, Err(NodeVersionValidatorError::InvalidErr(_))));
  }

  #[test]
  fn test_validate_node_version_file_not_found() {
    let res = NodeVersionValidator::builder()
      .config_path("./fixtures/.not-found")
      .build()
      .validate();

    assert!(matches!(
      res,
      Err(NodeVersionValidatorError::NotFoundErr(_))
    ));
  }

  #[test]
  fn test_validate_node_version_file_empty() {
    let res = NodeVersionValidator::builder()
      .config_path("./fixtures/.empty")
      .build()
      .validate();

    assert!(matches!(res, Err(NodeVersionValidatorError::InvalidErr(_))));
  }

  #[test]
  fn test_validate_node_version_file_success() {
    let res = NodeVersionValidator::builder()
      .config_path("./fixtures/.success")
      .build()
      .validate();

    assert!(res.is_ok());
  }

  #[test]
  fn test_validate_node_version_file_additional_validation() {
    let path = "./fixtures/.success";
    let res = NodeVersionValidator::builder()
      .config_path(path.to_string())
      .with_additional_validation(Box::new(|version| {
        if version.starts_with("v") {
          Ok(())
        } else {
          InvalidErr::builder()
            .config_path(path.to_string())
            .version(version.to_string())
            .build()
            .into()
        }
      }))
      .build()
      .validate();

    assert!(matches!(res, Err(NodeVersionValidatorError::InvalidErr(_))));
  }

  #[test]
  fn test_validate_node_version_file_valid_range() {
    let res = NodeVersionValidator::builder()
      .config_path("./fixtures/.range")
      .with_valid_range(vec!["^18.0.0".to_string(), "^2.0.0".to_string()])
      .build()
      .validate();

    assert!(res.is_ok());
  }
}
