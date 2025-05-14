use miette::SourceSpan;
use napi_derive::napi;

#[napi(object)]
pub struct NapiSourceSpan {
  pub offset: u32,
  pub length: u32,
}

impl From<SourceSpan> for NapiSourceSpan {
  fn from(span: SourceSpan) -> Self {
    NapiSourceSpan {
      offset: span.offset() as u32,
      length: span.len() as u32,
    }
  }
}
