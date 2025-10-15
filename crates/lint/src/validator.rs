use std::{
  fs,
  path::{Path, PathBuf},
  sync::Arc,
};

use doctor_core::{Ignore, Messages, ValidatorError, traits::Validator};
use doctor_walk::{WalkError, WalkParallelJs};
use oxc::{
  allocator::Allocator,
  diagnostics::{GraphicalReportHandler, NamedSource},
  parser::Parser,
  semantic::SemanticBuilder,
  span::SourceType,
};
use oxc_linter::{
  AllowWarnDeny, ConfigStore, ConfigStoreBuilder, ContextSubHost, ExternalPluginStore, FixKind,
  FrameworkFlags, LintOptions, Linter, Message, Oxlintrc, PossibleFixes,
};
use rustc_hash::FxHashMap;
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

  #[builder(default = Ignore::default())]
  pub ignore: Ignore,
}

impl LintValidator {
  fn apply_fixes(&self, source_code: &str, messages: &[Message]) -> Option<String> {
    let mut fixes = Vec::new();

    // 收集所有修复
    for msg in messages {
      match &msg.fixes {
        PossibleFixes::None => {}
        PossibleFixes::Single(fix) => fixes.push(fix.clone()),
        PossibleFixes::Multiple(fs) => fixes.extend(fs.iter().cloned()),
      }
    }

    if fixes.is_empty() {
      return None; // 没有修复
    }

    // 按 span.start 倒序排序（从后往前应用，避免位置偏移）
    fixes.sort_by(|a, b| b.span.start.cmp(&a.span.start));

    let mut result = source_code.to_string();
    for fix in fixes {
      let start = fix.span.start as usize;
      let end = fix.span.end as usize;
      result.replace_range(start..end, &fix.content);
    }

    Some(result)
  }
}

impl Validator for LintValidator {
  fn validate(&self) -> Result<Vec<Messages>, ValidatorError> {
    let mut external_plugin_store = ExternalPluginStore::default();

    let config = ConfigStoreBuilder::from_oxlintrc(
      true,
      self.oxlintrc.clone(),
      None,
      &mut external_plugin_store,
    )?
    .build(&external_plugin_store)?;

    let config_store =
      oxc_linter::ConfigStore::new(config, FxHashMap::default(), external_plugin_store);

    let lint = Linter::new(
      oxc_linter::LintOptions {
        fix: oxc_linter::FixKind::All,
        framework_hints: oxc_linter::FrameworkFlags::empty(),
        // report_unused_directive: Some(AllowWarnDeny::Deny),
        report_unused_directive: Some(oxc_linter::AllowWarnDeny::Allow),
      },
      config_store,
      None,
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
          SourceType::from_path(path).map_err(|e| WalkError::Unknown(e.to_string()))?;

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

          let module_record = Arc::new(oxc_linter::ModuleRecord::new(
            Path::new(&named_source.file_path),
            &parser_return.module_record,
            &semantic,
          ));

          let context_sub_hosts = ContextSubHost::new(semantic, module_record, 0);

          let res = lint.run(
            Path::new(&named_source.file_path),
            vec![context_sub_hosts],
            &allocator,
          );

          for msg in res {
            let error = msg.error;

            let diagnostic = doctor_core::Diagnostic::from(error);

            messages.push(diagnostic.into());
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
    let mut external_plugin_store = ExternalPluginStore::default();
    let config = ConfigStoreBuilder::from_oxlintrc(
      true,
      self.oxlintrc.clone(),
      None,
      &mut external_plugin_store,
    )?
    .build(&external_plugin_store)?;

    let config_store = ConfigStore::new(config, FxHashMap::default(), external_plugin_store);

    let lint = Linter::new(
      LintOptions {
        fix: FixKind::All,
        framework_hints: FrameworkFlags::empty(),
        // report_unused_directive: Some(AllowWarnDeny::Deny),
        report_unused_directive: Some(AllowWarnDeny::Allow),
      },
      config_store,
      None,
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
          SourceType::from_path(path).map_err(|e| WalkError::Unknown(e.to_string()))?;

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

          let module_record = Arc::new(oxc_linter::ModuleRecord::new(
            Path::new(&named_source.file_path),
            &parser_return.module_record,
            &semantic,
          ));

          let context_sub_hosts = ContextSubHost::new(semantic, module_record, 0);

          let res = lint.run(
            Path::new(&named_source.file_path),
            vec![context_sub_hosts],
            &allocator,
          );

          // TODO: 应用修复
          if !res.is_empty() {
            let source_code = self.apply_fixes(&named_source.source_code, &res);
            if let Some(source_code) = source_code {
              println!("source_code: {}", source_code);
              fs::write(&named_source.file_path, source_code).unwrap();
            }
          }

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
      let handler = GraphicalReportHandler::new().with_links(true);

      let named_source = NamedSource::new(&diagnostic.file_path, source_code.to_string());

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
