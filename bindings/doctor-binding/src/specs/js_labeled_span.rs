use doctor_core::loc::Location;
use miette::LabeledSpan;
use napi_derive::napi;

use super::{js_location::JsSourceLocation, js_source_span::JsSourceSpan};

#[napi(object)]
pub struct JsLabeledSpan {
  pub label: Option<String>,
  pub span: JsSourceSpan,
  pub loc: JsSourceLocation,
  pub primary: bool,
}

impl From<(LabeledSpan, Location)> for JsLabeledSpan {
  fn from((span, loc): (LabeledSpan, Location)) -> Self {
    JsLabeledSpan {
      label: span.label().map(|s| s.to_string()),
      span: JsSourceSpan::from(span.inner().to_owned()),
      loc: loc.into(),
      primary: span.primary(),
    }
  }
}
