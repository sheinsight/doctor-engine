use doctor_core::{
  Messages, ValidatorError,
  traits::{PathExt, Validator},
};
use jsonc_parser::{CollectOptions, ParseOptions, common::Ranged, parse_to_ast};
use miette::MietteDiagnostic;
use package_json_parser::PackageJsonParser;
use std::{fs::read_to_string, path::Path};
use typed_builder::TypedBuilder;

use crate::diagnostics::DiagnosticFactory;

#[derive(Debug)]
pub enum ValidateName {
  Exist,
  // Value(String),
}

#[derive(Debug)]
pub enum ValidatePackageManager {
  Exist,
  // Npm,
  // Pnpm,
  // Yarn,
}

#[derive(Debug)]
pub enum ValidatePrivate {
  Exist,
  True,
  False,
}

/// validate package.json file
///
/// # Example
///
/// ```rust
/// use doctor_package_json::validator::PackageJsonValidator;
/// use std::path::Path;
/// use doctor_core::{ ValidatorError, traits::Validator};
///
/// let validator = PackageJsonValidator::builder()
///   .config_path("./fixtures/package.json")
///   .build();
///
/// let result = validator.validate().unwrap();
///
/// for msg in result {
///   assert!(!msg.has_error());
/// }
///
/// ```
///
/// # Validate name
///
/// ```rust
/// use doctor_package_json::validator::{PackageJsonValidator,ValidateName};
/// use std::path::Path;
/// use doctor_core::{ ValidatorError, traits::Validator};
///
/// let validator = PackageJsonValidator::builder()
///   .config_path("./fixtures/no_name.json")
///   .with_validate_name(ValidateName::Exist)
///   .build();
///
/// let result = validator.validate().unwrap();
///
/// for msg in result {
///   assert!(msg.has_error());
/// }
///
///
/// ```
///
/// # Validate private
///
/// ```rust
/// use doctor_package_json::validator::{PackageJsonValidator,ValidatePrivate};
/// use std::path::Path;
/// use doctor_core::{ ValidatorError, traits::Validator};
///
/// let validator = PackageJsonValidator::builder()
///   .config_path("./fixtures/no_private.json")
///   .with_validate_private(ValidatePrivate::Exist)
///   .build();
///
/// let result = validator.validate().unwrap();
///
/// for msg in result {
///   assert!(msg.has_error());
/// }
///
/// ```
///
/// # Validate package manager
///
/// ```rust
/// use doctor_package_json::validator::{PackageJsonValidator,ValidatePackageManager};
/// use std::path::Path;  
/// use doctor_core::{ ValidatorError, traits::Validator};
///
/// let validator = PackageJsonValidator::builder()
///   .config_path("./fixtures/no_package_manager.json")
///   .with_validate_package_manager(ValidatePackageManager::Exist)
///   .build();
///
/// let result = validator.validate().unwrap();
///
/// for msg in result {
///   assert!(msg.has_error());
/// }
///
/// ```
#[derive(TypedBuilder)]
pub struct PackageJsonValidator<P>
where
  P: AsRef<Path>,
{
  config_path: P,

  #[builder(default = None, setter(strip_option))]
  with_validate_name: Option<ValidateName>,

  #[builder(default = None, setter(strip_option))]
  with_validate_private: Option<ValidatePrivate>,

  #[builder(default = None, setter(strip_option))]
  with_validate_package_manager: Option<ValidatePackageManager>,
}

