use std::path::Path;

use doctor_ext::Validator;
use lazy_regex::regex;
use typed_builder::TypedBuilder;

use crate::{
  error::{
    EmptyNodeVersionError, InvalidNodeVersion, NodeVersionValidatorError, VersionRequirementNotMet,
  },
  node_version::NodeVersion,
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
    Option<Box<dyn Fn(&NodeVersion) -> Result<(), NodeVersionValidatorError> + 'a>>,
}

impl<'a, P> NodeVersionValidator<'a, P>
where
  P: AsRef<Path>,
{
  fn validate_additional_validation(
    &self,
    node_version: &NodeVersion,
  ) -> Result<(), NodeVersionValidatorError> {
    if let Some(with_additional_validation) = &self.with_additional_validation {
      with_additional_validation(node_version)?;
    }

    let version = node_version.version.clone().unwrap();
    let version = node_semver::Version::parse(&version)?;

    if let Some(with_valid_range) = &self.with_valid_range {
      for range in with_valid_range {
        if let Ok(range) = node_semver::Range::parse(range) {
          if range.satisfies(&version) {
            return Ok(());
          } else {
            return Err(VersionRequirementNotMet::new(node_version))?;
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

    let node_version = NodeVersion::parse(path)?;

    if let Some(version) = &node_version.version {
      let r = regex!(r#"^\d+\.\d+\.\d+$"#);
      if !r.is_match(&version) {
        // let diagnostic =
        //   MietteDiagnostic::new(r#"Only support version numbers that meet '^\d+\.\d+\.\d+$'."#)
        //     .with_label(LabeledSpan::at(
        //       0..version.len(),
        //       r#"Wrong version number format"#,
        //     ))
        //     .with_help(r#"Please modify your version number to meet the format '^\d+\.\d+\.\d+$'."#)
        //     .with_code("shined_doctor/invalid_node_version")
        //     .with_severity(miette::Severity::Error);

        // let report = miette::Report::new(diagnostic)
        //   .with_source_code(node_version.__raw_source.clone().unwrap());

        // println!("{:?}", report);

        return Err(InvalidNodeVersion::new(&node_version))?;
      }
    } else {
      return Err(EmptyNodeVersionError::new(&node_version))?;
    }

    self.validate_additional_validation(&node_version)?;

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

    assert!(matches!(
      res,
      Err(NodeVersionValidatorError::InvalidNodeVersionError(_))
    ));

    if let Err(e) = res {
      println!("--->>> {:?}", miette::Report::new(e));
    }
  }

  #[test]
  fn test_validate_node_version_file_not_found() {
    let res = NodeVersionValidator::builder()
      .config_path("./fixtures/.not-found")
      .build()
      .validate();

    assert!(matches!(
      res,
      Err(NodeVersionValidatorError::NotFoundConfigFileError(_))
    ));

    if let Err(e) = res {
      println!("--->>> {:?}", miette::Report::new(e));
    }
  }

  #[test]
  fn test_validate_node_version_file_empty() {
    let res = NodeVersionValidator::builder()
      .config_path("./fixtures/.empty")
      .build()
      .validate();

    assert!(matches!(
      res,
      Err(NodeVersionValidatorError::EmptyNodeVersionError(_))
    ));

    if let Err(e) = res {
      println!("--->>> {:?}", miette::Report::new(e));
    }
  }

  #[test]
  fn test_validate_node_version_file_success() {
    let res = NodeVersionValidator::builder()
      .config_path("./fixtures/.success")
      .build()
      .validate();

    assert!(res.is_ok());
  }

  // #[test]
  // fn test_validate_node_version_file_additional_validation() {
  //   let path = "./fixtures/.success";
  //   let res = NodeVersionValidator::builder()
  //     .config_path(path.to_string())
  //     .with_additional_validation(Box::new(|version| {
  //       if version.starts_with("v") {
  //         Ok(())
  //       } else {
  //         InvalidErr::builder()
  //           .config_path(path.to_string())
  //           .version(version.to_string())
  //           .build()
  //           .into()
  //       }
  //     }))
  //     .build()
  //     .validate();

  //   assert!(matches!(
  //     res,
  //     Err(NodeVersionValidatorError::VersionRequirementNotMet(_))
  //   ));
  // }

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
