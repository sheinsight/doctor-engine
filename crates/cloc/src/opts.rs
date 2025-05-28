use doctor_core::Ignore;

#[derive(Debug, Clone)]
pub struct Opts {
  pub ignore: Ignore,
}

impl Default for Opts {
  fn default() -> Self {
    Self {
      ignore: Ignore::default(),
    }
  }
}
