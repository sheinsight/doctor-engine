use biome_json_parser::{JsonParserOptions, parse_json};
use miette::{NamedSource, Result, SourceSpan};
use package_json_parser::PackageJsonParser;

use crate::error::MissingPrivateErrDemo;

pub fn demo_pj() -> Result<()> {
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
    "private": false,
    "scripts": {
      "lint": "eslint .",
      "lint:fix": "eslint . --fix"
    }
  }"#;

  let mut p: PackageJsonParser = serde_json::from_str(source_code).unwrap();
  // p.private = Some(true);
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

        // let name_range = name_token.text_trimmed_range();

        let value_range = value_token.text_trimmed_range();

        let start_byte: usize = value_range.start().into();
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
