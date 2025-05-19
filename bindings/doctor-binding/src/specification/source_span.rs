use napi_derive::napi;

#[napi(object)]
pub struct SourceSpan {
  pub offset: u32,
  pub length: u32,
}

impl From<miette::SourceSpan> for SourceSpan {
  fn from(span: miette::SourceSpan) -> Self {
    SourceSpan {
      offset: span.offset() as u32,
      length: span.len() as u32,
    }
  }
}
