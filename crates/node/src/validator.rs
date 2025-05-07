use std::path::Path;

use doctor_ext::{Messages, PathExt, Validator, ValidatorError};
use lazy_regex::regex;
use miette::{LabeledSpan, MietteDiagnostic};
use typed_builder::TypedBuilder;

use crate::node_version::NodeVersion;

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
pub struct NodeVersionValidator<'a, P, S = String>
where
  P: AsRef<Path>,
  S: Into<String> + AsRef<str>,
{
  config_path: P,

  #[builder(default = None, setter(strip_option))]
  with_valid_range: Option<Vec<S>>,

  #[builder(default = None, setter(strip_option))]
  with_additional_validation:
    Option<Box<dyn Fn(&NodeVersion) -> Result<Vec<MietteDiagnostic>, ValidatorError> + 'a>>,
}

impl<'a, P, S> NodeVersionValidator<'a, P, S>
where
  P: AsRef<Path>,
  S: Into<String> + AsRef<str>,
{
  fn validate_valid_range(
    &self,
    node_version: &NodeVersion,
  ) -> Result<Vec<MietteDiagnostic>, ValidatorError> {
    let mut diagnostics = vec![];

    if let Some(with_valid_range) = &self.with_valid_range {
      let version = node_version.version.clone().unwrap();
      let len = node_version.__raw_source.as_ref().map_or(0, |s| s.len());
      let version = node_semver::Version::parse(&version)?;

      let mut ranges = vec![];

      for range_str in with_valid_range {
        let range = node_semver::Range::parse(range_str)?;
        ranges.push(range);
      }

      let is_in_range = ranges.iter().any(|range| range.satisfies(&version));

      if is_in_range {
        return Ok(diagnostics);
      }

      let diagnostic = MietteDiagnostic::new(r#"The node version is not in the valid range."#)
        .with_code("shined_doctor/node_version_not_in_valid_range")
        .with_label(LabeledSpan::at(
          0..len,
          format!(
            r#"Wrong version number format , Only support version range in {}"#,
            ranges
              .iter()
              .map(|r| r.to_string())
              .collect::<Vec<String>>()
              .join(", ")
          ),
        ))
        .with_severity(miette::Severity::Error);

      diagnostics.push(diagnostic);
    }

    Ok(diagnostics)
  }

  fn validate_additional_validation(
    &self,
    node_version: &NodeVersion,
  ) -> Result<Vec<MietteDiagnostic>, ValidatorError> {
    let mut diagnostics = vec![];

    if let Some(with_additional_validation) = &self.with_additional_validation {
      diagnostics.extend(with_additional_validation(node_version)?);
    }

    Ok(diagnostics)
  }
}

impl<'a, P, S> Validator for NodeVersionValidator<'a, P, S>
where
  P: AsRef<Path>,
  S: Into<String> + AsRef<str>,
{
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
  fn validate(&self) -> Result<Vec<Messages>, ValidatorError> {
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

      return Ok(vec![
        Messages::builder().diagnostics(vec![diagnostic]).build(),
      ]);
    }

    let node_version = NodeVersion::parse(path)?;

    let mut messages = Messages::builder()
      .source_code(node_version.__raw_source.clone().unwrap_or_default())
      .source_path(path.to_string_owned())
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

        return Ok(vec![messages]);
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

        return Ok(vec![messages]);
      }
    }

    let diagnostics = self.validate_valid_range(&node_version)?;
    messages.diagnostics.extend(diagnostics);

    self.validate_additional_validation(&node_version)?;

    Ok(vec![messages])
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_validate_node_version_file_invalid() {
    let res = NodeVersionValidator::builder()
      .config_path("./fixtures/.invalid")
      .with_valid_range(vec!["^18.0.0".to_string()])
      .build()
      .validate();

    let res = res.unwrap();

    for msg in res {
      assert!(msg.has_error());
      msg.render();
    }
  }

  #[test]
  fn test_validate_node_version_file_not_found() {
    let res = NodeVersionValidator::builder()
      .config_path("./fixtures/.not-found")
      .with_valid_range(vec!["^18.0.0".to_string()])
      .build()
      .validate();

    let res = res.unwrap();

    for msg in res {
      assert!(msg.has_error());
      msg.render();
    }
  }

  #[test]
  fn test_validate_node_version_file_empty() {
    let res = NodeVersionValidator::builder()
      .config_path("./fixtures/.empty")
      .with_valid_range(vec!["^18.0.0".to_string()])
      .build()
      .validate();

    let res = res.unwrap();

    for msg in res {
      assert!(msg.has_error());
      msg.render();
    }
  }

  #[test]
  fn test_validate_node_version_file_success() {
    let res = NodeVersionValidator::builder()
      .config_path("./fixtures/.success")
      .with_valid_range(vec!["^18.0.0".to_string()])
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

  #[test]
  fn test_validate_node_version_file_valid_range_error() {
    let res = NodeVersionValidator::builder()
      .config_path("./fixtures/.range")
      .with_valid_range(vec!["^14.0.0".to_string(), "^15.0.0".to_string()])
      .build()
      .validate();

    let res = res.unwrap();

    for msg in res {
      assert!(msg.has_error());
      msg.render();
    }
  }
}
