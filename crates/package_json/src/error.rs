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

  #[error("no matched private field , expect {expect_value} but actual {actual_value}")]
  NoMatchedPrivateError {
    expect_value: bool,
    actual_value: bool,
  },

  #[error("no package manager field")]
  NoPackageManagerError,

  #[error("IO error: {0}")]
  IoError(#[from] std::io::Error),

  #[error("Parse error: {0}")]
  ParseError(#[from] serde_json::Error),
}
