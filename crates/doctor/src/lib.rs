mod dashboard;
pub use dashboard::*;
pub mod specs;

pub mod cloc {
  pub use doctor_cloc::*;
}

pub mod core {
  pub use doctor_core::*;
}

pub mod lint {
  pub use doctor_lint::*;
}
