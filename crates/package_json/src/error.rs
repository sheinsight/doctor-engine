use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
  #[error("npm alias parser error: {version}")]
  NpmAliasParserError { version: String },

  #[error("{0} : no name field")]
  NoNameError(String),

  #[error("{0} : not found private field")]
  NoPrivateError(String),

  #[error(
    "{file_path} : no matched private field , expect {expect_value} but actual {actual_value}"
  )]
  NoMatchedPrivateError {
    file_path: String,
    expect_value: bool,
    actual_value: bool,
  },

  #[error("{0} : no package manager field")]
  NoPackageManagerError(String),

  #[error("IO error: {0}")]
  IoError(#[from] std::io::Error),

  #[error("Parse error: {0}")]
  ParseError(#[from] serde_json::Error),
}
