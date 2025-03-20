use std::path::Path;

pub trait PathExt {
  fn to_string_owned(&self) -> String;
}

impl PathExt for Path {
  fn to_string_owned(&self) -> String {
    self.to_string_lossy().to_string()
  }
}
