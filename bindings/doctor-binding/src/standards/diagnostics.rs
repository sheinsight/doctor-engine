use doctor_core::loc::get_source_location;
use miette::MietteDiagnostic;
use napi_derive::napi;

use super::{labeled_span::NapiLabeledSpan, severity::Severity};

#[napi(object)]
pub struct NapiDiagnostics {
  pub message: String,
  pub code: Option<String>,
  pub severity: Option<Severity>,
  pub help: Option<String>,
  pub url: Option<String>,
  pub labels: Option<Vec<NapiLabeledSpan>>,
}

impl From<(String, MietteDiagnostic)> for NapiDiagnostics {
  fn from((source_code, diagnostic): (String, MietteDiagnostic)) -> Self {
    NapiDiagnostics {
      message: diagnostic.message,
      code: diagnostic.code,
      severity: diagnostic.severity.map(|s| s.into()),
      help: diagnostic.help,
      url: diagnostic.url,
      labels: diagnostic.labels.map(|labels| {
        labels
          .into_iter()
          .map(|labeled_span| {
            let loc = get_source_location(
              source_code.clone(),
              labeled_span.offset(),
              labeled_span.len(),
            );
            (labeled_span, loc)
          })
          .map(NapiLabeledSpan::from)
          .collect()
      }),
    }
  }
}
