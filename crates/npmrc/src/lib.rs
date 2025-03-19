use std::path::Path;

use config::{Config, File, FileFormat};
use error::NpmrcError;

pub mod error;

const FILE_NAME: &str = ".npmrc";

#[derive(Debug)]
pub struct Npmrc<P: AsRef<Path>> {
  path: P,
}

impl<P: AsRef<Path>> Npmrc<P> {
  pub fn new(path: P) -> Self {
    Self { path }
  }

  pub fn validate_registry(self, expected: &str) -> Result<Self, NpmrcError> {
    let cwd = self.path.as_ref();

    if !cwd.exists() {
      return Err(NpmrcError::NpmrcFileNotFound(
        cwd.to_string_lossy().to_string(),
      ));
    }

    let file = cwd.join(FILE_NAME);

    let source = File::from(file).format(FileFormat::Ini);

    let config = Config::builder()
      .add_source(source)
      .build()
      .map_err(|e| NpmrcError::BuildConfigError(e))?;

    let registry = config
      .get::<String>("registry")
      .map_err(|_| NpmrcError::RegistryNotFound)?;
    if registry.is_empty() {
      return Err(NpmrcError::RegistryValueIsEmpty);
    }

    if registry != expected {
      return Err(NpmrcError::RegistryValueIsNotExpected(
        expected.to_owned(),
        registry,
      ));
    }

    Ok(self)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_validate_registry() {
    let npmrc = Npmrc::new("fixtures/success");
    let result = npmrc.validate_registry("https://test.npmjs.org/");
    assert!(result.is_ok());
  }

  #[test]
  fn test_validate_registry_error() {
    let npmrc = Npmrc::new("fixtures/not_found_registry");
    let result = npmrc.validate_registry("https://test.npmjs.org/");
    assert!(result.is_err());
  }

  #[test]
  fn test_validate_registry_error_registry_value_is_empty() {
    let npmrc = Npmrc::new("fixtures/registry_value_is_empty");
    let result = npmrc.validate_registry("https://test.npmjs.org/");
    assert!(result.is_err());
  }
}
