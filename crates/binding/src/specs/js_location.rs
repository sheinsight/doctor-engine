use doctor::core::loc::Location;
use napi_derive::napi;

use super::js_position::JsSourcePosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[napi(object)]
pub struct JsSourceLocation {
  pub start: JsSourcePosition,
  pub end: JsSourcePosition,
}

impl From<Location> for JsSourceLocation {
  fn from(location: Location) -> Self {
    JsSourceLocation {
      start: location.start.into(),
      end: location.end.into(),
    }
  }
}
