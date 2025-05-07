use miette::{LabeledSpan, MietteDiagnostic, SourceSpan};

pub struct NpmrcWrongRegistryDiagnostic;

impl NpmrcWrongRegistryDiagnostic {
  pub fn at(span: impl Into<SourceSpan>, validate_registry: &[String]) -> MietteDiagnostic {
    let label = format!(
      r#"Wrong registry , Only support registry: {}"#,
      validate_registry.join(", ")
    );

    MietteDiagnostic::new("Wrong registry")
      .with_code("shined(npmrc-wrong-registry)")
      .with_severity(miette::Severity::Error)
      .with_label(LabeledSpan::at(span, label))
      .with_help("Please add a registry field to your .npmrc file")
  }
}

pub struct NpmrcMissingRegistryDiagnostic;

impl NpmrcMissingRegistryDiagnostic {
  pub fn at(span: impl Into<SourceSpan>) -> MietteDiagnostic {
    MietteDiagnostic::new("No registry field found")
      .with_code("shined(npmrc-missing-registry)")
      .with_severity(miette::Severity::Error)
      .with_label(LabeledSpan::at(span, "No registry field found"))
      .with_help("Please add a registry field to your .npmrc file")
  }
}
