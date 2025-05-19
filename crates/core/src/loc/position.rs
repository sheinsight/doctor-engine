#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
  pub row: usize,
  pub col: usize,
}

impl Position {
  pub fn from_source(source_text: &str, offset: usize) -> Self {
    let rope = ropey::Rope::from_str(&source_text);
    let line = rope.try_byte_to_line(offset).unwrap_or(0);

    let first_char_of_line = rope.try_line_to_char(line).unwrap_or(0);
    let offset = rope.try_byte_to_char(offset).unwrap_or(0);
    let col = offset - first_char_of_line;

    Self {
      row: line + 1,
      col: col + 1,
    }
  }
}
