use doctor_core::loc::Location;
use miette::LabeledSpan;
use napi_derive::napi;

use super::{location::SourceLocation, source_span::SourceSpan};

#[napi(object)]
pub struct NapiLabeledSpan {
  pub label: Option<String>,
  pub span: SourceSpan,
  pub loc: SourceLocation,
  pub primary: bool,
}

impl From<(LabeledSpan, Location)> for NapiLabeledSpan {
  fn from((span, loc): (LabeledSpan, Location)) -> Self {
    NapiLabeledSpan {
      label: span.label().map(|s| s.to_string()),
      span: SourceSpan::from(span.inner().to_owned()),
      loc: loc.into(),
      primary: span.primary(),
    }
  }
}
