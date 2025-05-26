use std::path::Path;

use doctor_core::traits::Validator;
use doctor_package_json::validator::{
  PackageJsonValidator, ValidateName, ValidatePackageManager, ValidatePrivate,
};

pub fn register_package_json(cwd: impl AsRef<Path>) -> Box<dyn Validator> {
  let validator = PackageJsonValidator::builder()
    .config_path(cwd.as_ref().to_path_buf())
    .with_validate_name(ValidateName::Exist)
    .with_validate_private(ValidatePrivate::True)
    .with_validate_package_manager(ValidatePackageManager::Exist)
    .build();

  Box::new(validator)
}