impl<P> PackageJsonValidator<P>
where
  P: AsRef<Path>,
{
  fn find_private_value_range(
    &self,
    json_raw: &str,
  ) -> Result<Option<jsonc_parser::common::Range>, ValidatorError> {
    let parse_result = parse_to_ast(
      json_raw,
      &CollectOptions::default(),
      &ParseOptions::default(),
    )
    .unwrap();

    let res = parse_result
      .value
      .and_then(|v| v.as_object().cloned())
      .and_then(|o| o.get("private").cloned())
      .map(|v| v.value.range());
    return Ok(res);
  }

  fn validate_package_manager(
    &self,
    package_json: &PackageJsonParser,
  ) -> Result<Vec<MietteDiagnostic>, ValidatorError> {
    let mut diagnostics = vec![];

    if let Some(_) = &self.with_validate_package_manager {
      let Some(_) = &package_json.package_manager else {
        let len = package_json
          .__raw_source
          .as_ref()
          .map_or(0, |source| source.len());

        diagnostics.push(DiagnosticFactory::at_missing_package_manager(0..len));

        return Ok(diagnostics);
      };
    }

    Ok(diagnostics)
  }

  fn validate_private(
    &self,
    package_json: &PackageJsonParser,
  ) -> Result<Vec<MietteDiagnostic>, ValidatorError> {
    let mut diagnostics = vec![];

    if let Some(validate_private) = &self.with_validate_private {
      if let Some(actual) = package_json.private {
        match validate_private {
          ValidatePrivate::Exist => (),
          ValidatePrivate::True if actual == true => (),
          ValidatePrivate::False if actual == false => (),
          _ => {
            let range = self.find_private_value_range(
              package_json
                .__raw_source
                .as_ref()
                .map_or("", |source| source),
            )?;

            let end = range.map_or(0, |r| r.end);
            let start = range.map_or(0, |r| r.start);
            diagnostics.push(DiagnosticFactory::at_private_not_true(start..end));
            return Ok(diagnostics);
          }
        };
      } else {
        let len = package_json
          .__raw_source
          .as_ref()
          .map_or(0, |source| source.len());
        diagnostics.push(DiagnosticFactory::at_missing_private_field(0..len));
        return Ok(diagnostics);
      }
    }

    Ok(diagnostics)
  }

  fn validate_name(
    &self,
    package_json: &PackageJsonParser,
  ) -> Result<Vec<MietteDiagnostic>, ValidatorError> {
    let mut diagnostics = vec![];

    if let Some(_) = &self.with_validate_name {
      if package_json.name.is_none() {
        let len = package_json
          .__raw_source
          .as_ref()
          .map_or(0, |source| source.len());

        diagnostics.push(DiagnosticFactory::at_missing_name_field(0..len));
      }
    }

    Ok(diagnostics)
  }
}
impl<P> Validator for PackageJsonValidator<P>
where
  P: AsRef<Path>,
{
  /// validate package.json file
  ///
  /// # Example
  ///
  /// ```rust
  /// use doctor_package_json::validator::PackageJsonValidator;
  /// use std::path::Path;
  /// use doctor_core::{ Messages, ValidatorError, traits::Validator};
  ///
  /// let validator = PackageJsonValidator::builder()
  ///   .config_path("./fixtures/package.json")
  ///   .build();
  ///
  /// let result = validator.validate();
  ///
  /// assert!(result.is_ok());
  /// ```
  fn validate(&self) -> Result<Vec<Messages>, ValidatorError> {
    let path = self.config_path.as_ref();

    if !path.exists() {
      return Ok(vec![
        Messages::builder()
          .source_code(String::new())
          .source_path(path.to_string_owned())
          .diagnostics(vec![DiagnosticFactory::at_config_file_not_found(path)])
          .build(),
      ]);
    }

    let raw = read_to_string(path)?;

    let mut messages = Messages::builder()
      .source_code(raw)
      .source_path(path.to_string_owned())
      .diagnostics(vec![])
      .build();

    let package_json = match package_json_parser::PackageJsonParser::parse(path) {
      Ok(package_json) => package_json,
      Err(e) => match e.downcast::<package_json_parser::ErrorKind>() {
        Ok(package_json_parser::ErrorKind::JsonParseError {
          primary_span,
          other_spans,
          ..
        }) => {
          let mut labels = other_spans.clone();

          if let Some(span) = primary_span {
            labels.push(miette::LabeledSpan::at(span, ""));
          }

          let diagnostic = DiagnosticFactory::at_private_type_error(labels);
          messages.diagnostics.push(diagnostic);

          return Ok(vec![messages]);
        }
        Ok(e) => {
          return Err(ValidatorError::PackageJsonParserError(e.into()));
        }
        Err(e) => {
          return Err(ValidatorError::Unknown(e.to_string().into()));
        }
      },
    };

    let diagnostics = self.validate_name(&package_json)?;
    messages.diagnostics.extend(diagnostics.into_iter());

    let diagnostics = self.validate_private(&package_json)?;
    messages.diagnostics.extend(diagnostics.into_iter());

    let diagnostics = self.validate_package_manager(&package_json)?;
    messages.diagnostics.extend(diagnostics.into_iter());

    Ok(vec![messages])
  }

  fn fix(&self) -> Result<Vec<Messages>, ValidatorError> {
    todo!()
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn should_return_missing_name_diagnostic() {
    let result = PackageJsonValidator::builder()
      .config_path("fixtures/no_name.json")
      .with_validate_name(ValidateName::Exist)
      .build()
      .validate()
      .unwrap();

    for msg in result {
      assert!(msg.has_error());
      msg.render();
      assert!(msg.diagnostics.len() == 1);
      assert!(msg.diagnostics[0].code == Some("shined(package-json:missing-name)".into()));
    }
  }

  #[test]
  fn should_return_missing_private_diagnostic() {
    let result = PackageJsonValidator::builder()
      .config_path("fixtures/no_private.json")
      .with_validate_private(ValidatePrivate::True)
      .build()
      .validate()
      .unwrap();

    for msg in result {
      assert!(msg.has_error());
      msg.render();
      assert!(msg.diagnostics.len() == 1);
      assert!(msg.diagnostics[0].code == Some("shined(package-json:missing-private)".into()));
    }
  }

  #[test]
  fn should_return_missing_package_manager_diagnostic() {
    let result = PackageJsonValidator::builder()
      .config_path("fixtures/no_package_manager.json")
      .with_validate_package_manager(ValidatePackageManager::Exist)
      .build()
      .validate()
      .unwrap();

    for msg in result {
      assert!(msg.has_error());
      msg.render();
      assert!(msg.diagnostics.len() == 1);
      assert!(
        msg.diagnostics[0].code == Some("shined(package-json:missing-package-manager)".into())
      );
    }
  }

  #[test]
  fn should_return_private_not_true_diagnostic() {
    let result = PackageJsonValidator::builder()
      .config_path("fixtures/private_false.json")
      .with_validate_private(ValidatePrivate::True)
      .build()
      .validate()
      .unwrap();

    for msg in result {
      assert!(msg.has_error());
      msg.render();
      assert!(msg.diagnostics.len() == 1);
      assert!(msg.diagnostics[0].code == Some("shined(package-json:private-not-true)".into()));
    }
  }

  #[test]
  fn test_validate_private_str() {
    let result = PackageJsonValidator::builder()
      .config_path("fixtures/str_private.json")
      .with_validate_private(ValidatePrivate::True)
      .build()
      .validate()
      .unwrap();

    for msg in result {
      assert!(msg.has_error());
      msg.render();
      assert!(msg.diagnostics.len() == 1);
      assert!(msg.diagnostics[0].code == Some("shined(package-json:private-type-error)".into()));
    }
  }
}
