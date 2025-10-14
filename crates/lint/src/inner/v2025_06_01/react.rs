use crate::config::{ReactConfig, ReactRuntime};

impl ReactConfig {
  pub fn with_runtime(mut self, runtime: ReactRuntime) -> Self {
    self.runtime = runtime;
    self
  }
}

impl Default for ReactConfig {
  fn default() -> Self {
    Self {
      runtime: ReactRuntime::Automatic,
    }
  }
}
