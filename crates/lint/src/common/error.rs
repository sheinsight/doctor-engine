use thiserror::Error;

#[derive(Debug, Error)]
pub enum LintError {
  #[error("IO error: {0}")]
  Io(#[from] std::io::Error),

  #[error("Other error: {0}")]
  Other(String),

  #[error("Failed to build config: {0}")]
  FailedToBuildOxlintrc(String),

  #[error("Failed to build config store builder: {0}")]
  FailedToBuildConfigStoreBuilder(#[from] oxc_linter::ConfigBuilderError),

  #[error("Failed to build config store: {0}")]
  FailedToBuildConfigStore(#[from] oxc_diagnostics::OxcDiagnostic),
}
