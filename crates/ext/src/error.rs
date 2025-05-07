#[derive(Debug, thiserror::Error)]
pub enum ValidatorError {
  #[error(transparent)]
  IoError(#[from] std::io::Error),

  #[error(transparent)]
  SemverError(#[from] node_semver::SemverError),

  #[error(transparent)]
  SerdeIniError(#[from] serde_ini::de::Error),

  #[error(transparent)]
  JsonSyntaxError(#[from] biome_rowan::SyntaxError),

  #[error(transparent)]
  OxcConfigBuilderError(#[from] oxc_linter::ConfigBuilderError),

  #[error("unknown error")]
  Unknown,
}
