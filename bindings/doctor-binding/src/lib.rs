pub mod cloc;
mod lint;
mod log;
pub mod standards;

pub use lint::*;
pub use log::*;
use napi_derive::napi;

#[napi]
pub struct Demo {
  pub cwd: String,
}

#[napi]
impl Demo {
  #[napi(factory)]
  pub fn create(cwd: String) -> Demo {
    Demo { cwd }
  }

  #[napi]
  pub fn get_cwd(&self) -> String {
    self.cwd.clone()
  }
}
