use std::{borrow::Cow, path::Path};

use doctor_core::{
  Messages, ValidatorError,
  traits::{PathExt, Validator},
};
use lazy_regex::regex;
use miette::MietteDiagnostic;
use typed_builder::TypedBuilder;

use crate::{
  diagnostics::{
    EmptyNodeVersionDiagnostic, InvalidVersionFormatDiagnostic, InvalidVersionRangeDiagnostic,
    NodeVersionFileNotFoundDiagnostic,
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
/// use doctor_core::{ ValidatorError, traits::Validator};
///
/// let validator = NodeVersionValidator::builder()
///   .config_path("./fixtures/.success")
///   .with_valid_range(vec!["^18.0.0".to_string()])
///   .build();
///
/// let result = validator.validate();
///
/// assert!(result.is_ok());
/// ```
#[derive(TypedBuilder)]
pub struct NodeVersionValidator<P, T>
where
  P: AsRef<Path>,
  T: Into<Cow<'static, str>> + AsRef<str>,
{
  config_path: P,

  #[builder(default = None, setter(strip_option))]
  with_valid_range: Option<Vec<T>>,
}

impl<P, T> NodeVersionValidator<P, T>
where
  P: AsRef<Path>,
  T: Into<Cow<'static, str>> + AsRef<str>,
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
        let range = node_semver::Range::parse(range_str.as_ref())?;
        ranges.push(range);
      }

      let is_in_range = ranges.iter().any(|range| range.satisfies(&version));

      if is_in_range {
        return Ok(diagnostics);
      }

      let diagnostic =
        InvalidVersionRangeDiagnostic::at(0..len, ranges.iter().map(|r| r.to_string()).collect());

      diagnostics.push(diagnostic);
    }

    Ok(diagnostics)
  }
}

impl<P, T> Validator for NodeVersionValidator<P, T>
where
  P: AsRef<Path>,
  T: Into<Cow<'static, str>> + AsRef<str>,
{
  /// validate node version file
  ///
  /// # Example
  ///
  /// ```rust
  /// use doctor_node::validator::NodeVersionValidator;
  /// use std::path::Path;
  /// use doctor_core::{ ValidatorError, traits::Validator};
  ///
  /// let validator = NodeVersionValidator::builder()
  ///   .config_path("./fixtures/.success")
  ///   .with_valid_range(vec!["^18.0.0".to_string()])
  ///   .build();
  ///
  /// let result = validator.validate();
  ///
  /// assert!(result.is_ok());
  /// ```
  fn validate(&self) -> Result<Vec<Messages>, ValidatorError> {
    let path = self.config_path.as_ref();

    if !path.exists() {
      return Ok(vec![
        Messages::builder()
          .diagnostics(vec![NodeVersionFileNotFoundDiagnostic::at(path)])
          .build(),
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
        let diagnostic = InvalidVersionFormatDiagnostic::at(0..version.len());

        messages.push(diagnostic);

        return Ok(vec![messages]);
      }
    }

    if let Some(raw_source) = &node_version.__raw_source {
      if raw_source.trim().is_empty() {
        let diagnostic = EmptyNodeVersionDiagnostic::at(0..raw_source.len());

        messages.push(diagnostic);

        return Ok(vec![messages]);
      }
    }

    let diagnostics = self.validate_valid_range(&node_version)?;
    messages.diagnostics.extend(diagnostics);

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
      .with_valid_range(vec!["^18.0.0"])
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
      .with_valid_range(vec!["^18.0.0"])
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
      .with_valid_range(vec!["^18.0.0"])
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
      .with_valid_range(vec!["^18.0.0"])
      .build()
      .validate();

    assert!(res.is_ok());
  }

  #[test]
  fn test_validate_node_version_file_valid_range() {
    let res = NodeVersionValidator::builder()
      .config_path("./fixtures/.range")
      .with_valid_range(vec!["^18.0.0", "^2.0.0"])
      .build()
      .validate();

    assert!(res.is_ok());
  }

  #[test]
  fn test_validate_node_version_file_valid_range_error() {
    let res = NodeVersionValidator::builder()
      .config_path("./fixtures/.range")
      .with_valid_range(vec!["^14.0.0", "^15.0.0"])
      .build()
      .validate();

    let res = res.unwrap();

    for msg in res {
      assert!(msg.has_error());
      msg.render();
    }
  }
}
