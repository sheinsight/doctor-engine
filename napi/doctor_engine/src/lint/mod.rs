mod diagnostic;
mod label;
mod location;
mod position;
mod span;

use std::collections::HashMap;

pub use diagnostic::Diagnostic;
use doctor::{
  ext::PathExt,
  lint::{
    Category, EnvironmentFlags, GlobalValue, Globals, LintMode, LinterRunner,
    config::OxlintrcBuilder, inner::Category20250601Inner,
  },
  walk_parallel::WalkIgnore,
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
  pub ignore: Option<Vec<String>>,
  pub cwd: String,
  pub verbose: Option<bool>,
  pub absolute: Option<bool>,
  pub globals: Option<HashMap<String, String>>,
}

fn to_napi_error<E: ToString>(e: E) -> napi::Error {
  napi::Error::new(napi::Status::GenericFailure, e.to_string())
}

#[napi]
pub async fn inner_debug_lint(
  oxlint_config: String,
  glob_js_args: GlobJsArgs,
) -> Result<Vec<Diagnostic>> {
  let rc: Oxlintrc = serde_json::from_str(&oxlint_config)
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;

  let mut ignore = WalkIgnore::default();

  if let Some(ignore_patterns) = glob_js_args.ignore {
    ignore.extend(ignore_patterns);
  }

  let linter_runner = LinterRunner::builder()
    .cwd(glob_js_args.cwd.clone().into())
    .ignore(ignore)
    .with_show_report(glob_js_args.verbose.unwrap_or(false))
    .oxlintrc(rc)
    .build();

  let file_diagnostics = linter_runner.run().map_err(to_napi_error)?;

  let mut diags = Vec::new();

  for file_diagnostic in file_diagnostics {
    let file_diagnostic =
      file_diagnostic.map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;

    let source_code = std::fs::read_to_string(&file_diagnostic.file_path)
      .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;

    let relative_path =
      if let Some(r) = pathdiff::diff_paths(&file_diagnostic.file_path, &glob_js_args.cwd) {
        r.to_string_owned()
      } else {
        file_diagnostic.file_path.to_string()
      };

    let f_diags = Diagnostic::from_file_diagnostic(&file_diagnostic, &relative_path, &source_code);
    diags.extend(f_diags);
  }

  Ok(diags)
}

#[napi]
pub async fn inner_lint(
  glob_js_args: GlobJsArgs,
  category: NaPiCategory,
) -> Result<Vec<Diagnostic>> {
  let category = match category {
    NaPiCategory::V20250601Inner => Category::V20250601Inner(Category20250601Inner::default()),
  };

  let mut ignore = WalkIgnore::default();
  if let Some(ignore_patterns) = glob_js_args.ignore {
    ignore.extend(ignore_patterns);
  }

  let mut globals = Globals::default();

  if let Some(g) = glob_js_args.globals {
    for (key, value) in g {
      if let Ok(value) = value.parse::<GlobalValue>() {
        globals.insert(key.into(), value);
      } else {
        return Err(napi::Error::new(
          napi::Status::InvalidArg,
          format!("invalid global value: {value} , Only `readonly` or `writable` is allowed"),
        ));
      }
    }
  }

  let rc = OxlintrcBuilder::default()
    .with_category(category)
    .with_mode(LintMode::Production)
    .with_envs(EnvironmentFlags::default())
    .with_globals(globals)
    .build()
    .map_err(to_napi_error)?;

  let linter_runner = LinterRunner::builder()
    .cwd(glob_js_args.cwd.clone().into())
    .ignore(ignore)
    .with_show_report(glob_js_args.verbose.unwrap_or(false))
    .oxlintrc(rc)
    .build();

  let file_diagnostics = linter_runner.run().map_err(to_napi_error)?;

  let mut map = HashMap::new();

  let mut diags = Vec::new();

  for file_diagnostic in file_diagnostics {
    let file_diagnostic = file_diagnostic.map_err(to_napi_error)?;

    let source_code = std::fs::read_to_string(&file_diagnostic.file_path)?;

    let relative_path = if glob_js_args.absolute.unwrap_or(false) {
      file_diagnostic.file_path.to_string()
    } else if let Some(r) = pathdiff::diff_paths(&file_diagnostic.file_path, &glob_js_args.cwd) {
      r.to_string_owned()
    } else {
      file_diagnostic.file_path.to_string()
    };

    let f_diags = Diagnostic::from_file_diagnostic(&file_diagnostic, &relative_path, &source_code);
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
