use std::path::Path;

pub trait PathExt {
  fn to_string_owned(&self) -> String;
}

impl PathExt for Path {
  fn to_string_owned(&self) -> String {
    self.to_string_lossy().to_string()
  }
}

pub trait MultiFrom: Sized {
  type Error;
  fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Self::Error>;
  fn from_cwd<P: AsRef<Path>>(cwd: P) -> Result<Self, Self::Error>;
}

pub trait Validator {
  type Error;
  fn validate(&self) -> Result<(), Self::Error>;
}
