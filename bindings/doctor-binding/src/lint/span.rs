use napi_derive::napi;
use oxc::diagnostics::LabeledSpan;
use serde::{Deserialize, Serialize};

#[napi(object, js_name = "Span")]
#[derive(
  Debug, Default, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Copy, PartialOrd, Ord,
)]
pub struct Span {
  pub offset: u32,
  pub length: u32,
}

impl Span {
  pub fn new(offset: usize, len: usize) -> Self {
    Self {
      offset: offset as u32,
      length: len as u32,
    }
  }
}

impl From<LabeledSpan> for Span {
  fn from(value: LabeledSpan) -> Self {
    Self {
      offset: value.offset() as u32,
      length: value.offset() as u32 + value.len() as u32,
    }
  }
}
