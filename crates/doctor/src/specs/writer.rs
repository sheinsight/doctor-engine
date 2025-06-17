use std::ops::{Deref, DerefMut};

pub trait Writer {
  fn write(&mut self, message: String);
}

pub struct ConsoleWriter;

impl Default for ConsoleWriter {
  fn default() -> Self {
    Self {}
  }
}

impl Writer for ConsoleWriter {
  fn write(&mut self, message: String) {
    println!("{}", message);
  }
}

pub struct StringWriter(pub Vec<String>);

impl Default for StringWriter {
  fn default() -> Self {
    Self(Vec::new())
  }
}

impl Deref for StringWriter {
  type Target = Vec<String>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for StringWriter {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl Writer for StringWriter {
  fn write(&mut self, message: String) {
    self.push(message);
  }
}
