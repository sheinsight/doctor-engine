use std::path::Path;

use oxc::diagnostics::NamedSource;

#[derive(Debug, Clone)]
pub struct PathWithSource {
  pub file_path: String,
  pub source_code: String,
}

impl PathWithSource {
  pub fn try_from<T: AsRef<Path>>(path: T) -> std::io::Result<Self> {
    let source_code = std::fs::read_to_string(path.as_ref())?;
    let file_path = path.as_ref().to_string_lossy().to_string();

    Ok(Self {
      file_path,
      source_code,
    })
  }
}

impl Into<NamedSource<String>> for PathWithSource {
  fn into(self) -> NamedSource<String> {
    NamedSource::new(self.file_path, self.source_code)
  }
}
