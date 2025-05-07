use std::path::Path;

use doctor_ext::ValidatorError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NpmrcConfig {
  pub registry: Option<String>,

  #[serde(skip)]
  pub __raw_source: String,

  #[serde(skip)]
  pub __config_path: String,
}

impl NpmrcConfig {
  pub fn parse<P: AsRef<Path>>(path: P) -> Result<Self, ValidatorError> {
    let raw_source = std::fs::read_to_string(path.as_ref())?;

    let mut config: NpmrcConfig = serde_ini::from_str(&raw_source)?;
    config.__raw_source = raw_source;
    config.__config_path = path.as_ref().display().to_string();

    Ok(config)
  }
}
