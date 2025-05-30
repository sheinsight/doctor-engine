use std::path::Path;

use biome_json_parser::{JsonParserOptions, parse_json};
use biome_rowan::TextRange;
use doctor_core::{
  Messages, ValidatorError,
  traits::{PathExt, Validator},
};
use miette::MietteDiagnostic;
use package_json_parser::PackageJsonParser;
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
  fn find_private_range(&self, json_raw: &str) -> Result<Option<TextRange>, ValidatorError> {
    let parse = parse_json(&json_raw, JsonParserOptions::default());

    let root = parse.tree();

    let root_any_json_value = root.value()?;

    let root = root_any_json_value.as_json_object_value().unwrap();

    for member in root.json_member_list() {
      let member = member?;

      let name = member.name()?;

      if name.inner_string_text()? == "private" {
        let value = member.value().unwrap();
        let value = value.as_json_boolean_value().unwrap();

        let value_range = value.value_token()?.text_trimmed_range();

        return Ok(Some(value_range));
      }
    }

    Ok(None)
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
            let range = self.find_private_range(
              package_json
                .__raw_source
                .as_ref()
                .map_or("", |source| source),
            )?;
            let end = range.unwrap_or_default().end().into();
            let start = range.unwrap_or_default().start().into();
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

    let package_json = package_json_parser::PackageJsonParser::parse(path)?;

    let mut messages = Messages::builder()
      .source_code(package_json.__raw_source.clone().unwrap_or_default())
      .source_path(path.to_string_owned())
      .diagnostics(vec![])
      .build();

    let diagnostics = self.validate_name(&package_json)?;
    messages.diagnostics.extend(diagnostics.into_iter());

    let diagnostics = self.validate_private(&package_json)?;
    messages.diagnostics.extend(diagnostics.into_iter());

    let diagnostics = self.validate_package_manager(&package_json)?;
    messages.diagnostics.extend(diagnostics.into_iter());

    Ok(vec![messages])
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
      .validate();

    println!("{:?}", result);

    assert!(!result.is_ok());

    assert!(matches!(result, Err(ValidatorError::IoError(_))));
  }
}
