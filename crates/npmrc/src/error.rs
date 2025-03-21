use thiserror::Error;

#[derive(Error, Debug)]
pub enum NpmrcValidatorError {
  #[error("Npmrc file not found {0}")]
  NpmrcFileNotFound(String),

  #[error("Build config error {0}")]
  BuildConfigError(#[from] config::ConfigError),

  #[error("Registry not found")]
  RegistryNotFound,

  #[error("Registry value is empty")]
  RegistryValueIsEmpty,

  #[error("Registry value is not expected {0} , actual {1}")]
  RegistryValueMatchedFailed(String, String),
}
