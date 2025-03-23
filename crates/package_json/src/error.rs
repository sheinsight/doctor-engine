use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum PackageJsonValidatorError {
  #[error("npm alias parser error: {version}")]
  NpmAliasParserError { version: String },

  #[error("{0} no name field")]
  NoNameError(String),

  #[error("{0} not found private field")]
  NoPrivateError(String),

  #[error("{path} no matched private field , expect {expect_value} but actual {actual_value}")]
  NoMatchedPrivateError {
    path: String,
    expect_value: bool,
    actual_value: bool,
  },

  #[error("{0} no package manager field")]
  NoPackageManagerError(String),

  #[error("{path} {error}")]
  IoError {
    path: String,
    #[source]
    error: std::io::Error,
  },

  #[error("{path} {error}")]
  ParseError {
    path: String,
    #[source]
    error: serde_json::Error,
  },
}
