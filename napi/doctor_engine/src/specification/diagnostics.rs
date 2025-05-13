use miette::MietteDiagnostic;
use napi_derive::napi;

use super::{labeled_span::NapiLabeledSpan, severity::NapiSeverity};

#[napi(object)]
pub struct NapiDiagnostics {
  pub message: String,
  pub code: Option<String>,
  pub severity: Option<NapiSeverity>,
  pub help: Option<String>,
  pub url: Option<String>,
  pub labels: Option<Vec<NapiLabeledSpan>>,
}

impl From<MietteDiagnostic> for NapiDiagnostics {
  fn from(diagnostic: MietteDiagnostic) -> Self {
    NapiDiagnostics {
      message: diagnostic.message,
      code: diagnostic.code,
      severity: diagnostic.severity.map(|s| s.into()),
      help: diagnostic.help,
      url: diagnostic.url,
      labels: diagnostic
        .labels
        .map(|labels| labels.into_iter().map(|l| l.into()).collect()),
    }
  }
}
