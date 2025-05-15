use napi_derive::napi;
use serde::{Deserialize, Serialize};

use crate::lint::Position;

#[napi(object, js_name = "Location")]
#[derive(
  Debug, Default, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Copy, PartialOrd, Ord,
)]
pub struct Location {
  pub start: Position,
  pub end: Position,
}

impl Location {
  pub fn new(start: Position, end: Position) -> Self {
    Location { start, end }
  }

  pub fn with_source(source_code: &str, start: usize, end: usize) -> Self {
    let start = Position::with_source(source_code, start);
    let end = Position::with_source(source_code, end);
    Self { start, end }
  }
}
