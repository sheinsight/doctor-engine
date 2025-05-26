use std::path::Path;

use base64::{Engine, engine::general_purpose::STANDARD};
use doctor_core::traits::Validator;
use doctor_npmrc::validator::NpmrcValidator;

const ENCODED: [&str; 36] = [
  "a", "H", "R", "0", "c", "H", "M", "6", "L", "y", "9", "u", "c", "G", "1", "q", "c", "y", "5",
  "z", "a", "G", "V", "p", "b", "m", "N", "v", "c", "n", "A", "u", "Y", "2", "4", "=",
];

fn decode_to_str(encoded: &str) -> String {
  let decoded = STANDARD.decode(encoded).unwrap();
  String::from_utf8(decoded).unwrap()
}

pub fn register_npmrc(cwd: impl AsRef<Path>) -> Box<dyn Validator> {
  let text = decode_to_str(ENCODED.join("").as_str());

  let validator = NpmrcValidator::builder()
    .config_path(cwd.as_ref().to_path_buf())
    .with_registry_url(vec![text])
    .build();

  Box::new(validator)
}
