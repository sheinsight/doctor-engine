use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{error::VersionError, version::Version};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackageJson {
  pub private: Option<bool>,
  pub name: Option<String>,
  pub version: Option<String>,
  pub author: Option<String>,
  pub license: Option<String>,
  pub repository: Option<String>,
  pub homepage: Option<String>,
  pub keywords: Option<Vec<String>>,
  pub description: Option<String>,
  pub main: Option<String>,
  pub exports: Option<HashMap<String, String>>,
  pub files: Option<Vec<String>>,
  pub scripts: Option<HashMap<String, String>>,
  #[serde(rename = "packageManager")]
  pub package_manager: Option<String>,
  pub dependencies: Option<HashMap<String, String>>,
  #[serde(rename = "devDependencies")]
  pub dev_dependencies: Option<HashMap<String, String>>,
  #[serde(rename = "peerDependencies")]
  pub peer_dependencies: Option<HashMap<String, String>>,
  #[serde(rename = "optionalDependencies")]
  pub optional_dependencies: Option<HashMap<String, String>>,
}

impl PackageJson {
  fn get_deps(
    deps: &Option<HashMap<String, String>>,
  ) -> Result<HashMap<String, Version>, VersionError> {
    match deps {
      Some(deps) => {
        let mut dependencies = HashMap::with_capacity(deps.len());
        for (k, v) in deps.iter() {
          let version = Version::try_from(v.clone())?;
          dependencies.insert(k.clone(), version);
        }
        Ok(dependencies)
      }
      None => Ok(HashMap::new()),
    }
  }

  pub fn get_dependencies(&self) -> Result<HashMap<String, Version>, VersionError> {
    Self::get_deps(&self.dependencies)
  }

  pub fn get_dev_dependencies(&self) -> Result<HashMap<String, Version>, VersionError> {
    Self::get_deps(&self.dev_dependencies)
  }
}
