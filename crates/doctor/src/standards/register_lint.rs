use std::path::Path;

use doctor_core::traits::Validator;
use doctor_lint::{
  Category, EnvironmentFlags, LintMode, LintValidator, OxlintrcBuilder, Sfconfig,
  inner::Category20250601Inner,
};

pub fn register_lint(cwd: impl AsRef<Path>, sfconfig: Sfconfig) -> Box<dyn Validator> {
  let category = Category::V20250601Inner(Category20250601Inner::default());
  let rc = OxlintrcBuilder::default()
    .with_category(category)
    .with_globals(sfconfig.globals)
    .with_mode(LintMode::Production)
    .with_envs(EnvironmentFlags::default())
    .build()
    .unwrap();
  let validator = LintValidator::builder()
    .cwd(cwd.as_ref().to_path_buf())
    .ignore(sfconfig.ignore)
    .with_show_report(false)
    .oxlintrc(rc)
    .build();

  Box::new(validator)
}
