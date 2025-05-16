use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

pub const IGNORE_PATTERNS: [&str; 3] = ["**/node_modules/**", "node_modules", "**/*.d.ts"];

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalkIgnore(pub Vec<String>);

impl Default for WalkIgnore {
  fn default() -> Self {
    Self(IGNORE_PATTERNS.iter().map(|s| s.to_string()).collect())
  }
}

impl Deref for WalkIgnore {
  type Target = Vec<String>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for WalkIgnore {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
