use doctor::lint::FileDiagnostic;
use napi_derive::napi;
use oxc_diagnostics::Severity;

use super::LabeledLoc;

#[napi(object)]
#[derive(Debug, Clone)]
pub struct Diagnostic {
  pub file_name: String,
  pub help: String,
  pub url: String,
  pub severity: String,
  pub code: String,
  pub message: String,
  pub labels: Vec<LabeledLoc>,
}

impl Diagnostic {
  pub fn from_file_diagnostic(file_diagnostic: &FileDiagnostic) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for diag in file_diagnostic.diagnostics.iter() {
      let file_name = file_diagnostic.path_with_source.file_path.clone();

      let code = file_diagnostic.path_with_source.source_code.clone();

      let help = diag.help.as_deref().unwrap_or_default().to_string();

      let url = diag.url.as_deref().unwrap_or_default().to_string();

      let severity = match diag.severity {
        Severity::Advice => "advice".to_string(),
        Severity::Warning => "warning".to_string(),
        Severity::Error => "error".to_string(),
      };

      let labels = diag
        .labels
        .as_ref()
        .map(|v| {
          v.iter()
            .map(|l| {
              return LabeledLoc::with_labeled_span(
                &file_diagnostic.path_with_source.source_code,
                l.clone(),
              );
            })
            .collect::<Vec<_>>()
        })
        .unwrap_or_default();

      let diagnostic = Diagnostic {
        file_name,
        help,
        url,
        severity,
        code,
        message: diag.message.to_string(),
        labels,
      };
      diagnostics.push(diagnostic);
    }
    diagnostics
  }
}
