use napi_derive::napi;

use super::position::SourcePosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[napi(object)]
pub struct SourceLocation {
  pub start: SourcePosition,
  pub end: SourcePosition,
}

impl From<doctor_core::loc::Location> for SourceLocation {
  fn from(location: doctor_core::loc::Location) -> Self {
    SourceLocation {
      start: location.start.into(),
      end: location.end.into(),
    }
  }
}
