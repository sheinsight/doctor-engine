use std::path::Path;

use doctor_core::ValidatorError;

pub struct NodeVersion {
  pub version: Option<String>,
  pub __raw_source: Option<String>,
  pub __config_path: String,
}

impl NodeVersion {
  pub fn parse<P: AsRef<Path>>(path: P) -> Result<Self, ValidatorError> {
    let path = path.as_ref();

    let raw_source = std::fs::read_to_string(path)?;
    let version = raw_source.trim().to_string();

    let version = if version.is_empty() {
      None
    } else {
      Some(version)
    };

    Ok(Self {
      version: version,
      __raw_source: Some(raw_source),
      __config_path: path.display().to_string(),
    })
  }
}
