use napi_derive::napi;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[napi(object)]
pub struct JsSourcePosition {
  pub row: u32,
  pub col: u32,
}

impl From<doctor_core::loc::Position> for JsSourcePosition {
  fn from(position: doctor_core::loc::Position) -> Self {
    JsSourcePosition {
      row: position.row as u32,
      col: position.col as u32,
    }
  }
}
