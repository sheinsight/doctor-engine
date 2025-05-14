use std::{
  path::{Path, PathBuf},
  rc::Rc,
  sync::Arc,
};

use doctor_core::{Messages, ValidatorError, traits::Validator};
use doctor_walk_parallel::{WalkError, WalkIgnore, WalkParallelJs};
use miette::MietteDiagnostic;
use oxc_allocator::Allocator;
use oxc_linter::{ConfigStoreBuilder, Oxlintrc};
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use typed_builder::TypedBuilder;

use crate::{
  FileDiagnostic,
  common::{error::LintError, named_source},
};

#[derive(Debug, Clone, TypedBuilder)]
pub struct LintValidator {
  cwd: PathBuf,
  oxlintrc: Oxlintrc,

  #[builder(default = false)]
  with_show_report: bool,

  #[builder(default = WalkIgnore::default())]
  pub ignore: WalkIgnore,
}

impl Validator for LintValidator {
  fn validate(&self) -> Result<Vec<Messages>, ValidatorError> {
    let config = ConfigStoreBuilder::from_oxlintrc(true, self.oxlintrc.clone())?
      .build()
      .unwrap();

    let lint = oxc_linter::Linter::new(
      oxc_linter::LintOptions {
        fix: oxc_linter::FixKind::None,
        framework_hints: oxc_linter::FrameworkFlags::empty(),
        // report_unused_directive: Some(AllowWarnDeny::Deny),
        report_unused_directive: Some(oxc_linter::AllowWarnDeny::Allow),
      },
      config,
    );

    let parallel = WalkParallelJs::builder()
      .cwd(self.cwd.clone())
      .ignore(self.ignore.clone())
      .build();

    let res: Vec<Result<Messages, WalkError>> = parallel
      .walk(|path| -> Result<Messages, WalkError> {
        let path = path.clone();

        let named_source = named_source::PathWithSource::try_from(path.clone())?;

        let mut messages = Messages::builder()
          .diagnostics(vec![])
          .source_code(named_source.source_code.clone())
          .source_path(named_source.file_path.clone())
          .build();

        let source_type =
          oxc_span::SourceType::from_path(path).map_err(|e| WalkError::Unknown(e.to_string()))?;

        let allocator = Allocator::default();

        let parser = Parser::new(&allocator, &named_source.source_code, source_type);

        let parser_return = parser.parse();

        if !parser_return.panicked {
          let program = allocator.alloc(&parser_return.program);

          let semantic_builder_return = SemanticBuilder::new()
            .with_check_syntax_error(true)
            .with_cfg(true)
            .build(program);

          let semantic = semantic_builder_return.semantic;

          let semantic = Rc::new(semantic);

          let module_record = Arc::new(oxc_linter::ModuleRecord::new(
            Path::new(&named_source.file_path),
            &parser_return.module_record,
            &semantic,
          ));

          let res = lint.run(Path::new(&named_source.file_path), semantic, module_record);

          for msg in res {
            let error = msg.error;
            let mut diagnostic = MietteDiagnostic::new(error.message.clone());

            if let Some(help) = &error.help {
              diagnostic = diagnostic.with_help(help.to_string());
            }

            let code = error
              .code
              .scope
              .as_ref()
              .map_or(String::new(), |s| s.to_string());

            let number = error
              .code
              .number
              .as_ref()
              .map_or(String::new(), |s| s.to_string());

            diagnostic = diagnostic.with_code(format!("{code}({number})"));

            match error.severity {
              oxc_diagnostics::Severity::Error => {
                diagnostic = diagnostic.with_severity(miette::Severity::Error)
              }
              oxc_diagnostics::Severity::Warning => {
                diagnostic = diagnostic.with_severity(miette::Severity::Warning)
              }
              oxc_diagnostics::Severity::Advice => {
                diagnostic = diagnostic.with_severity(miette::Severity::Advice)
              }
            }

            let labels = error.labels.as_ref().map(|labels| {
              labels
                .iter()
                .map(|label| {
                  let label_text = label.label().map(|s| s.to_string());
                  let miette_label =
                    miette::LabeledSpan::new(label_text, label.offset(), label.len());
                  miette_label
                })
                .collect::<Vec<_>>()
            });

            if let Some(labels) = labels {
              diagnostic = diagnostic.with_labels(labels);
            }

            if let Some(url) = &error.url {
              diagnostic = diagnostic.with_url(url.to_string());
            }

            messages.push(diagnostic);
          }

          Ok(messages)
        } else {
          Ok(messages)
        }
      })
      .map_err(|e| ValidatorError::Unknown(Box::new(e)))?;

    let res = res.into_iter().filter_map(|r| r.ok()).collect::<Vec<_>>();

    Ok(res)
  }
}

