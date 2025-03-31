use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[napi(object, js_name = "Span")]
#[derive(
  Debug, Default, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Copy, PartialOrd, Ord,
)]
pub struct Span {
  pub offset: u32,
  pub len: u32,
}

impl Span {
  pub fn new(offset: usize, len: usize) -> Self {
    Self {
      offset: offset as u32,
      len: len as u32,
    }
  }
}

impl From<oxc_diagnostics::LabeledSpan> for Span {
  fn from(value: oxc_diagnostics::LabeledSpan) -> Self {
    Self {
      offset: value.offset() as u32,
      len: value.offset() as u32 + value.len() as u32,
    }
  }
}
