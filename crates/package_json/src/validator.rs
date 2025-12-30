use crate::diagnostics::DiagnosticFactory;
use doctor_core::{
  Messages, ValidatorError,
  traits::{PathExt, Validator},
};
use jsonc_parser::{CollectOptions, ParseOptions, common::Ranged, parse_to_ast};
use miette::MietteDiagnostic;
use node_semver::Range;
use package_json_parser::{FxHashMap, PackageJsonParser};
use std::{fs::read_to_string, path::Path};
use typed_builder::TypedBuilder;

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

  #[builder(default = None, setter(strip_option))]
  with_validate_shineout_version: Option<bool>,
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

  fn validate_library_version(
    &self,
    package_json: &PackageJsonParser,
  ) -> Result<Vec<MietteDiagnostic>, ValidatorError> {
    let mut diagnostics = vec![];

    let parse_result = parse_to_ast(
      package_json
        .__raw_source
        .as_ref()
        .map_or("", |source| source),
      &CollectOptions::default(),
      &ParseOptions::default(),
    )
    .unwrap();

    let root_object = parse_result
      .value
      .as_ref()
      .and_then(|v| v.as_object().cloned());

    // 辅助闭包：验证特定的依赖项字段
    let mut validate_deps =
      |field_name: &str, deps: &FxHashMap<String, String>| -> Result<(), ValidatorError> {
        for (name, value) in deps {
          if ["*", "http", "https"].iter().any(|v| value.starts_with(v)) {
            let range = root_object
              .as_ref()
              .and_then(|o| o.get(field_name).cloned())
              .and_then(|v| v.value.as_object().cloned())
              .and_then(|v| v.get(name).cloned())
              .map(|v| v.value.range())
              .unwrap();

            diagnostics.push(DiagnosticFactory::at_library_version_not_allowed(
              range.start..range.end,
            ));
          }

          if let Some(_validate_shineout_version) = &self.with_validate_shineout_version {
            if name == "shineout" {
              let target_range = Range::parse(">=3.0.0 <3.9.0")?;

              let current_range = Range::parse(value)?;

              let intersect = target_range.intersect(&current_range);

              if intersect.is_some() && !value.contains("fix.") && value != "3.9.0" {
                let range = root_object
                  .as_ref()
                  .and_then(|o| o.get(field_name).cloned())
                  .and_then(|v| v.value.as_object().cloned())
                  .and_then(|v| v.get(name).cloned())
                  .map(|v| v.value.range())
                  .unwrap();
                diagnostics.push(DiagnosticFactory::at_wrong_shineout_version(
                  range.start..range.end,
                  &format!(r##"{value}-fix.[version number]"##),
                ));
              }
            }
          }
        }
        Ok(())
      };

    // 统一处理各种依赖项
    if let Some(dependencies) = &package_json.dependencies {
      validate_deps("dependencies", dependencies)?;
    }

    if let Some(dev_dependencies) = &package_json.dev_dependencies {
      validate_deps("devDependencies", dev_dependencies)?;
    }

    if let Some(peer_dependencies) = &package_json.peer_dependencies {
      validate_deps("peerDependencies", peer_dependencies)?;
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

    let diagnostics = self.validate_library_version(&package_json)?;
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
  fn should_return_library_version_not_allowed_diagnostic() {
    let result = PackageJsonValidator::builder()
      .config_path("fixtures/library_version_not_allowed.json")
      .build()
      .validate()
      .unwrap();
    for msg in result {
      assert!(msg.has_error());
      msg.render();
      assert!(msg.diagnostics.len() == 3);
      assert!(
        msg.diagnostics[0].code == Some("shined(package-json:library-version-not-allowed)".into())
      );
    }
  }

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

  #[test]
  fn test_validate_shineout_version_fix() {
    let result = PackageJsonValidator::builder()
      .config_path("fixtures/shineout_3_fix.json")
      .with_validate_shineout_version(true)
      .build()
      .validate()
      .unwrap();

    for msg in result {
      println!("{:#?}", msg.diagnostics);
      assert!(msg.has_error());
      msg.render();
      assert!(msg.diagnostics.len() == 1);
      assert!(
        msg.diagnostics[0].code == Some("shined(package-json:library-version-not-allowed)".into())
      );
    }
  }

  #[test]
  fn test_validate_shineout_version_ne_fix() {
    let result = PackageJsonValidator::builder()
      .config_path("fixtures/shineout_2_ne_fix.json")
      .with_validate_shineout_version(true)
      .build()
      .validate()
      .unwrap();

    for msg in result {
      assert!(!msg.has_error());
      msg.render();
    }
  }

  #[test]
  fn test_validate_shineout_version_3_9_ne_fix() {
    let result = PackageJsonValidator::builder()
      .config_path("fixtures/shineout_3_9_ne_fix.json")
      .with_validate_shineout_version(true)
      .build()
      .validate()
      .unwrap();

    for msg in result {
      assert!(!msg.has_error());
      msg.render();
    }
  }

  #[test]
  fn test_validate_shineout_version_3_fix_2() {
    let result = PackageJsonValidator::builder()
      .config_path("fixtures/shineout_3_fix_2.json")
      .with_validate_shineout_version(true)
      .build()
      .validate()
      .unwrap();

    for msg in result {
      assert!(!msg.has_error());
      msg.render();
    }
  }
}
