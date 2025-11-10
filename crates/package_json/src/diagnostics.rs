use std::path::Path;

use miette::{LabeledSpan, MietteDiagnostic, SourceSpan, diagnostic};

pub struct DiagnosticFactory;

impl DiagnosticFactory {
  pub fn at_config_file_not_found<P: AsRef<Path>>(path: P) -> MietteDiagnostic {
    let file = path.as_ref();
    let dir = file.parent().map_or(Path::new(""), |p| p);
    let code = "shined(package-json:config-file-not-found)";
    let help = format!(
      "Please add a package.json file to your project at {}",
      dir.display().to_string()
    );
    diagnostic!(
      severity = miette::Severity::Error,
      code = code,
      help = help,
      "Config file was not found: {}",
      file.display().to_string()
    )
  }

  pub fn at_library_version_not_allowed(span: impl Into<SourceSpan>) -> MietteDiagnostic {
    let code = "shined(package-json:library-version-not-allowed)";
    let help = "The library version is not allowed";
    let labels = vec![LabeledSpan::at(span, "Library version is not allowed")];
    diagnostic!(
      severity = miette::Severity::Error,
      code = code,
      help = help,
      labels = labels,
      "The library version is not allowed",
    )
  }

  pub fn at_missing_package_manager(span: impl Into<SourceSpan>) -> MietteDiagnostic {
    let code = "shined(package-json:missing-package-manager)";
    let help = r#"Add packageManager field to your package.json file. 
    
e.g.:  "packageManager": "npm@8.19.2""#;
    let labels = vec![LabeledSpan::at(span, "Add packageManager field here")];

    diagnostic!(
      severity = miette::Severity::Error,
      code = code,
      help = help,
      labels = labels,
      "Missing 'packageManager' field",
    )
  }

  pub fn at_missing_private_field(span: impl Into<SourceSpan>) -> MietteDiagnostic {
    let code = "shined(package-json:missing-private)";
    let help = r#"Add private field to your package.json file.
    
e.g.: "private": true"#;
    let labels = vec![LabeledSpan::at(span, "Add private field here")];

    diagnostic!(
      severity = miette::Severity::Error,
      code = code,
      help = help,
      labels = labels,
      "Missing 'private' field",
    )
  }

  pub fn at_missing_name_field(span: impl Into<SourceSpan>) -> MietteDiagnostic {
    let code = "shined(package-json:missing-name)";
    let help = r#"Add name field to your package.json file.
    
e.g.: "name": "my-package""#;
    let labels = vec![LabeledSpan::at(span, "Add name field here")];

    diagnostic!(
      severity = miette::Severity::Error,
      code = code,
      help = help,
      labels = labels,
      "Missing 'name' field",
    )
  }

  pub fn at_private_not_true(span: impl Into<SourceSpan>) -> MietteDiagnostic {
    let code = "shined(package-json:private-not-true)";
    let help = r#"Update your package.json to include: "private": true

This ensures the package cannot be accidentally published to npm."#;
    let labels = vec![LabeledSpan::at(span, "Set private field to true")];

    diagnostic!(
      severity = miette::Severity::Error,
      code = code,
      help = help,
      labels = labels,
      "The 'private' field in package.json must be set to true",
    )
  }

  pub fn at_private_type_error(labels: Vec<LabeledSpan>) -> MietteDiagnostic {
    let message = "The 'private' field in package.json must be a boolean";
    let b = MietteDiagnostic::new(message)
      .with_labels(labels)
      .with_help(
        r#"Update your package.json to include: "private": true
This ensures the package cannot be accidentally published to npm."#,
      )
      .with_code("shined(package-json:private-type-error)")
      .with_severity(miette::Severity::Error);
    b
  }
}
