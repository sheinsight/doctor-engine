use package_json_parser::ErrorKind;

#[derive(Debug, thiserror::Error)]
pub enum ValidatorError {
  #[error(transparent)]
  IoError(#[from] std::io::Error),

  #[error(transparent)]
  SemverError(#[from] node_semver::SemverError),

  #[error(transparent)]
  SerdeIniError(#[from] serde_ini::de::Error),

  // #[error(transparent)]
  // SerdeJsonError(#[from] serde_json::Error),
  #[error(transparent)]
  PackageJsonParserError(#[from] ErrorKind),

  #[error(transparent)]
  JsonSyntaxError(#[from] biome_rowan::SyntaxError),

  #[error(transparent)]
  OxcConfigBuilderError(#[from] oxc_linter::ConfigBuilderError),

  #[error(transparent)]
  MietteInstallError(#[from] miette::InstallError),

  #[error(transparent)]
  FailedToBuildConfigStore(#[from] oxc_diagnostics::OxcDiagnostic),

  #[error("unknown error {0}")]
  Unknown(#[from] Box<dyn std::error::Error + Send + Sync>),
}
