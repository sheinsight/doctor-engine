use doctor_core::loc::get_source_location;
use miette::MietteDiagnostic;
use napi_derive::napi;

use super::{js_labeled_span::JsLabeledSpan, js_severity::JsSeverity};

#[napi(object)]
pub struct JsDiagnostics {
  pub message: String,
  pub code: Option<String>,
  pub severity: Option<JsSeverity>,
  pub help: Option<String>,
  pub url: Option<String>,
  pub labels: Option<Vec<JsLabeledSpan>>,
}

impl From<(String, MietteDiagnostic)> for JsDiagnostics {
  fn from((source_code, diagnostic): (String, MietteDiagnostic)) -> Self {
    JsDiagnostics {
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
          .map(JsLabeledSpan::from)
          .collect()
      }),
    }
  }
}
