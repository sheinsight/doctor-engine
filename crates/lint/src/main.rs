use doctor_lint::{
  Category, EnvironmentFlags, GlobalValue, Globals, LintMode, LintValidator,
  config::OxlintrcBuilder, inner::Category20250601Inner,
};
use doctor_walk_parallel::WalkIgnore;

use std::time::Instant;
fn main() -> anyhow::Result<()> {
  let start_time = Instant::now();

  let cwd = "/Users/10015448/Git/drawio_ui";

  eprintln!("1--->>>");

  let category = Category::V20250601Inner(Category20250601Inner::default());

  eprintln!("2--->>> 规范 ");

  let mut globals = Globals::default();

  globals.insert("a".to_string(), GlobalValue::Writable);

  let rc = OxlintrcBuilder::default()
    .with_category(category)
    .with_globals(globals)
    .with_mode(LintMode::Production)
    .with_envs(EnvironmentFlags::default())
    .build()?;

  std::fs::write("oxlintrc.json", serde_json::to_string_pretty(&rc).unwrap())?;

  eprintln!("3--->>>");

  let ignore = WalkIgnore(vec!["**/node_modules/**".to_string()]);

  let linter_runner = LintValidator::builder()
    .cwd(cwd.to_string().into())
    .ignore(ignore)
    .with_show_report(false)
    .oxlintrc(rc)
    .build();

  eprintln!("4--->>>");

  let res = linter_runner.run();

  eprintln!("5--->>>{:#?}", res.is_err());

  let duration = start_time.elapsed();
  eprintln!("Total execution time: {:?}", duration);

  Ok(())
}
