#![recursion_limit = "512"]
mod category;
mod common;
pub use category::Category;
pub use common::{
  FileDiagnostic,
  environments::{Environment, EnvironmentFlags},
  lint_mode::LintMode,
};

pub mod config;
pub mod ext;
pub mod inner;
mod validator;

pub use validator::*;

pub use config::*;
