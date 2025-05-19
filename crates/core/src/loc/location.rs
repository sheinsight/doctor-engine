use super::Position;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Location {
  pub start: Position,
  pub end: Position,
}

impl Location {
  pub fn new(source_code: String, offset: usize, len: usize) -> Self {
    let start = offset;
    let end = offset + len;

    let start_pos = Position::from_source(&source_code, start);
    let end_pos = Position::from_source(&source_code, end);

    Self {
      start: start_pos,
      end: end_pos,
    }
  }
}
