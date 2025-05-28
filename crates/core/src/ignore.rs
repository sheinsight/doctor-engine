use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

pub const IGNORE_PATTERNS: [&str; 17] = [
  "**/node_modules/**",
  "node_modules",
  "**/*.d.ts",
  "**/*.min.*",
  "**/*-min.*",
  "**/*_min.*",
  "**/.yarn/**",
  "**/.temp/**",
  "**/.tmp/**",
  "**/.next/**",
  "**/.nuxt/**",
  "**/.output/**",
  "**/.idea/**",
  "**/.history/**",
  "**/.cache/**",
  "**/coverage/**",
  "**/build/**",
];

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ignore(pub Vec<String>);

impl Default for Ignore {
  fn default() -> Self {
    Self(IGNORE_PATTERNS.iter().map(|s| s.to_string()).collect())
  }
}

impl Deref for Ignore {
  type Target = Vec<String>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Ignore {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl From<Vec<String>> for Ignore {
  fn from(value: Vec<String>) -> Self {
    Self(value)
  }
}

impl<const N: usize> From<&[&str; N]> for Ignore {
  fn from(value: &[&str; N]) -> Self {
    Self(value.iter().map(|s| s.to_string()).collect())
  }
}

impl From<Vec<&str>> for Ignore {
  fn from(value: Vec<&str>) -> Self {
    Self(value.into_iter().map(|s| s.to_string()).collect())
  }
}
