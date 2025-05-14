use std::{fs::read_to_string, path::Path};

use doctor_core::ValidatorError;
use doctor_walk_parallel::WalkIgnore;
use serde::{Deserialize, Serialize};

use super::Globals;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Sfconfig {
  #[serde(default)]
  pub globals: Globals,
  #[serde(default)]
  pub ignore: WalkIgnore,
}

impl Sfconfig {
  pub fn parse<P: AsRef<Path>>(path: P) -> Result<Self, ValidatorError> {
    if let Ok(config) = read_to_string(path) {
      let config = serde_json::from_str::<Sfconfig>(&config)?;
      return Ok(config);
    }

    Ok(Self::default())
  }
}
