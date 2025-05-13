use miette::LabeledSpan;
use napi_derive::napi;

use super::source_span::NapiSourceSpan;

#[napi(object)]
pub struct NapiLabeledSpan {
  pub label: Option<String>,
  pub span: NapiSourceSpan,
  pub primary: bool,
}

impl From<LabeledSpan> for NapiLabeledSpan {
  fn from(span: LabeledSpan) -> Self {
    NapiLabeledSpan {
      label: span.label().map(|s| s.to_string()),
      span: NapiSourceSpan::from(span.inner().to_owned()),
      primary: span.primary(),
    }
  }
}
