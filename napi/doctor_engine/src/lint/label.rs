use napi_derive::napi;
use oxc_diagnostics::LabeledSpan;

use super::{Location, Span};

#[napi(object)]
#[derive(Debug, Clone)]
pub struct LabeledLoc {
  pub span: Span,
  pub loc: Location,
}

impl LabeledLoc {
  pub fn with_labeled_span(source_text: &str, span: LabeledSpan) -> Self {
    let span = Span {
      start: span.offset() as u32,
      end: span.offset() as u32 + span.len() as u32,
    };
    let loc = Location::with_source(source_text, span);
    Self { span, loc }
  }
}
