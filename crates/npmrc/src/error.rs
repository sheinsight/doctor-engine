use miette::{Diagnostic, NamedSource, SourceSpan};

use crate::npmrc_config::NpmrcConfig;

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum NpmrcValidatorError {
  #[error(transparent)]
  #[diagnostic(transparent)]
  MissingRegistryError(#[from] MissingRegistryError),

  #[error(transparent)]
  #[diagnostic(transparent)]
  IoError(#[from] IoErrorWrapper),

  #[error(transparent)]
  #[diagnostic(transparent)]
  SerdeIniError(#[from] SerdeIniErrorWrapper),

  #[error(transparent)]
  #[diagnostic(transparent)]
  WrongRegistryError(#[from] WrongRegistryError),
}

#[derive(Debug, thiserror::Error, Diagnostic)]
#[error("serde ini error {0}")]
#[diagnostic(
  code(npmrc::serde_ini_error),
  url(docsrs),
  help("please check config file")
)]
pub struct SerdeIniErrorWrapper(#[from] serde_ini::de::Error);

impl From<serde_ini::de::Error> for NpmrcValidatorError {
  fn from(err: serde_ini::de::Error) -> Self {
    Self::SerdeIniError(SerdeIniErrorWrapper(err))
  }
}

#[derive(Debug, thiserror::Error, Diagnostic)]
#[error("io error {0}")]
#[diagnostic(code(npmrc::io_error), url(docsrs), help("please check config file"))]
pub struct IoErrorWrapper(#[from] std::io::Error);

impl From<std::io::Error> for NpmrcValidatorError {
  fn from(err: std::io::Error) -> Self {
    Self::IoError(IoErrorWrapper(err))
  }
}

#[derive(Debug, Diagnostic, thiserror::Error)]
#[error("missing registry field")]
#[diagnostic(
  code(npmrc::missing_registry_err),
  url(docsrs),
  help("please add registry field")
)]
pub struct MissingRegistryError {
  #[source_code]
  pub src: NamedSource<String>,
  #[label("missing registry field")]
  pub bad_bit: SourceSpan,
}

impl MissingRegistryError {
  pub fn new(npmrc_config: &NpmrcConfig) -> Result<(), NpmrcValidatorError> {
    let config_path = npmrc_config.__config_path.clone();
    let raw_source = npmrc_config.__raw_source.clone();
    let len = raw_source.len();
    let src = NamedSource::new(config_path, raw_source);
    let bad_bit = SourceSpan::new(0.into(), len.into());
    Err(Self { src, bad_bit })?
  }
}

#[derive(Debug, thiserror::Error)]
#[error("only support registry: {expect}")]
pub struct WrongRegistryError {
  pub src: NamedSource<String>,
  pub bad_bit: SourceSpan,
  pub expect: String,
}

impl Diagnostic for WrongRegistryError {
  fn code<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
    Some(Box::new("npmrc::wrong_registry_err"))
  }

  fn severity(&self) -> Option<miette::Severity> {
    Some(miette::Severity::Error)
  }

  fn help<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
    Some(Box::new("please check config file"))
  }

  fn url<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
    Some(Box::new(
      "https://docs.rs/miette/latest/miette/trait.Diagnostic.html#method.url",
    ))
  }

  fn source_code(&self) -> Option<&dyn miette::SourceCode> {
    Some(&self.src)
  }

  fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
    Some(Box::new(std::iter::once(miette::LabeledSpan::new(
      Some(format!("only support registry: {}", self.expect)),
      self.bad_bit.offset(),
      self.bad_bit.len(),
    ))))
  }

  fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic> + 'a>> {
    None
  }

  fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
    None
  }
}

impl WrongRegistryError {
  pub fn new(npmrc_config: &NpmrcConfig, expect: &str) -> Result<(), NpmrcValidatorError> {
    let raw_source = npmrc_config.__raw_source.clone();
    let config_path = npmrc_config.__config_path.clone();
    let (offset, length) = Self::find_registry_position(&raw_source).unwrap_or((0, 0));

    let src = NamedSource::new(config_path, raw_source);
    let bad_bit = SourceSpan::new(offset.into(), length.into());
    Err(Self {
      src,
      bad_bit,
      expect: expect.to_string(),
    })?
  }

  fn find_registry_position(content: &str) -> Option<(usize, usize)> {
    for line in content.lines() {
      if let Some(key_pos) = line.find("registry=") {
        let equals_pos = key_pos + "registry".len();
        let value_start = equals_pos + 1;
        let value_length = line.len() - value_start;

        let line_offset = content.find(line).unwrap();
        let absolute_offset = line_offset + value_start;

        return Some((absolute_offset, value_length));
      }
    }
    None
  }
}
