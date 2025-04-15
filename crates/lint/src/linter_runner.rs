use std::{
  path::{Path, PathBuf},
  rc::Rc,
  sync::Arc,
};

use doctor_walk_parallel::{WalkError, WalkParallel, WalkPatterns};
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
pub struct LinterRunner {
  cwd: PathBuf,
  walk_patterns: WalkPatterns,
  oxlintrc: Oxlintrc,

  #[builder(default = false)]
  with_show_report: bool,
}

impl LinterRunner {
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

    let parallel = WalkParallel::builder()
      .cwd(self.cwd.as_ref())
      .patterns(self.walk_patterns.clone())
      .build();

    let res: Vec<Result<FileDiagnostic, WalkError>> = parallel
      .walk(|path| -> Result<FileDiagnostic, WalkError> {
        let path = path.clone();

        let named_source = named_source::PathWithSource::try_from(path.clone())?;

        let source_type =
          oxc_span::SourceType::from_path(path).map_err(|e| WalkError::Unknown(e.into()))?;

        let allocator = Allocator::default();

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

        if self.with_show_report {
          self.custom_render_report(&diag);
        }

        Ok(diag)
      })
      .unwrap();

    Ok(res)
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
