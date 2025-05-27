use napi_derive::napi;

#[napi(object)]
pub struct JsSourceSpan {
  pub offset: u32,
  pub length: u32,
}

impl From<miette::SourceSpan> for JsSourceSpan {
  fn from(span: miette::SourceSpan) -> Self {
    JsSourceSpan {
      offset: span.offset() as u32,
      length: span.len() as u32,
    }
  }
}
