use std::path::Path;

use config::{Config, File, FileFormat};
use doctor_ext::{PathExt, Validator};

use typed_builder::TypedBuilder;

use crate::error::{BuildConfigErr, MatchedFailErr, NotFoundFieldErr, NpmrcValidatorError};

const REGISTRY: &str = "registry";

#[derive(TypedBuilder)]
pub struct NpmrcValidator<'a, P>
where
  P: AsRef<Path>,
{
  config_path: P,

  #[builder(default = None,setter(strip_option))]
  with_registry_url: Option<Vec<&'a str>>,

  #[builder(default = None, setter(strip_option))]
  additional_validation: Option<Box<dyn Fn(&Config) -> Result<(), NpmrcValidatorError> + 'a>>,
}

impl<'a, P> NpmrcValidator<'a, P>
where
  P: AsRef<Path>,
{
  fn validate_registry(&self, config: &Config) -> Result<(), NpmrcValidatorError> {
    let config_path = self.config_path.as_ref();

    let err = NotFoundFieldErr::builder()
      .field(REGISTRY.to_string())
      .config_path(config_path.to_string_owned());

    let registry = config
      .get::<String>(REGISTRY)
      .map_err(|e| err.source(e).build().into())?;

    if let Some(validate_registry) = &self.with_registry_url {
      if !validate_registry.iter().any(|item| item == &registry) {
        return MatchedFailErr::builder()
          .expect(validate_registry.join(" or "))
          .actual(registry)
          .config_path(config_path.to_string_owned())
          .build()
          .into();
      }
    }

    Ok(())
  }

  fn validate_additional_validation(&self, config: &Config) -> Result<(), NpmrcValidatorError> {
    if let Some(additional_validation) = &self.additional_validation {
      additional_validation(config)?;
    }

    Ok(())
  }
}

impl<'a, P> Validator for NpmrcValidator<'a, P>
where
  P: AsRef<Path>,
{
  type Error = NpmrcValidatorError;

  fn validate(&self) -> Result<(), Self::Error> {
    let source = File::from(self.config_path.as_ref()).format(FileFormat::Ini);

    let config = Config::builder().add_source(source).build().map_err(|e| {
      BuildConfigErr::builder()
        .config_path(self.config_path.as_ref().to_string_owned())
        .source(e)
        .build()
        .into()
    })?;

    self.validate_registry(&config)?;

    self.validate_additional_validation(&config)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::error::UnknownErr;

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

    assert!(matches!(
      result,
      Err(NpmrcValidatorError::MatchedFailErr { .. })
    ));
  }

  #[test]
  fn test_validate_registry_error_not_found_field() {
    let result = NpmrcValidator::builder()
      .config_path("fixtures/.undef_registry")
      .with_registry_url(vec!["https://test.npmjs.org/"])
      .build()
      .validate();

    assert!(matches!(
      result,
      Err(NpmrcValidatorError::NotFoundFieldErr { .. })
    ));
  }

  #[test]
  fn test_validate_registry_error_registry_value_is_empty() {
    let result = NpmrcValidator::builder()
      .config_path("fixtures/.empty_registry")
      .with_registry_url(vec!["https://test.npmjs.org/"])
      .build()
      .validate();
    assert!(matches!(
      result,
      Err(NpmrcValidatorError::MatchedFailErr { .. })
    ));
  }

  #[test]
  fn test_validate_additional_validation() {
    // let additional_validation = |config: &Config| Ok(());

    let result = NpmrcValidator::builder()
      .config_path("fixtures/.npmrc")
      .additional_validation(Box::new(|config| {
        config
          .get::<String>("unknown_field")
          .map_err(|e| UnknownErr::builder().source(Box::new(e)).build().into())?;
        Ok(())
      }))
      .build()
      .validate();
    assert!(matches!(
      result,
      Err(NpmrcValidatorError::UnknownErr { .. })
    ));
  }
}
