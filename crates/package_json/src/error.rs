use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
  #[error("npm alias parser error: {version}")]
  NpmAliasParserError { version: String },

  #[error("no name field")]
  NoNameError,

  #[error("no private field")]
  NoPrivateError,

  #[error("no package manager field")]
  NoPackageManagerError,
}
