use doctor::core::loc::Position;
use napi_derive::napi;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[napi(object)]
pub struct JsSourcePosition {
  pub row: u32,
  pub col: u32,
}

impl From<Position> for JsSourcePosition {
  fn from(position: Position) -> Self {
    JsSourcePosition {
      row: position.row as u32,
      col: position.col as u32,
    }
  }
}
