use std::collections::HashMap;

use doctor::{
  lint::{
    Category, EnvironmentFlags, LintMode, Linter, config::OxlintrcBuilder,
    inner::Category20250601Inner,
  },
  walk_parallel::{WalkParallel, error::WalkError, walk_patterns::WalkPatterns},
};
use napi_derive::napi;

#[napi]
pub enum NaPiCategory {
  V20250601Inner,
}

#[napi]
pub fn inner_lint(cwd: String, category: NaPiCategory) {
  let category = match category {
    NaPiCategory::V20250601Inner => Category::V20250601Inner(Category20250601Inner::default()),
  };

  println!("cwd: {:?}", cwd);
  println!("category: {:?}", category);

  let rc = OxlintrcBuilder::default()
    .with_category(category)
    .with_mode(LintMode::Production)
    .with_envs(EnvironmentFlags::default())
    .build()
    .unwrap();

  let linter = Linter::from(rc).with_show_report(true);

  let file_diagnostics = WalkParallel::new(&cwd)
    .with_patterns(WalkPatterns::default())
    .walk(|path| {
      linter.lint(&path).map_err(|e| WalkError::HandlerError {
        path: path.clone(),
        error: e.to_string(),
      })
    })
    .unwrap();

  let mut map = HashMap::new();

  for file_diagnostic in file_diagnostics {
    file_diagnostic
      .unwrap()
      .diagnostics
      .iter()
      .for_each(|diag| {
        let name = match (diag.code.scope.as_ref(), diag.code.number.as_ref()) {
          (None, None) => String::new(),
          (None, Some(number)) => number.to_string(),
          (Some(scope), None) => scope.to_string(),
          (Some(scope), Some(number)) => format!("{scope}/{number}"),
        };
        map.entry(name).and_modify(|count| *count += 1).or_insert(1);
      });
  }
}
