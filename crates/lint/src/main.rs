use std::{collections::HashMap, env::current_dir};

use lint::{
  Linter, category_getter::Category, inner::v2025_06_01::category::Category20250601Inner,
  oxlint_rc_builder::OxlintrcBuilder, react_config::ReactConfig,
  typescript_config::TypescriptConfig,
};

use walk_parallel::{WalkParallel, walk_patterns::WalkPatterns};

fn main() {
  let category = Category::V20250601Inner(
    Category20250601Inner::default()
      .with_react(ReactConfig::default())
      .with_typescript(TypescriptConfig::default()),
  );

  let rc = OxlintrcBuilder::default()
    .with_category(category)
    .build()
    .unwrap();

  let rc_str = serde_json::to_string_pretty(&rc).unwrap();

  std::fs::write(".oxlintrc.json", rc_str).unwrap();

  let linter: Linter = rc.into();

  let linter = linter.with_show_report(true);

  let cwd = current_dir().unwrap().join("examples");

  println!("{:?}", cwd);

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
