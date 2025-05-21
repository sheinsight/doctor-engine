use std::path::Path;

use miette::{LabeledSpan, MietteDiagnostic, SourceSpan, diagnostic};

pub struct DiagnosticFactory;

impl DiagnosticFactory {
  pub fn at_config_file_not_found<P: AsRef<Path>>(path: P) -> MietteDiagnostic {
    let file = path.as_ref();
    let dir = file.parent().map_or(Path::new(""), |p| p);

    let code = "shined(npmrc:config-file-not-found)";
    let help = format!(
      "Please add a .npmrc file to your project at {}",
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

  pub fn at_invalid_registry(
    span: impl Into<SourceSpan>,
    validate_registry: &[String],
  ) -> MietteDiagnostic {
    let code = "shined(npmrc:invalid-registry)";
    let help = format!("Only support registry: {:?}", validate_registry);

    let labels = vec![LabeledSpan::underline(span)];

    diagnostic!(
      severity = miette::Severity::Error,
      code = code,
      help = help,
      labels = labels,
      r#"Invalid registry "#,
    )
  }

  pub fn at_missing_registry(span: impl Into<SourceSpan>) -> MietteDiagnostic {
    let code = "shined(npmrc:missing-registry)";
    let help = "Please add a registry field to your .npmrc file";
    let labels = vec![LabeledSpan::underline(span)];

    diagnostic!(
      severity = miette::Severity::Error,
      code = code,
      help = help,
      labels = labels,
      "Missing registry field",
    )
  }
}
