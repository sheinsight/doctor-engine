use miette::{Diagnostic, NamedSource, SourceSpan};

use crate::node_version::NodeVersion;

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum NodeVersionValidatorError {
  // #[error(transparent)]
  // #[diagnostic(transparent)]
  // NotFoundConfigFileError(#[from] NotFoundConfigFile),
  // #[error(transparent)]
  // #[diagnostic(transparent)]
  // InvalidNodeVersionError(#[from] InvalidNodeVersion),
  // #[error(transparent)]
  // #[diagnostic(transparent)]
  // EmptyNodeVersionError(#[from] EmptyNodeVersionError),
  // #[error(transparent)]
  // #[diagnostic(transparent)]
  // VersionRequirementNotMetError(#[from] VersionRequirementNotMet),
  #[error(transparent)]
  #[diagnostic(transparent)]
  IoError(#[from] IoErrorWrapper),

  #[error(transparent)]
  #[diagnostic(transparent)]
  SemverError(#[from] SemverErrorWrapper),
}

#[derive(Debug, Diagnostic, thiserror::Error)]
#[error("semver error {0}")]
#[diagnostic(
  code(node::semver_error),
  url(docsrs),
  help("please check config file")
)]
pub struct SemverErrorWrapper(#[from] node_semver::SemverError);

impl From<node_semver::SemverError> for NodeVersionValidatorError {
  fn from(err: node_semver::SemverError) -> Self {
    Self::SemverError(SemverErrorWrapper(err))
  }
}

#[derive(Debug, Diagnostic, thiserror::Error)]
#[error("io error {0}")]
#[diagnostic(code(node::io_error), url(docsrs), help("please check config file"))]
pub struct IoErrorWrapper(#[from] std::io::Error);

impl From<std::io::Error> for NodeVersionValidatorError {
  fn from(err: std::io::Error) -> Self {
    Self::IoError(IoErrorWrapper(err))
  }
}

#[derive(Debug, Diagnostic, thiserror::Error)]
#[error("Not found .node-version config file")]
#[diagnostic(
  code(node::not_found_config_file),
  url(docsrs),
  help("please check config file")
)]
pub struct NotFoundConfigFile {
  #[source_code]
  pub src: NamedSource<String>,
  #[label("Not found .node-version config file")]
  pub bad_bit: SourceSpan,
}

impl NotFoundConfigFile {
  pub fn new(node_version: &NodeVersion) -> Self {
    let config_path = node_version.__config_path.clone();
    let src = NamedSource::new(config_path, "".to_string());
    let bad_bit = SourceSpan::new(0.into(), 0);
    Self { src, bad_bit }
  }
}

#[derive(Debug, Diagnostic, thiserror::Error)]
#[error("Invalid node version {version} in {config_path}")]
#[diagnostic(
  code(node::invalid_node_version),
  url(docsrs),
  help("please check config file")
)]
pub struct InvalidNodeVersion {
  #[source_code]
  pub src: NamedSource<String>,
  #[label("Invalid node version {version} in {config_path}")]
  pub bad_bit: SourceSpan,

  pub version: String,

  pub config_path: String,
}

impl InvalidNodeVersion {
  pub fn new(node_version: &NodeVersion) -> Self {
    let raw = node_version.__raw_source.clone().unwrap();
    let path = node_version.__config_path.clone();

    let src = NamedSource::new(path.clone(), raw.clone());
    let bad_bit = SourceSpan::new(0.into(), 0);
    Self {
      src,
      bad_bit,
      version: raw,
      config_path: path,
    }
  }
}

#[derive(Debug, Diagnostic, thiserror::Error)]
#[error("Empty node version in {config_path}")]
#[diagnostic(
  code(node::empty_node_version),
  url(docsrs),
  help("please check config file")
)]
pub struct EmptyNodeVersionError {
  #[source_code]
  pub src: NamedSource<String>,
  #[label("Empty node version in {config_path}")]
  pub bad_bit: SourceSpan,

  pub config_path: String,
}

impl EmptyNodeVersionError {
  pub fn new(node_version: &NodeVersion) -> Self {
    let raw = node_version.__raw_source.clone().unwrap();
    let path = node_version.__config_path.clone();

    let src = NamedSource::new(path.clone(), raw.clone());
    let bad_bit = SourceSpan::new(0.into(), 0);
    Self {
      src,
      bad_bit,
      config_path: path,
    }
  }
}

#[derive(Debug, Diagnostic, thiserror::Error)]
#[error("Version {version} found in {config_path} does not meet the version requirements.")]
#[diagnostic(
  code(node::version_requirement_not_met),
  url(docsrs),
  help("please check config file")
)]
pub struct VersionRequirementNotMet {
  #[source_code]
  pub src: NamedSource<String>,
  #[label("Version {version} found in {config_path} does not meet the version requirements.")]
  pub bad_bit: SourceSpan,

  pub version: String,
  pub config_path: String,
}

impl VersionRequirementNotMet {
  pub fn new(node_version: &NodeVersion) -> Self {
    let raw = node_version.__raw_source.clone().unwrap();
    let config_path = node_version.__config_path.clone();
    let src = NamedSource::new(config_path.clone(), raw.clone());
    let bad_bit = SourceSpan::new(0.into(), 0);
    Self {
      src,
      bad_bit,
      version: raw,
      config_path,
    }
  }
}
