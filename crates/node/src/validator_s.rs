use std::{fs::read_to_string, path::Path};

use doctor_ext::{PathExt, Validator};
use lazy_regex::regex;
use typed_builder::TypedBuilder;

use crate::error::{InvalidErr, NodeVersionValidatorError, NotFoundErr, UnknowErr};

#[derive(TypedBuilder)]
pub struct NodeVersionValidator<P>
where
  P: AsRef<Path>,
{
  config_path: P,
}

impl<P> Validator for NodeVersionValidator<P>
where
  P: AsRef<Path>,
{
  type Error = NodeVersionValidatorError;

  fn validate(&self) -> Result<(), Self::Error> {
    let path = self.config_path.as_ref();

    if !path.exists() {
      return NotFoundErr::builder()
        .config_path(path.to_string_owned())
        .build()
        .into();
    }

    let version =
      read_to_string(path).map_err(|e| UnknowErr::builder().source(Box::new(e)).build().into())?;

    let version = version.trim();

    let r = regex!(r#"^\d+\.\d+\.\d+$"#);

    if !r.is_match(&version) {
      return InvalidErr::builder()
        .config_path(path.to_string_owned())
        .version(version.to_string())
        .build()
        .into();
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_validate_node_version_file_invalid() {
    let res = NodeVersionValidator::builder()
      .config_path("./fixtures/.invalid")
      .build()
      .validate();

    assert!(matches!(res, Err(NodeVersionValidatorError::InvalidErr(_))));
  }

  #[test]
  fn test_validate_node_version_file_not_found() {
    let res = NodeVersionValidator::builder()
      .config_path("./fixtures/.not-found")
      .build()
      .validate();

    assert!(matches!(
      res,
      Err(NodeVersionValidatorError::NotFoundErr(_))
    ));
  }

  #[test]
  fn test_validate_node_version_file_empty() {
    let res = NodeVersionValidator::builder()
      .config_path("./fixtures/.empty")
      .build()
      .validate();

    assert!(matches!(res, Err(NodeVersionValidatorError::InvalidErr(_))));
  }

  #[test]
  fn test_validate_node_version_file_success() {
    let res = NodeVersionValidator::builder()
      .config_path("./fixtures/.success")
      .build()
      .validate();

    assert!(res.is_ok());
  }
}