impl LintValidator {
  pub fn run(&self) -> Result<Vec<Result<FileDiagnostic, WalkError>>, LintError> {
    let config = ConfigStoreBuilder::from_oxlintrc(true, self.oxlintrc.clone())?.build()?;

    let lint = oxc_linter::Linter::new(
      oxc_linter::LintOptions {
        fix: oxc_linter::FixKind::None,
        framework_hints: oxc_linter::FrameworkFlags::empty(),
        // report_unused_directive: Some(AllowWarnDeny::Deny),
        report_unused_directive: Some(oxc_linter::AllowWarnDeny::Allow),
      },
      config,
    );

    let parallel = WalkParallelJs::builder()
      .cwd(self.cwd.clone())
      .ignore(self.ignore.clone())
      .build();

    let res: Vec<Result<FileDiagnostic, WalkError>> = parallel
      .walk(|path| -> Result<FileDiagnostic, WalkError> {
        let path = path.clone();

        let named_source = named_source::PathWithSource::try_from(path.clone())?;

        let source_type =
          oxc_span::SourceType::from_path(path).map_err(|e| WalkError::Unknown(e.to_string()))?;

        let allocator = Allocator::default();

        let parser = Parser::new(&allocator, &named_source.source_code, source_type);

        let parser_return = parser.parse();

        if !parser_return.panicked {
          let program = allocator.alloc(&parser_return.program);

          let semantic_builder_return = SemanticBuilder::new()
            .with_check_syntax_error(true)
            .with_cfg(true)
            .build(program);

          let semantic = semantic_builder_return.semantic;

          let semantic = Rc::new(semantic);

          let module_record = Arc::new(oxc_linter::ModuleRecord::new(
            Path::new(&named_source.file_path),
            &parser_return.module_record,
            &semantic,
          ));

          let res = lint.run(Path::new(&named_source.file_path), semantic, module_record);

          let diag = FileDiagnostic {
            file_path: named_source.file_path.clone(),
            diagnostics: res.into_iter().map(|msg| msg.error).collect(),
          };

          if self.with_show_report {
            self
              .custom_render_report(&diag, &named_source.source_code)
              .map_err(|e| WalkError::Unknown(e.to_string()))?;
          }

          Ok(diag)
        } else {
          Ok(FileDiagnostic {
            file_path: named_source.file_path,
            diagnostics: vec![],
          })
        }
      })
      .map_err(|e| LintError::Unknown(e.to_string()))?;

    Ok(res)
  }

  pub fn custom_render_report(
    &self,
    diagnostic: &FileDiagnostic,
    source_code: &str,
  ) -> Result<(), LintError> {
    if !diagnostic.diagnostics.is_empty() {
      let handler = oxc_diagnostics::GraphicalReportHandler::new().with_links(true);

      let named_source =
        oxc_diagnostics::NamedSource::new(&diagnostic.file_path, source_code.to_string());

      let mut output = String::with_capacity(1024 * 1024);
      for diag in &diagnostic.diagnostics {
        let diag = diag.clone().with_source_code(named_source.clone());
        handler
          .render_report(&mut output, diag.as_ref())
          .map_err(|e| LintError::Unknown(e.to_string()))?;
      }
      eprintln!("{}", output);
    }

    Ok(())
  }
}

// #[cfg(test)]
// mod tests {
//   use std::error::Error;

//   use crate::{
//     category::Category,
//     common::{environments::EnvironmentFlags, lint_mode::LintMode},
//     config::OxlintrcBuilder,
//     inner::Category20250601Inner,
//     linter_runner::LinterRunner,
//   };

//   #[test]
//   fn test1() -> Result<(), Box<dyn Error>> {
//     let cwd = "/Users/10015448/Git/drawio_ui";

//     let category = Category::V20250601Inner(Category20250601Inner::default());

//     let rc = OxlintrcBuilder::default()
//       .with_category(category)
//       .with_mode(LintMode::Production)
//       .with_envs(EnvironmentFlags::default())
//       .build()
//       .unwrap();

//     let linter_runner = LinterRunner::builder()
//       .cwd(cwd.to_string().into())
//       .with_show_report(true)
//       .oxlintrc(rc)
//       .build();

//     let _ = linter_runner.run();

//     Ok(())
//   }
// }
