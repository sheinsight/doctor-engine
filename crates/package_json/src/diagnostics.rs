use miette::{LabeledSpan, MietteDiagnostic, SourceSpan};

// #[derive(Debug, Diagnostic, thiserror::Error)]
// #[error("Missing required 'packageManager' field in package.json")]
// #[diagnostic(
//   code = "shined(package-json:missing-package-manager)",
//   severity = "error",
//   // label = "Add packageManager field here",
//   help = r#"Add the packageManager field to specify your package manager version, e.g.:  "packageManager": "npm@8.19.2""#
// )]
// pub struct MissingPackageManagerDiagnosticDemo {
//   #[label(primary, "Add packageManager field here")]
//   pub primary_span: SourceSpan,
// }

pub struct MissingPackageManagerDiagnostic;

impl MissingPackageManagerDiagnostic {
  pub fn at(span: impl Into<SourceSpan>) -> MietteDiagnostic {
    MietteDiagnostic::new("Missing required 'packageManager' field in package.json")
      .with_code("doctor:package-json:missing-package-manager")
      .with_severity(miette::Severity::Error)
      .with_label(LabeledSpan::at(span, "Add packageManager field here"))
      .with_help(
        r#"Add the packageManager field to specify your package manager version, e.g.:  "packageManager": "npm@8.19.2""#,
      )
  }
}

pub struct MissingPrivateFieldDiagnostic;

impl MissingPrivateFieldDiagnostic {
  pub fn at(span: impl Into<SourceSpan>) -> MietteDiagnostic {
    MietteDiagnostic::new("Require 'private' field")
      .with_code("shined(package-json-missing-private)")
      .with_severity(miette::Severity::Error)
      .with_label(LabeledSpan::at(span, "private is required"))
      .with_help("Please add a private field to your package.json file")
  }
}

pub struct MissingNameFieldDiagnostic;

impl MissingNameFieldDiagnostic {
  pub fn at(span: impl Into<SourceSpan>) -> MietteDiagnostic {
    MietteDiagnostic::new("Require 'name' field")
      .with_code("shined(package-json-missing-name)")
      .with_severity(miette::Severity::Error)
      .with_label(LabeledSpan::at(span, "name is required"))
      .with_help("Please add a name field to your package.json file")
  }
}

pub struct PrivateNotTrueDiagnostic;

impl PrivateNotTrueDiagnostic {
  pub fn at(span: impl Into<SourceSpan>) -> MietteDiagnostic {
    MietteDiagnostic::new("The 'private' field in package.json must be set to true")
      .with_code("doctor:package-json:private-not-true")
      .with_severity(miette::Severity::Error)
      .with_label(LabeledSpan::at(span, "Set private field to true"))
      .with_help(
        r#"Update your package.json to include: "private": true

This ensures the package cannot be accidentally published to npm."#,
      )
  }
}
