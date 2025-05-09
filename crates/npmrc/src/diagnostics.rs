use std::path::Path;

use miette::{LabeledSpan, MietteDiagnostic, SourceSpan};

pub struct NpmrcConfigFileNotFoundDiagnostic;

impl NpmrcConfigFileNotFoundDiagnostic {
  pub fn at<P: AsRef<Path>>(path: P) -> MietteDiagnostic {
    let dir = path.as_ref().parent().map_or(Path::new(""), |p| p);
    MietteDiagnostic::new(format!("Config file was not found."))
      .with_code("shined(npmrc:config-file-not-found)")
      .with_severity(miette::Severity::Error)
      .with_help(format!(
        "Please add a .npmrc file to your project at {}",
        dir.display().to_string()
      ))
  }
}

pub struct NpmrcWrongRegistryDiagnostic;

impl NpmrcWrongRegistryDiagnostic {
  pub fn at(span: impl Into<SourceSpan>, validate_registry: &[String]) -> MietteDiagnostic {
    let label = format!(
      r#"Wrong registry , Only support registry: {}"#,
      validate_registry.join(", ")
    );

    MietteDiagnostic::new("Wrong registry")
      .with_code("shined(npmrc:wrong-registry)")
      .with_severity(miette::Severity::Error)
      .with_label(LabeledSpan::at(span, label))
      .with_help("Please add a registry field to your .npmrc file")
  }
}

pub struct NpmrcMissingRegistryDiagnostic;

impl NpmrcMissingRegistryDiagnostic {
  pub fn at(span: impl Into<SourceSpan>) -> MietteDiagnostic {
    MietteDiagnostic::new("No registry field found")
      .with_code("shined(npmrc:missing-registry)")
      .with_severity(miette::Severity::Error)
      .with_label(LabeledSpan::at(span, "No registry field found"))
      .with_help("Please add a registry field to your .npmrc file")
  }
}
