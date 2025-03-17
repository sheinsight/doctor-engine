#![recursion_limit = "512"]
use std::{path::Path, rc::Rc, sync::Arc};

use config_builder::ConfigBuilder;
use error::LintError;
use file_diagnostic::FileDiagnostic;
use oxc_allocator::Allocator;
use oxc_linter::{ConfigStoreBuilder, FixKind, FrameworkFlags, LintOptions, Oxlintrc};
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use react_config::ReactConfig;
use rustc_hash::FxHashMap;
use typescript_config::TypescriptConfig;
pub mod category_getter;
pub mod config_builder;
pub mod error;
pub mod file_diagnostic;
pub mod inner;
pub mod lint_mode;
pub mod named_source;
pub mod react_config;
pub mod rule_getter;
pub mod typescript_config;

fn source_type_from_path<P: AsRef<Path>>(path: P) -> oxc_span::SourceType {
  match path.as_ref().extension().and_then(|ext| ext.to_str()) {
    Some("ts") | Some("cts") | Some("mts") => oxc_span::SourceType::ts(),
    Some("tsx") => oxc_span::SourceType::tsx(),
    Some("jsx") => oxc_span::SourceType::jsx(),
    Some("cjs") => oxc_span::SourceType::cjs(),
    Some("mjs") => oxc_span::SourceType::mjs(),
    _ => oxc_span::SourceType::mjs(),
  }
}

fn custom_render_report(diagnostic: &FileDiagnostic) {
  if !diagnostic.diagnostics.is_empty() {
    let handler = oxc_diagnostics::GraphicalReportHandler::new().with_links(true);
    let mut output = String::with_capacity(1024 * 1024);

    let named_source: oxc_diagnostics::NamedSource<String> =
      diagnostic.path_with_source.clone().into();

    for diag in &diagnostic.diagnostics {
      let diag = diag.clone().with_source_code(named_source.clone());
      handler.render_report(&mut output, diag.as_ref()).unwrap();
    }
    eprintln!("{}", output);
  }
}

pub fn lint(path: &str) -> Result<(), LintError> {
  let rc = ConfigBuilder::default()
    // .with_react(ReactConfig::default())
    // .with_typescript(TypescriptConfig::default())
    .build()?;

  let config = ConfigStoreBuilder::from_oxlintrc(true, rc)
    .unwrap()
    .build()
    .unwrap();

  let lint_options = LintOptions {
    fix: FixKind::None,
    framework_hints: FrameworkFlags::empty(),
  };

  // let _nested_configs = FxHashMap::default();

  let lint = oxc_linter::Linter::new(lint_options, config);

  let named_source = named_source::PathWithSource::try_from(path)?;
  let allocator = Allocator::default();
  let source_type = source_type_from_path(path);
  let parser = Parser::new(&allocator, &named_source.source_code, source_type);
  let parser_return = parser.parse();
  let program = allocator.alloc(&parser_return.program);
  let semantic_builder_return = SemanticBuilder::new()
    .with_check_syntax_error(false)
    .with_cfg(true)
    .build(program);
  let semantic = semantic_builder_return.semantic;
  let module_record = Arc::new(oxc_linter::ModuleRecord::new(
    Path::new(&named_source.file_path),
    &parser_return.module_record,
    &semantic,
  ));
  let semantic = Rc::new(semantic);

  let res = lint.run(Path::new(&named_source.file_path), semantic, module_record);

  let diag = FileDiagnostic {
    path_with_source: named_source.clone(),
    diagnostics: res.into_iter().map(|msg| msg.error).collect(),
  };

  custom_render_report(&diag);

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_lint() {
    let path = ".";
    let result = lint(path);
    assert_eq!(result.is_ok(), true);
  }
}
