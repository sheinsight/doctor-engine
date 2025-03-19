#![recursion_limit = "512"]
use std::{path::Path, rc::Rc, sync::Arc};

use common::{error::LintError, file_diagnostic::FileDiagnostic, named_source};
use oxc_allocator::Allocator;
use oxc_diagnostics::OxcDiagnostic;
use oxc_linter::{ConfigStoreBuilder, FixKind, FrameworkFlags, LintOptions, Oxlintrc};
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
pub mod common;
pub mod inner;
pub mod oxlint_rc_builder;

pub struct Linter {
  oxlintrc: Oxlintrc,
  show_report: bool,
}

impl Default for Linter {
  fn default() -> Self {
    Self {
      oxlintrc: Oxlintrc::default(),
      show_report: false,
    }
  }
}

impl From<Oxlintrc> for Linter {
  fn from(config: Oxlintrc) -> Self {
    Self {
      oxlintrc: config,
      show_report: false,
    }
  }
}

impl Linter {
  pub fn with_show_report(mut self, show_report: bool) -> Self {
    self.show_report = show_report;
    self
  }

  fn source_type_from_path<P: AsRef<Path>>(&self, path: P) -> oxc_span::SourceType {
    match path.as_ref().extension().and_then(|ext| ext.to_str()) {
      Some("ts") | Some("cts") | Some("mts") => oxc_span::SourceType::ts(),
      Some("tsx") => oxc_span::SourceType::tsx(),
      Some("jsx") => oxc_span::SourceType::jsx(),
      Some("cjs") => oxc_span::SourceType::cjs(),
      Some("mjs") => oxc_span::SourceType::mjs(),
      _ => oxc_span::SourceType::mjs(),
    }
  }

  fn convert_severity(severity: oxc_diagnostics::Severity) -> miette::Severity {
    match severity {
      oxc_diagnostics::Severity::Error => miette::Severity::Error,
      oxc_diagnostics::Severity::Warning => miette::Severity::Warning,
      oxc_diagnostics::Severity::Advice => miette::Severity::Advice,
    }
  }

  pub fn render_report(
    &self,
    source_code: miette::NamedSource<String>,
    diagnostic: &OxcDiagnostic,
  ) -> String {
    let url = diagnostic
      .url
      .as_ref()
      .map_or(String::new(), |url| url.to_string());
    let help = diagnostic
      .help
      .as_ref()
      .map_or(String::new(), |help| help.to_string());
    let scope = diagnostic
      .code
      .scope
      .as_ref()
      .map(|scope| scope.to_string());

    let number = diagnostic
      .code
      .number
      .as_ref()
      .map(|number| number.to_string());

    let labels = diagnostic.labels.as_ref().map_or(vec![], |labels| {
      labels
        .iter()
        .map(|label| {
          let start = label.offset();
          let end = start + label.len();
          let label = label
            .label()
            .map_or(diagnostic.message.clone().to_string(), |label| {
              label.to_string()
            });
          miette::LabeledSpan::at(start..end, label)
        })
        .collect()
    });

    let severity = Self::convert_severity(diagnostic.severity);

    let report = miette::miette!(
      severity = severity,
      url = url,
      labels = labels,
      help = help,
      "{}/{}: {}",
      scope.as_ref().unwrap_or(&String::new()),
      number.as_ref().unwrap_or(&String::new()),
      diagnostic.message
    )
    .with_source_code(source_code);

    eprintln!("{:?}", report);

    // println!("{}", diagnostic.message);

    format!(
      "{}/{}",
      scope.unwrap_or_default(),
      number.unwrap_or_default()
    )
  }

  pub fn lint<P: AsRef<Path>>(&self, path: P) -> Result<FileDiagnostic, LintError> {
    let named_source = named_source::PathWithSource::try_from(path)?;

    let config = ConfigStoreBuilder::from_oxlintrc(true, self.oxlintrc.clone())?.build()?;

    let lint = oxc_linter::Linter::new(
      LintOptions {
        fix: FixKind::None,
        framework_hints: FrameworkFlags::empty(),
      },
      config,
    );

    let allocator = Allocator::default();

    let source_type = self.source_type_from_path(&named_source.file_path);

    let parser = Parser::new(&allocator, &named_source.source_code, source_type);

    let parser_return = parser.parse();

    let program = allocator.alloc(&parser_return.program);

    let semantic_builder_return = SemanticBuilder::new()
      .with_check_syntax_error(true)
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

    if self.show_report {
      self.custom_render_report(&diag);
    }

    Ok(diag)
  }

  pub fn custom_render_report(&self, diagnostic: &FileDiagnostic) {
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
}

#[cfg(test)]
mod tests {
  use std::{collections::HashMap, env::current_dir};

  use walk_parallel::{WalkParallel, walk_patterns::WalkPatterns};

  use crate::{
    common::{category_getter::Category, lint_mode::LintMode},
    inner::v2025_06_01::category::Category20250601Inner,
    oxlint_rc_builder::OxlintrcBuilder,
  };

  use super::*;

  #[test]
  fn test_lint() {
    let category = Category::V20250601Inner(Category20250601Inner::default());

    let rc = OxlintrcBuilder::default()
      .with_category(category)
      .with_mode(LintMode::Production)
      .build()
      .unwrap();

    let rc_str = serde_json::to_string_pretty(&rc).unwrap();

    std::fs::write(".oxlintrc.json", rc_str).unwrap();

    let linter = Linter::from(rc).with_show_report(true);

    let cwd = current_dir().unwrap().join("examples");

    let file_diagnostics = WalkParallel::new(&cwd)
      .with_patterns(WalkPatterns::default())
      .walk(|path| {
        let result = linter.lint(path).unwrap();
        Some(result)
      })
      .unwrap();

    let mut map = HashMap::new();

    for file_diagnostic in file_diagnostics {
      file_diagnostic.diagnostics.iter().for_each(|diag| {
        let name = match (diag.code.scope.as_ref(), diag.code.number.as_ref()) {
          (None, None) => "".to_string(),
          (None, Some(number)) => number.to_string(),
          (Some(scope), None) => scope.to_string(),
          (Some(scope), Some(number)) => format!("{}/{}", scope, number).to_string(),
        };

        map.entry(name).and_modify(|count| *count += 1).or_insert(1);
      });
    }

    println!("{:#?}", map);
  }
}
