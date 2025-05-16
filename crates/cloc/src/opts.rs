use doctor_walk::WalkIgnore;

#[derive(Debug, Clone)]
pub struct Opts {
  pub ignore: WalkIgnore,
}

impl Default for Opts {
  fn default() -> Self {
    Self {
      ignore: WalkIgnore::default(),
    }
  }
}
