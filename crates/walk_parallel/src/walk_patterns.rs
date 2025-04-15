use serde::Serialize;

pub const DEFAULT_PATTERNS: &str = "**/*.{js,jsx,ts,tsx,cjs,mjs,cts,mts}";

pub const DEFAULT_IGNORE_PATTERNS: &[&str] = &[
  "**/node_modules/**",
  "**/dist/**",
  "**/build/**",
  "**/coverage/**",
  "**/*.d.ts",
  "node_modules",
  "*.min.js",
  "*.min.css",
];

pub const DEFAULT_TEST_PATTERNS: &[&str] = &[
  "**/test/**",
  "**/tests/**",
  "**/spec/**",
  "**/specs/**",
  "*.spec.js",
  "*.spec.jsx",
  "*.spec.ts",
  "*.spec.tsx",
  "*.test.js",
  "*.test.jsx",
  "*.test.ts",
  "*.test.tsx",
];

pub const DEFAULT_DTS_PATTERNS: &[&str] = &["**/*.d.ts"];

#[derive(Debug, Clone, Serialize)]
pub struct WalkPatterns {
  pub walk: String,
  pub ignore: Vec<String>,
  pub testing: Vec<String>,
  pub dts: Vec<String>,
}

impl Default for WalkPatterns {
  fn default() -> Self {
    Self {
      walk: DEFAULT_PATTERNS.to_string(),
      ignore: DEFAULT_IGNORE_PATTERNS
        .iter()
        .map(|s| s.to_string())
        .collect(),
      testing: DEFAULT_TEST_PATTERNS
        .iter()
        .map(|s| s.to_string())
        .collect(),
      dts: DEFAULT_DTS_PATTERNS.iter().map(|s| s.to_string()).collect(),
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

  pub fn with_testing(mut self, testing: &[String]) -> Self {
    self.testing = testing.iter().map(|s| s.to_string()).collect();
    self
  }

  pub fn with_dts(mut self, dts: &[String]) -> Self {
    self.dts = dts.iter().map(|s| s.to_string()).collect();
    self
  }

  pub fn build(self) -> Self {
    self
  }
}
