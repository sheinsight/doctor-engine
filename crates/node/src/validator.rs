use std::path::Path;

use doctor_ext::{Messages, Validator};
use lazy_regex::regex;
use miette::{LabeledSpan, MietteDiagnostic};
use typed_builder::TypedBuilder;

use crate::{
  error::{NodeVersionValidatorError, VersionRequirementNotMet},
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
  with_additional_validation: Option<
    Box<dyn Fn(&NodeVersion) -> Result<Vec<MietteDiagnostic>, NodeVersionValidatorError> + 'a>,
  >,
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
  fn validate(&self) -> Result<Messages, Self::Error> {
    let path = self.config_path.as_ref();

    if !path.exists() {
      let diagnostic =
        MietteDiagnostic::new(r#"The .node-version configuration file was not found."#)
          .with_code("shined_doctor/node_version_file_not_found")
          .with_severity(miette::Severity::Error)
          .with_help(format!(
            r#"Please create a new .node-version file under {config_path}, and correctly declare the version number of the node you are using, which needs to meet the format '^\d+\.\d+\.\d+$'."#,
            config_path = path.display().to_string()
          ));

      return Ok(Messages::builder().diagnostics(vec![diagnostic]).build());
    }

    let node_version = NodeVersion::parse(path)?;

    let mut messages = Messages::builder()
      .source_code(node_version.__raw_source.clone().unwrap_or_default())
      .diagnostics(vec![])
      .build();

    if let Some(version) = &node_version.version {
      let r = regex!(r#"^\d+\.\d+\.\d+$"#);
      if !r.is_match(&version) {
        let diagnostic =
          MietteDiagnostic::new(r#"Only support version numbers that meet '^\d+\.\d+\.\d+$'."#)
            .with_label(LabeledSpan::at(
              0..version.len(),
              r#"Wrong version number format"#,
            ))
            .with_help(r#"Please modify your version number to meet the format '^\d+\.\d+\.\d+$'."#)
            .with_code("shined_doctor/invalid_node_version")
            .with_severity(miette::Severity::Error);

        messages.push(diagnostic);

        return Ok(messages);
      }
    }

    if let Some(raw_source) = &node_version.__raw_source {
      if raw_source.trim().is_empty() {
        let diagnostic = MietteDiagnostic::new(r#"Empty node version"#)
          .with_code("shined_doctor/empty_node_version")
          .with_severity(miette::Severity::Error)
          .with_label(LabeledSpan::at(
            0..raw_source.len(),
            r#"Empty node version"#,
          ))
          .with_help(r#"Please add a node version to your .node-version file."#);

        messages.push(diagnostic);

        return Ok(messages);
      }
    }

    self.validate_additional_validation(&node_version)?;

    Ok(messages)
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

    let res = res.unwrap();

    assert!(res.has_error());
  }

  #[test]
  fn test_validate_node_version_file_not_found() {
    let res = NodeVersionValidator::builder()
      .config_path("./fixtures/.not-found")
      .build()
      .validate();

    let res = res.unwrap();

    assert!(res.has_error());

    res.render();
  }

  #[test]
  fn test_validate_node_version_file_empty() {
    let res = NodeVersionValidator::builder()
      .config_path("./fixtures/.empty")
      .build()
      .validate();

    let res = res.unwrap();

    assert!(res.has_error());

    res.render();
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
