use biome_json_parser::{JsonParserOptions, parse_json};
use biome_rowan::TextRange;
use miette::{Diagnostic, NamedSource, SourceSpan};
use package_json_parser::PackageJsonParser;

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum PackageJsonValidatorError {
  #[error(transparent)]
  #[diagnostic(transparent)]
  MissingPrivateError(#[from] MissingPrivateError),

  #[error(transparent)]
  #[diagnostic(transparent)]
  MissingPackageManagerError(#[from] MissingPackageManagerError),

  #[error(transparent)]
  #[diagnostic(transparent)]
  MissingNameError(#[from] MissingNameError),

  #[error(transparent)]
  #[diagnostic(transparent)]
  MustBeTrueError(#[from] MustBeTrueError),

  #[error(transparent)]
  #[diagnostic(transparent)]
  JsonSyntaxError(#[from] JsonSyntaxErrorWrapper),
}

#[derive(Debug, thiserror::Error, Diagnostic)]
#[error("JSON syntax error: {0}")]
#[diagnostic(
  code(package_json::json_syntax_err),
  url(docsrs),
  help("please check json syntax")
)]
pub struct JsonSyntaxErrorWrapper(#[from] pub biome_rowan::SyntaxError);

// 转换函数
impl From<biome_rowan::SyntaxError> for PackageJsonValidatorError {
  fn from(err: biome_rowan::SyntaxError) -> Self {
    Self::JsonSyntaxError(JsonSyntaxErrorWrapper(err))
  }
}

#[derive(Debug, Diagnostic, thiserror::Error)]
#[error("missing name field")]
#[diagnostic(
  code(package_json::missing_name_err),
  url(docsrs),
  help("please add name field")
)]
pub struct MissingNameError {
  #[source_code]
  pub src: NamedSource<String>,
  #[label("missing name field")]
  pub bad_bit: SourceSpan,
}

impl MissingNameError {
  pub fn new(package_json: &PackageJsonParser) -> Result<(), PackageJsonValidatorError> {
    let path = package_json.__raw_path.clone().unwrap_or_default();
    let source = package_json.__raw_source.clone().unwrap_or_default();
    let len = source.len();
    let src = NamedSource::new(path, source);
    let bad_bit = SourceSpan::new(0.into(), len);
    Err(Self { src, bad_bit })?
  }
}

#[derive(Debug, Diagnostic, thiserror::Error)]
#[error("missing package manager field")]
#[diagnostic(
  code(package_json::missing_package_manager_err),
  url(docsrs),
  help("please add package manager field")
)]
pub struct MissingPackageManagerError {
  #[source_code]
  pub src: NamedSource<String>,
  #[label("missing package manager field")]
  pub bad_bit: SourceSpan,
}

impl MissingPackageManagerError {
  pub fn new(package_json: &PackageJsonParser) -> Result<(), PackageJsonValidatorError> {
    let path = package_json.__raw_path.clone().unwrap_or_default();
    let source = package_json.__raw_source.clone().unwrap_or_default();
    let len = source.len();
    let src = NamedSource::new(path, source);
    let bad_bit = SourceSpan::new(0.into(), len);
    Err(Self { src, bad_bit })?
  }
}

#[derive(Debug, Diagnostic, thiserror::Error)]
#[error("missing private field")]
#[diagnostic(
  code(package_json::missing_private_err),
  url(docsrs),
  help("please add private field")
)]
pub struct MissingPrivateError {
  #[source_code]
  pub src: NamedSource<String>,
  #[label("missing private field")]
  pub bad_bit: SourceSpan,
}

impl MissingPrivateError {
  pub fn new(package_json: &PackageJsonParser) -> Result<(), PackageJsonValidatorError> {
    let path = package_json.__raw_path.clone().unwrap_or_default();
    let source = package_json.__raw_source.clone().unwrap_or_default();
    let len = source.len();
    let src = NamedSource::new(path, source);
    let bad_bit = SourceSpan::new(0.into(), len);
    Err(Self { src, bad_bit })?
  }
}

#[derive(Debug, Diagnostic, thiserror::Error)]
#[error("private field should be true")]
#[diagnostic(
  code(package_json::must_be_true_err),
  url(docsrs),
  help("please make private field true")
)]
pub struct MustBeTrueError {
  #[source_code]
  pub src: NamedSource<String>,
  #[label("private field should be true")]
  pub bad_bit: SourceSpan,
}

impl MustBeTrueError {
  pub fn new(package_json: &PackageJsonParser) -> Result<(), PackageJsonValidatorError> {
    let path = package_json.__raw_path.clone().unwrap_or_default();
    let source = package_json.__raw_source.clone().unwrap_or_default();
    let range = Self::find_private_range(&source)?;
    let start: usize = range.unwrap_or_default().start().into();
    let end: usize = range.unwrap_or_default().end().into();
    let src = NamedSource::new(path, source);
    let bad_bit = SourceSpan::new(start.into(), (end - start).into());
    Err(Self { src, bad_bit })?
  }

  fn find_private_range(json_raw: &str) -> Result<Option<TextRange>, PackageJsonValidatorError> {
    let parse = parse_json(&json_raw, JsonParserOptions::default());

    let root = parse.tree();

    let root_any_json_value = root.value()?;

    let root = root_any_json_value.as_json_object_value().unwrap();

    for member in root.json_member_list() {
      let member = member?;

      let name = member.name()?;

      if name.inner_string_text()? == "private" {
        let value = member.value().unwrap();
        let value = value.as_json_boolean_value().unwrap();

        let value_range = value.value_token()?.text_trimmed_range();

        return Ok(Some(value_range));
      }
    }

    Ok(None)
  }
}
