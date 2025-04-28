use doctor_ext::define_errors;
use miette::{Diagnostic, NamedSource, SourceSpan};

define_errors! {
  PackageJsonValidatorError{
    #[message = "missing name field in {config_path}"]
    MissingNameErr{
      config_path: String,
    },

    #[message = "missing private field in {config_path}"]
    MissingPrivateErr{
      config_path: String,
    },

    #[message = "no matched private field , expect {expect} but actual {actual} in {config_path}"]
    NoMatchedPrivateErr{
      config_path: String,
      expect: bool,
      actual: bool,
    },

    #[message = "missing package manager field in {config_path}"]
    MissingPackageManagerErr{
      config_path: String,
    },

    #[message = "parse error: {path} {source}"]
    ParseErr{
      path: String,
      source: std::io::Error,
    },
  }
}

define_errors! {
  VersionError {
    #[message = "npm alias parser error: {version}"]
    AliasParserErr{
      version: String,
    },
  }
}

#[derive(Debug, Diagnostic, thiserror::Error)]
#[error("missing private field")]
#[diagnostic(
  code(package_json::missing_private_err),
  url(docsrs),
  help("please add private field in package.json")
)]
pub struct MissingPrivateErrDemo {
  #[source_code]
  src: NamedSource<String>,

  #[label("add private field like this")]
  bad_bit: SourceSpan,
}

#[cfg(test)]
mod tests {
  use super::*;
  use biome_json_parser::{JsonParserOptions, parse_json};
  use miette::{GraphicalTheme, Result};
  use package_json_parser::PackageJsonParser;

  #[test]
  fn test_missing_private_err() -> Result<()> {
    miette::set_hook(Box::new(|_| {
      Box::new(
        miette::MietteHandlerOpts::new()
          .terminal_links(true)
          .unicode(true)
          .context_lines(10)
          // .graphical_theme(GraphicalTheme::unicode())
          // .tab_width(4)
          .break_words(true)
          .build(),
      )
    }))?;

    let source_code = r#"{
      "name": "doctor",
      "version": "0.0.1",
      "packageManager": "pnpm@10.8.1",
      "scripts": {
        "lint": "eslint .",
        "lint:fix": "eslint . --fix"
      }
    }"#;

    let mut p: PackageJsonParser = serde_json::from_str(source_code).unwrap();
    p.private = Some(true);
    let source_code = serde_json::to_string_pretty(&p).unwrap();

    let parse = parse_json(source_code.as_str(), JsonParserOptions::default());

    let root = parse.tree();

    if let Some(obj_node) = root.value().unwrap().as_json_object_value() {
      for member in obj_node.json_member_list() {
        let member = member.ok().unwrap();

        let name = member.name().ok().unwrap();

        let any_value = member.value().ok().unwrap();

        if name.inner_string_text().unwrap().to_string() == "private" {
          let bool_value = any_value.as_json_boolean_value().unwrap();

          let name_token = name.value_token().unwrap();
          let value_token = bool_value.value_token().unwrap();

          let name_range = name_token.text_trimmed_range();

          let value_range = value_token.text_trimmed_range();

          let start_byte: usize = name_range.start().into();
          let end_byte: usize = value_range.end().into();

          let span = SourceSpan::new(start_byte.into(), end_byte - start_byte);
          let src = NamedSource::new("package.json", source_code.to_string());
          let err = MissingPrivateErrDemo { src, bad_bit: span };
          println!("{}", err);
          return Err(err.into());
        }
      }
    }

    Ok(())
  }
}
