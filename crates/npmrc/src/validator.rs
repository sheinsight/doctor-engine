use std::{borrow::Cow, path::Path};

use doctor_core::{
  Messages, ValidatorError,
  traits::{PathExt, Validator},
};

use miette::MietteDiagnostic;
use typed_builder::TypedBuilder;

use crate::{diagnostics::DiagnosticFactory, npmrc_config::NpmrcConfig};

/// NpmrcValidator is a validator for npmrc file
///
/// # Example
///
/// ```rust
/// use doctor_npmrc::validator::NpmrcValidator;
/// use doctor_core::{ ValidatorError, traits::Validator};
///
/// let validator = NpmrcValidator::builder()
///   .config_path("./fixtures/.npmrc")
///   .with_registry_url(vec!["https://test.npmjs.org/"])
///   .build();
/// assert!(validator.validate().is_ok());
/// ```
#[derive(TypedBuilder)]
pub struct NpmrcValidator<P, S>
where
  P: AsRef<Path>,
  S: Into<Cow<'static, str>> + AsRef<str>,
{
  config_path: P,

  #[builder(default = None,setter(strip_option))]
  with_registry_url: Option<Vec<S>>,
}

impl<P, S> NpmrcValidator<P, S>
where
  P: AsRef<Path>,
  S: Into<Cow<'static, str>> + AsRef<str>,
{
  fn validate_registry(
    &self,
    config: &NpmrcConfig,
  ) -> Result<Vec<MietteDiagnostic>, ValidatorError> {
    let mut diagnostics = vec![];

    if let Some(validate_registry) = &self.with_registry_url {
      let validate_registry = validate_registry
        .into_iter()
        .map(|item| item.as_ref().to_string())
        .collect::<Vec<String>>();

      if let Some(registry) = &config.registry {
        if !validate_registry
          .iter()
          .any(|item| item.trim_end_matches("/") == registry.trim_end_matches("/"))
        {
          let (offset, length) = self
            .find_registry_position(&config.__raw_source)
            .unwrap_or((0, 0));

          let diagnostic =
            DiagnosticFactory::at_invalid_registry(offset..offset + length, &validate_registry);

          diagnostics.push(diagnostic);

          return Ok(diagnostics);
        }
      } else {
        let diagnostic = DiagnosticFactory::at_missing_registry(0..config.__raw_source.len());

        diagnostics.push(diagnostic);

        return Ok(diagnostics);
      }
    }
    Ok(diagnostics)
  }

  fn find_registry_position(&self, content: &str) -> Option<(usize, usize)> {
    for line in content.lines() {
      if let Some(key_pos) = line.find("registry=") {
        let equals_pos = key_pos + "registry".len();
        let value_start = equals_pos + 1;
        let value_length = line.len() - value_start;

        let line_offset = content.find(line).unwrap();
        let absolute_offset = line_offset + value_start;

        return Some((absolute_offset, value_length));
      }
    }
    None
  }
}

impl<P, S> Validator for NpmrcValidator<P, S>
where
  P: AsRef<Path>,
  S: Into<Cow<'static, str>> + AsRef<str>,
{
  /// Validate npmrc file
  ///
  /// # Example
  ///
  /// ```rust
  /// use doctor_npmrc::validator::NpmrcValidator;
  /// use doctor_core::{ ValidatorError, traits::Validator};
  ///
  /// let validator = NpmrcValidator::builder()
  ///   .config_path("./fixtures/.npmrc")
  ///   .with_registry_url(vec!["https://test.npmjs.org/"])
  ///   .build();
  /// assert!(validator.validate().is_ok());
  /// ```
  fn validate(&self) -> Result<Vec<Messages>, ValidatorError> {
    let path = self.config_path.as_ref();

    if !path.exists() {
      return Ok(vec![
        Messages::builder()
          .source_code(String::new())
          .source_path(path.to_string_owned())
          .diagnostics(vec![DiagnosticFactory::at_config_file_not_found(path)])
          .build(),
      ]);
    }
    let config = NpmrcConfig::parse(self.config_path.as_ref())?;

    let mut messages = Messages::builder()
      .source_code(config.__raw_source.clone())
      .source_path(self.config_path.as_ref().to_string_owned())
      .diagnostics(vec![])
      .build();

    let diagnostics = self.validate_registry(&config)?;

    messages.diagnostics.extend(diagnostics.into_iter());

    Ok(vec![messages])
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn should_return_ok_diagnostic() {
    let result = NpmrcValidator::builder()
      .config_path("fixtures/.npmrc")
      .with_registry_url(vec!["https://test.npmjs.org/"])
      .build()
      .validate();

    assert!(result.is_ok());
  }

  #[test]
  fn should_return_invalid_registry_diagnostic() {
    let result = NpmrcValidator::builder()
      .config_path("fixtures/.npmrc")
      .with_registry_url(vec!["https://test2.npmjs.org"])
      .build()
      .validate();

    let result = result.unwrap();

    for msg in result {
      assert!(msg.has_error());
      msg.render();
      assert_eq!(msg.diagnostics.len(), 1);
      assert_eq!(
        msg.diagnostics[0].code,
        Some("shined(npmrc:invalid-registry)".into())
      );
    }
  }

  #[test]
  fn should_return_missing_registry_diagnostic() {
    let result = NpmrcValidator::builder()
      .config_path("fixtures/.undef_registry")
      .with_registry_url(vec!["https://test.npmjs.org/"])
      .build()
      .validate();

    let result = result.unwrap();

    for msg in result {
      assert!(msg.has_error());
      msg.render();
      assert_eq!(msg.diagnostics.len(), 1);
      assert_eq!(
        msg.diagnostics[0].code,
        Some("shined(npmrc:missing-registry)".into())
      );
    }
  }

  #[test]
  fn should_return_invalid_registry_diagnostic_when_registry_is_empty() {
    let result = NpmrcValidator::builder()
      .config_path("fixtures/.empty_registry")
      .with_registry_url(vec!["https://test.npmjs.org/"])
      .build()
      .validate();

    let result = result.unwrap();

    for msg in result {
      assert!(msg.has_error());
      msg.render();
      assert_eq!(msg.diagnostics.len(), 1);
      assert_eq!(
        msg.diagnostics[0].code,
        Some("shined(npmrc:invalid-registry)".into())
      );
    }
  }

  #[test]
  fn should_return_ok_diagnostic_when_has_end_slash() {
    let result = NpmrcValidator::builder()
      .config_path("fixtures/.has_end")
      .with_registry_url(vec!["https://test.npmjs.org"])
      .build()
      .validate();

    let result = result.unwrap();

    for msg in result {
      assert!(!msg.has_error());
      msg.render();
    }
  }

  #[test]
  fn should_return_config_file_not_found_diagnostic() {
    let result = NpmrcValidator::builder()
      .config_path("fixtures/.not_found")
      .with_registry_url(vec!["https://test.npmjs.org/"])
      .build()
      .validate();

    let result = result.unwrap();

    for msg in result {
      assert!(msg.has_error());
      msg.render();
      assert_eq!(msg.diagnostics.len(), 1);
      assert_eq!(
        msg.diagnostics[0].code,
        Some("shined(npmrc:config-file-not-found)".into())
      );
    }
  }
}
