use std::path::Path;

use doctor_ext::{Messages, Validator};

use miette::{LabeledSpan, MietteDiagnostic};
use typed_builder::TypedBuilder;

use crate::{error::NpmrcValidatorError, npmrc_config::NpmrcConfig};

/// NpmrcValidator is a validator for npmrc file
///
/// # Example
///
/// ```rust
/// use doctor_npmrc::validator::NpmrcValidator;
/// use doctor_ext::Validator;
///
/// let validator = NpmrcValidator::builder()
///   .config_path("./fixtures/.npmrc")
///   .with_registry_url(vec!["https://test.npmjs.org/"])
///   .build();
/// assert!(validator.validate().is_ok());
/// ```
#[derive(TypedBuilder)]
pub struct NpmrcValidator<'a, P>
where
  P: AsRef<Path>,
{
  config_path: P,

  #[builder(default = None,setter(strip_option))]
  with_registry_url: Option<Vec<&'a str>>,

  #[builder(default = None, setter(strip_option))]
  with_additional_validation:
    Option<Box<dyn Fn(&NpmrcConfig) -> Result<Vec<MietteDiagnostic>, NpmrcValidatorError> + 'a>>,
}

impl<'a, P> NpmrcValidator<'a, P>
where
  P: AsRef<Path>,
{
  fn validate_registry(
    &self,
    config: &NpmrcConfig,
  ) -> Result<Vec<MietteDiagnostic>, NpmrcValidatorError> {
    let mut diagnostics = vec![];

    if let Some(validate_registry) = &self.with_registry_url {
      if let Some(registry) = &config.registry {
        if !validate_registry.iter().any(|item| item == &registry) {
          let (offset, length) = self
            .find_registry_position(&config.__raw_source)
            .unwrap_or((0, 0));
          diagnostics.push(
            MietteDiagnostic::new("Wrong registry")
              .with_code("shined_doctor/npmrc_wrong_registry")
              .with_severity(miette::Severity::Error)
              .with_label(LabeledSpan::at(
                offset..offset + length,
                format!(
                  r#"Wrong registry , Only support registry: {}"#,
                  validate_registry.join(", ")
                ),
              ))
              .with_help("Please add a registry field to your .npmrc file"),
          );
          return Ok(diagnostics);
        }
      } else {
        diagnostics.push(
          MietteDiagnostic::new("No registry field found")
            .with_code("shined_doctor/npmrc_missing_registry")
            .with_severity(miette::Severity::Error)
            .with_label(LabeledSpan::at(
              0..config.__raw_source.len(),
              "No registry field found",
            ))
            .with_help("Please add a registry field to your .npmrc file"),
        );

        return Ok(diagnostics);
      }
    }
    Ok(diagnostics)
  }

  fn validate_additional_validation(
    &self,
    config: &NpmrcConfig,
  ) -> Result<Vec<MietteDiagnostic>, NpmrcValidatorError> {
    let diagnostics = vec![];

    if let Some(additional_validation) = &self.with_additional_validation {
      additional_validation(config)?;
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

impl<'a, P> Validator for NpmrcValidator<'a, P>
where
  P: AsRef<Path>,
{
  type Error = NpmrcValidatorError;

  /// Validate npmrc file
  ///
  /// # Example
  ///
  /// ```rust
  /// use doctor_npmrc::validator::NpmrcValidator;
  /// use doctor_ext::Validator;
  ///
  /// let validator = NpmrcValidator::builder()
  ///   .config_path("./fixtures/.npmrc")
  ///   .with_registry_url(vec!["https://test.npmjs.org/"])
  ///   .build();
  /// assert!(validator.validate().is_ok());
  /// ```
  fn validate(&self) -> Result<Messages, Self::Error> {
    let config = NpmrcConfig::parse(self.config_path.as_ref())?;

    let mut messages = Messages::builder()
      .source_code(config.__raw_source.clone())
      .diagnostics(vec![])
      .build();

    let diagnostics = self.validate_registry(&config)?;

    messages.extend(diagnostics.into_iter());

    let diagnostics = self.validate_additional_validation(&config)?;

    messages.extend(diagnostics.into_iter());

    Ok(messages)
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_validate_registry() {
    let result = NpmrcValidator::builder()
      .config_path("fixtures/.npmrc")
      .with_registry_url(vec!["https://test.npmjs.org/"])
      .build()
      .validate();

    assert!(result.is_ok());
  }

  #[test]
  fn test_validate_registry_error() {
    let result = NpmrcValidator::builder()
      .config_path("fixtures/.npmrc")
      .with_registry_url(vec!["https://test.npmjs.org"])
      .build()
      .validate();

    let result = result.unwrap();

    assert!(result.has_error());

    result.render();
  }

  #[test]
  fn test_validate_registry_error_not_found_field() {
    let result = NpmrcValidator::builder()
      .config_path("fixtures/.undef_registry")
      .with_registry_url(vec!["https://test.npmjs.org/"])
      .build()
      .validate();

    let result = result.unwrap();

    assert!(result.has_error());

    result.render();
  }

  #[test]
  fn test_validate_registry_error_registry_value_is_empty() {
    let result = NpmrcValidator::builder()
      .config_path("fixtures/.empty_registry")
      .with_registry_url(vec!["https://test.npmjs.org/"])
      .build()
      .validate();

    let result = result.unwrap();

    assert!(result.has_error());

    result.render();
  }

  // #[test]
  // fn test_validate_additional_validation() {
  //   let result = NpmrcValidator::builder()
  //     .config_path("fixtures/.npmrc")
  //     .with_additional_validation(Box::new(|config| {
  //       config
  //         .get::<String>("unknown_field")
  //         .map_err(|e| UnknownErr::builder().source(Box::new(e)).build().into())?;
  //       Ok(())
  //     }))
  //     .build()
  //     .validate();
  //   assert!(matches!(
  //     result,
  //     Err(NpmrcValidatorError::UnknownErr { .. })
  //   ));
  // }
}
