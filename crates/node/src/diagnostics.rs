use std::path::Path;

use miette::{LabeledSpan, MietteDiagnostic, SourceSpan};

pub struct NodeVersionFileNotFoundDiagnostic;

impl NodeVersionFileNotFoundDiagnostic {
  pub fn at<P: AsRef<Path>>(path: P, regex_str: &str) -> MietteDiagnostic {
    let dir = path.as_ref().parent().map_or(Path::new(""), |p| p);

    MietteDiagnostic::new(r#"Config file was not found."#)
      .with_code("shined(node-version:config-file-not-found)")
      .with_severity(miette::Severity::Error)
      .with_help(format!(
        r#"Please add .node-version file to your project {}. 

Correctly declare the version number of the node you are using, which needs to meet the format '{}'.

e.g. 'v18.0.0' or '18.0.0'."#,
        regex_str,
        dir.display().to_string()
      ))
  }
}

pub struct InvalidVersionRangeDiagnostic;

impl InvalidVersionRangeDiagnostic {
  pub fn at(span: impl Into<SourceSpan>, ranges: Vec<String>) -> MietteDiagnostic {
    let label = format!(
      r#"Wrong version number format , Only support version range in {}"#,
      ranges.join(", ")
    );

    MietteDiagnostic::new(r#"The node version is not in the valid range."#)
      .with_code("shined(node-version:not-in-valid-range)")
      .with_label(LabeledSpan::at(span, label))
      .with_severity(miette::Severity::Error)
  }
}

pub struct InvalidVersionFormatDiagnostic;

impl InvalidVersionFormatDiagnostic {
  pub fn at(span: impl Into<SourceSpan>, regex_str: &str) -> MietteDiagnostic {
    MietteDiagnostic::new(format!(
      r#"Only support version numbers that meet '{}'."#,
      regex_str
    ))
    .with_label(LabeledSpan::at(
      span,
      r#"Invalid node version number format"#,
    ))
    .with_help(format!(
      r#"Please modify your version number to meet the format '{}'."#,
      regex_str
    ))
    .with_code("shined(node-version:invalid-version-format)")
    .with_severity(miette::Severity::Error)
  }
}

pub struct EmptyNodeVersionDiagnostic;

impl EmptyNodeVersionDiagnostic {
  pub fn at(span: impl Into<SourceSpan>) -> MietteDiagnostic {
    MietteDiagnostic::new(r#"Empty node version"#)
      .with_code("shined(node-version:empty-version)")
      .with_severity(miette::Severity::Error)
      .with_label(LabeledSpan::at(span, r#"Empty node version"#))
      .with_help(r#"Please add a node version to your .node-version file."#)
  }
}
