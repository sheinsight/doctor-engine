use std::{fs::read_to_string, path::PathBuf};

use doctor_core::{Ignore, Messages, hack_source_type_from_path, traits::Validator};
use doctor_walk::{WalkError, WalkParallelJs};
use miette::MietteDiagnostic;
use oxc::{allocator::Allocator, parser::Parser};
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct SyntaxValidator {
  cwd: PathBuf,
  #[builder(default = Ignore::default())]
  pub ignore: Ignore,
}

impl Validator for SyntaxValidator {
  fn validate(&self) -> Result<Vec<doctor_core::Messages>, doctor_core::ValidatorError> {
    let parallel = WalkParallelJs::builder()
      .cwd(self.cwd.clone())
      .ignore(self.ignore.clone())
      .build();

    let res = parallel
      .walk(|path| -> Result<Messages, WalkError> {
        let path = path.clone();

        let source_code = read_to_string(&path)?;

        let allocator = Allocator::default();

        let source_type = hack_source_type_from_path(&path);

        let parser = Parser::new(&allocator, &source_code, source_type);

        let parse = parser.parse();

        if !parse.errors.is_empty() {
          let messages = parse
            .errors
            .into_iter()
            .map(|item| doctor_core::Diagnostic::from(item).into())
            .collect::<Vec<MietteDiagnostic>>();

          return Ok(
            Messages::builder()
              .source_code(source_code.clone())
              .source_path(path.display().to_string())
              .diagnostics(messages)
              .build(),
          );
        }

        Ok(
          Messages::builder()
            .source_code(source_code.clone())
            .source_path(path.display().to_string())
            .diagnostics(vec![])
            .build(),
        )
      })
      .map_err(|e| doctor_core::ValidatorError::Unknown(Box::new(e)))?;

    let res = res.into_iter().filter_map(|r| r.ok()).collect::<Vec<_>>();

    Ok(res)
  }

  fn fix(&self) -> Result<Vec<Messages>, doctor_core::ValidatorError> {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_syntax_validator() {
    let validator = SyntaxValidator::builder()
      .cwd(PathBuf::from("./fixtures"))
      .build();
    let res = validator.validate().unwrap();
    for item in res {
      item.render();
    }
  }
}
