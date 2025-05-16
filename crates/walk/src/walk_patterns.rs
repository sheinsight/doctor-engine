use serde::Serialize;

pub const DEFAULT_PATTERNS: &str = "**/*.{js,jsx,ts,tsx,cjs,mjs,cts,mts}";

pub const DEFAULT_IGNORE_PATTERNS: &[&str] = &[
  "**/node_modules/**",
  "**/dist/**",
  "**/build/**",
  "**/coverage/**",
  "**/*.d.ts",
  "node_modules",
];

#[derive(Debug, Clone, Serialize)]
pub struct WalkPatterns {
  pub walk: String,
  pub ignore: Vec<String>,
}

impl Default for WalkPatterns {
  fn default() -> Self {
    Self {
      walk: DEFAULT_PATTERNS.to_string(),
      ignore: DEFAULT_IGNORE_PATTERNS
        .iter()
        .map(|s| s.to_string())
        .collect(),
    }
  }
}

impl WalkPatterns {
  pub fn with_walk(mut self, walk: &str) -> Self {
    self.walk = walk.to_string();
    self
  }

  pub fn with_ignore(mut self, ignore: &[String]) -> Self {
    self.ignore = ignore.iter().map(|s| s.to_string()).collect();
    self
  }

  pub fn build(self) -> Self {
    self
  }
}
