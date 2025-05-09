use std::path::Path;

use miette::{LabeledSpan, MietteDiagnostic, SourceSpan};

pub struct ConfigFileNotFoundDiagnostic;

impl ConfigFileNotFoundDiagnostic {
  pub fn at<P: AsRef<Path>>(path: P) -> MietteDiagnostic {
    MietteDiagnostic::new(format!(
      "Config file {:?} not found",
      path.as_ref().display()
    ))
    .with_code("shined(package-json:config-file-not-found)")
    .with_severity(miette::Severity::Error)
    .with_help("Please add a config file to your project")
  }
}

pub struct MissingPackageManagerDiagnostic;

impl MissingPackageManagerDiagnostic {
  pub fn at(span: impl Into<SourceSpan>) -> MietteDiagnostic {
    MietteDiagnostic::new("Missing required 'packageManager' field in package.json")
      .with_code("shined(package-json:missing-package-manager)")
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
      .with_code("shined(package-json-private-not-true)")
      .with_severity(miette::Severity::Error)
      .with_label(LabeledSpan::at(span, "Set private field to true"))
      .with_help(
        r#"Update your package.json to include: "private": true

This ensures the package cannot be accidentally published to npm."#,
      )
  }
}
