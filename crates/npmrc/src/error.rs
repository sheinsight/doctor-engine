use thiserror::Error;

#[derive(Error, Debug)]
pub enum NpmrcValidatorError {
  #[error("Npmrc file not found {0}")]
  NpmrcFileNotFound(String),

  #[error("{0} {1}")]
  BuildConfigError(String, #[source] config::ConfigError),

  #[error("Registry not found")]
  RegistryNotFound,

  #[error("Registry value is empty")]
  RegistryValueIsEmpty,

  #[error("Registry value is not expected {expect} , actual {actual}")]
  RegistryValueMatchedFailed { expect: String, actual: String },
}
