mod diagnostic;
mod label;
mod location;
mod position;
mod span;

use std::collections::HashMap;

pub use diagnostic::Diagnostic;
use doctor::{
  lint::{
    Category, EnvironmentFlags, LintMode, Linter, config::OxlintrcBuilder,
    inner::Category20250601Inner,
  },
  walk_parallel::{WalkError, WalkParallel, WalkPatterns},
};
pub use label::LabeledLoc;
pub use location::Location;
use napi::Result;
use napi_derive::napi;
use oxc_linter::Oxlintrc;
pub use position::Position;
pub use span::Span;

#[napi]
pub enum NaPiCategory {
  V20250601Inner,
}

#[napi(object)]
pub struct Response {
  pub rc: String,
  pub map: HashMap<String, u32>,
}

#[derive(Debug, Clone)]
#[napi[object]]
pub struct GlobJsArgs {
  pub pattern: Option<String>,
  pub ignore: Option<Vec<String>>,
  pub cwd: String,
}

fn to_napi_error<E: ToString>(e: E) -> napi::Error {
  napi::Error::new(napi::Status::GenericFailure, e.to_string())
}

#[napi]
pub fn inner_debug_lint(
  oxlint_config: String,
  glob_js_args: GlobJsArgs,
) -> Result<Vec<Diagnostic>> {
  let rc: Oxlintrc = serde_json::from_str(&oxlint_config)
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;
  let linter = Linter::from(rc.clone()).with_show_report(true);

  let mut patterns = WalkPatterns::default();

  if let Some(pattern_str) = glob_js_args.pattern {
    patterns = patterns.with_walk(&pattern_str);
  }

  if let Some(ignore) = glob_js_args.ignore {
    patterns = patterns.with_ignore(ignore.as_slice());
  }

  let file_diagnostics = WalkParallel::new(&glob_js_args.cwd)
    .with_patterns(patterns)
    .walk(|path| {
      linter.lint(&path).map_err(|e| WalkError::HandlerError {
        path: path.clone(),
        error: e.to_string(),
      })
    })
    .map_err(to_napi_error)?;

  let mut diags = Vec::new();

  for file_diagnostic in file_diagnostics {
    let file_diagnostic =
      file_diagnostic.map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;

    let f_diags = Diagnostic::from_file_diagnostic(&file_diagnostic);
    diags.extend(f_diags);
  }

  Ok(diags)
}

#[napi]
pub fn inner_lint(glob_js_args: GlobJsArgs, category: NaPiCategory) -> Result<Vec<Diagnostic>> {
  let category = match category {
    NaPiCategory::V20250601Inner => Category::V20250601Inner(Category20250601Inner::default()),
  };

  let mut patterns = WalkPatterns::default();

  if let Some(pattern_str) = glob_js_args.pattern {
    patterns = patterns.with_walk(&pattern_str);
  }

  if let Some(ignore) = glob_js_args.ignore {
    patterns = patterns.with_ignore(ignore.as_slice());
  }

  let rc = OxlintrcBuilder::default()
    .with_category(category)
    .with_mode(LintMode::Production)
    .with_envs(EnvironmentFlags::default())
    .build()
    .map_err(to_napi_error)?;

  let linter = Linter::from(rc.clone()).with_show_report(true);

  let file_diagnostics = WalkParallel::new(&glob_js_args.cwd)
    .with_patterns(patterns)
    .walk(|path| {
      linter.lint(&path).map_err(|e| WalkError::HandlerError {
        path: path.clone(),
        error: e.to_string(),
      })
    })
    .map_err(to_napi_error)?;

  let mut map = HashMap::new();

  let mut diags = Vec::new();

  for file_diagnostic in file_diagnostics {
    let file_diagnostic = file_diagnostic.map_err(to_napi_error)?;

    let f_diags = Diagnostic::from_file_diagnostic(&file_diagnostic);
    diags.extend(f_diags);

    for diag in file_diagnostic.diagnostics {
      let name = match (diag.code.scope.as_ref(), diag.code.number.as_ref()) {
        (None, None) => String::new(),
        (None, Some(number)) => number.to_string(),
        (Some(scope), None) => scope.to_string(),
        (Some(scope), Some(number)) => format!("{scope}/{number}"),
      };
      map.entry(name).and_modify(|count| *count += 1).or_insert(1);
    }
  }

  Ok(diags)
}
