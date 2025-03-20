use std::path::{Path, PathBuf};

use config::{Config, File, FileFormat};
use doctor_ext::MultiFrom;
use error::NpmrcError;

pub mod error;

const FILE_NAME: &str = ".npmrc";

#[derive(Debug)]
pub struct Npmrc {
  file_path: PathBuf,
}

impl MultiFrom for Npmrc {
  type Error = NpmrcError;

  fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Self::Error> {
    Ok(Self {
      file_path: path.as_ref().to_path_buf(),
    })
  }

  fn from_cwd<P: AsRef<Path>>(cwd: P) -> Result<Self, Self::Error> {
    let file_path = cwd.as_ref().join(FILE_NAME);
    Ok(Self { file_path })
  }
}

impl Npmrc {
  pub fn validate_registry(self, expected: &str) -> Result<Self, NpmrcError> {
    if !self.file_path.exists() {
      return Err(NpmrcError::NpmrcFileNotFound(
        self.file_path.to_string_lossy().to_string(),
      ));
    }

    let source = File::from(self.file_path.as_path()).format(FileFormat::Ini);

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
    let npmrc = Npmrc::from_cwd("fixtures/success").unwrap();
    let result = npmrc.validate_registry("https://test.npmjs.org/");
    assert!(result.is_ok());
  }

  #[test]
  fn test_validate_registry_error() {
    let npmrc = Npmrc::from_cwd("fixtures/not_found_registry").unwrap();
    let result = npmrc.validate_registry("https://test.npmjs.org/");
    assert!(result.is_err());
  }

  #[test]
  fn test_validate_registry_error_registry_value_is_empty() {
    let npmrc = Npmrc::from_cwd("fixtures/registry_value_is_empty").unwrap();
    let result = npmrc.validate_registry("https://test.npmjs.org/");
    assert!(result.is_err());
  }
}
