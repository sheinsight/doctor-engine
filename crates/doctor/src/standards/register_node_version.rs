use std::path::Path;

use doctor_core::traits::Validator;
use doctor_node::validator::NodeVersionValidator;

pub fn register_node_version(cwd: impl AsRef<Path>) -> Box<dyn Validator> {
  let validator = NodeVersionValidator::builder()
    .config_path(cwd.as_ref().to_path_buf())
    .with_valid_range(vec!["^18.12.0", "^20.9.0", "^22.11.0"])
    .build();

  Box::new(validator)
}
