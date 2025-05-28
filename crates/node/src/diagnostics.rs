use std::path::Path;

use miette::{LabeledSpan, MietteDiagnostic, SourceSpan, diagnostic};

pub struct DiagnosticFactory;

impl DiagnosticFactory {
  pub fn at_config_file_not_found<P: AsRef<Path>>(path: P, regex_str: &str) -> MietteDiagnostic {
    let dir = path.as_ref().parent().map_or(Path::new(""), |p| p);

    diagnostic!(
      severity = miette::Severity::Error,
      code = "shined(node-version:config-file-not-found)",
      help = format!(
        r#"Please add .node-version file to your project {}. 

Correctly declare the version number of the node you are using, which needs to meet the format '{}'.

e.g. 'v18.0.0' or '18.0.0'."#,
        dir.display().to_string(),
        regex_str,
      ),
      "Config file was not found.",
    )
  }

  pub fn at_invalid_version_range(
    span: impl Into<SourceSpan>,
    ranges: Vec<String>,
  ) -> MietteDiagnostic {
    let code = "shined(node-version:invalid-version-range)";
    let help = format!(r#"Select a valid version"#);
    let labels = vec![LabeledSpan::at(span, r#"Invalid version range"#)];

    diagnostic!(
      severity = miette::Severity::Error,
      code = code,
      help = help,
      labels = labels,
      "Only support version range in {:?}",
      ranges
    )
  }

  pub fn at_invalid_version_format(
    span: impl Into<SourceSpan>,
    regex_str: &str,
  ) -> MietteDiagnostic {
    let code = "shined(node-version:invalid-version-format)";
    let help = format!(
      r#"Please modify your version number to meet the format '{}'."#,
      regex_str
    );

    let labels = vec![LabeledSpan::at(span, "Invalid node version number format")];

    diagnostic!(
      severity = miette::Severity::Error,
      code = code,
      help = help,
      labels = labels,
      "Invalid node version number format",
    )
  }

  pub fn at_empty_node_version(span: impl Into<SourceSpan>) -> MietteDiagnostic {
    let code = "shined(node-version:empty-version)";
    let help = r#"Please add a node version to your .node-version file."#;
    let labels = vec![LabeledSpan::at(span, "Empty node version")];

    diagnostic!(
      severity = miette::Severity::Error,
      code = code,
      help = help,
      labels = labels,
      "Empty node version",
    )
  }
}
