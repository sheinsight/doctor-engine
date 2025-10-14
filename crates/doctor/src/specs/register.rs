use std::path::Path;

use base64::{Engine, engine::general_purpose::STANDARD};
use doctor_core::traits::Validator;
use doctor_lint::{
  Category, EnvironmentFlags, LintMode, LintValidator, OxlintrcBuilder, Sfconfig,
  inner::Category20250601Inner,
};
use doctor_node::validator::NodeVersionValidator;
use doctor_npmrc::validator::NpmrcValidator;
use doctor_package_json::validator::{
  PackageJsonValidator, ValidateName, ValidatePackageManager, ValidatePrivate,
};
use doctor_syntax::SyntaxValidator;

const ENCODED: [&str; 36] = [
  "a", "H", "R", "0", "c", "H", "M", "6", "L", "y", "9", "u", "c", "G", "1", "q", "c", "y", "5",
  "z", "a", "G", "V", "p", "b", "m", "N", "v", "c", "n", "A", "u", "Y", "2", "4", "=",
];

fn decode_to_str(encoded: &str) -> String {
  let decoded = STANDARD.decode(encoded).unwrap();
  String::from_utf8(decoded).unwrap()
}

pub fn register_lint(cwd: impl AsRef<Path>, sfconfig: Sfconfig) -> Box<dyn Validator> {
  let category = Category::V20250601Inner(Category20250601Inner::default());

  Category20250601Inner::builder()
    .globals(sfconfig.globals)
    .mode(LintMode::Production)
    .envs(EnvironmentFlags::default())
    .build();

  let rc = OxlintrcBuilder::default().with_category(category).build();

  let validator = LintValidator::builder()
    .cwd(cwd.as_ref().to_path_buf())
    .ignore(sfconfig.ignore)
    .with_show_report(false)
    .oxlintrc(rc)
    .build();

  Box::new(validator)
}

pub fn register_node_version(cwd: impl AsRef<Path>) -> Box<dyn Validator> {
  let validator = NodeVersionValidator::builder()
    .config_path(cwd.as_ref().to_path_buf())
    .with_valid_range(vec!["^18.12.0", "^20.9.0", "^22.11.0", "^24.10.0"])
    .build();

  Box::new(validator)
}

pub fn register_npmrc(cwd: impl AsRef<Path>) -> Box<dyn Validator> {
  let text = decode_to_str(ENCODED.join("").as_str());
  let validator = NpmrcValidator::builder()
    .config_path(cwd.as_ref().to_path_buf())
    .with_registry_url(vec![text])
    .build();

  Box::new(validator)
}

pub fn register_package_json(cwd: impl AsRef<Path>) -> Box<dyn Validator> {
  let validator = PackageJsonValidator::builder()
    .config_path(cwd.as_ref().to_path_buf())
    .with_validate_name(ValidateName::Exist)
    .with_validate_private(ValidatePrivate::True)
    .with_validate_package_manager(ValidatePackageManager::Exist)
    .build();

  Box::new(validator)
}

pub fn register_syntax(cwd: impl AsRef<Path>, sfconfig: Sfconfig) -> Box<dyn Validator> {
  let validator = SyntaxValidator::builder()
    .cwd(cwd.as_ref().to_path_buf())
    .ignore(sfconfig.ignore)
    .build();
  Box::new(validator)
}
