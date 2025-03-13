use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
  #[error("npm alias parser error: {version}")]
  NpmAliasParserError { version: String },
}

#[derive(Diagnostic, Debug, Error)]
#[error("doctor/no-name")]
#[diagnostic(code(package_json::no_name), url(docsrs), help("add name field"))]
pub struct NoNameError {
  #[source_code]
  pub src: NamedSource<String>,

  #[label("not found name field")]
  pub bit: SourceSpan,
}

impl NoNameError {
  pub fn new(file: &str, source_code: &str) -> Self {
    Self {
      src: NamedSource::new(file, source_code.to_string()),
      bit: (0, source_code.len()).into(),
    }
  }
}

#[derive(Diagnostic, Debug, Error)]
#[error("doctor/no-private")]
#[diagnostic(code(package_json::no_private), url(docsrs), help("add private field"))]
pub struct NoPrivateError {
  #[source_code]
  pub src: NamedSource<String>,

  #[label("not found private field")]
  pub bit: SourceSpan,
}

impl NoPrivateError {
  pub fn new(file: &str, source_code: &str) -> Self {
    Self {
      src: NamedSource::new(file, source_code.to_string()),
      bit: (0, source_code.len()).into(),
    }
  }
}

#[derive(Diagnostic, Debug, Error)]
#[error("doctor/private-value-not-match")]
#[diagnostic(
  code(package_json::private_value_not_match),
  url(docsrs),
  help("private field value not match")
)]
pub struct PrivateValueNotMatchError {
  #[source_code]
  pub src: NamedSource<String>,

  #[label("private field value not match")]
  pub bit: SourceSpan,
}

impl PrivateValueNotMatchError {
  pub fn new(file: &str, source_code: &str, bit: SourceSpan) -> Self {
    Self {
      src: NamedSource::new(file, source_code.to_string()),
      bit,
    }
  }
}
