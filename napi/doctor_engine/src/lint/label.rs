use napi_derive::napi;

use super::{Location, Span};

#[napi(object)]
#[derive(Debug, Clone)]
pub struct LabeledLoc {
  pub span: Span,
  pub loc: Location,
}

impl LabeledLoc {
  pub fn new(source_code: &str, offset: usize, len: usize) -> Self {
    let start = offset;
    let end = offset + len;
    let span = super::Span::new(offset, len);
    let loc = super::Location::with_source(&source_code, start, end);
    Self { span, loc }
  }
}
