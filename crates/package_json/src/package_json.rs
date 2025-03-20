use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{error::Error, version::Version};

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
  pub fn validate_name(self) -> Result<Self, Error> {
    if self.name.is_none() {
      return Err(Error::NoNameError);
    }
    Ok(self)
  }

  pub fn validate_private(self, expect_value: bool) -> Result<Self, Error> {
    if self.private.is_none() {
      return Err(Error::NoPrivateError);
    }
    if let Some(private) = self.private {
      if private != expect_value {
        return Err(Error::NoMatchedPrivateError {
          expect_value,
          actual_value: private,
        });
      }
      Ok(self)
    } else {
      return Err(Error::NoPrivateError);
    }
  }

  pub fn validate_package_manager(self) -> Result<Self, Error> {
    if self.package_manager.is_none() {
      return Err(Error::NoPackageManagerError);
    }
    Ok(self)
  }

  fn get_deps(deps: &Option<HashMap<String, String>>) -> Result<HashMap<String, Version>, Error> {
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

  pub fn get_dependencies(&self) -> Result<HashMap<String, Version>, Error> {
    Self::get_deps(&self.dependencies)
  }

  pub fn get_dev_dependencies(&self) -> Result<HashMap<String, Version>, Error> {
    Self::get_deps(&self.dev_dependencies)
  }

  pub fn new(cwd: &str) -> Result<Self, Error> {
    let path = PathBuf::from(cwd).join("package.json");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let package_json: PackageJson = serde_json::from_reader(reader)?;
    Ok(package_json)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_validate_name() {
    let src = r#"{
      "private": true,
      "packageManager": "npm@10.0.0",
      "dependencies": {
        "react": "npm:react@18.0.0"
      }
    }"#;

    let json: PackageJson = serde_json::from_str(src).unwrap();

    let result = json.validate_name();

    assert!(result.is_err());
  }

  #[test]
  fn test_validate_private() {
    let src = r#"{
      "name": "test"
    }"#;

    let json: PackageJson = serde_json::from_str(src).unwrap();

    let result = json.validate_private(true);

    assert!(result.is_err());
  }

  #[test]
  fn test_validate_private_with_expect_value() {
    let src = r#"{
      "name": "test",
      "private": true
    }"#;

    let json: PackageJson = serde_json::from_str(src).unwrap();

    let result = json.validate_private(true);

    assert!(result.is_ok());
  }

  #[test]
  fn test_validate_package_manager() {
    let src = r#"{
      "name": "test",
      "private": true
    }"#;

    let json: PackageJson = serde_json::from_str(src).unwrap();

    let result = json.validate_package_manager();

    assert!(result.is_err());
  }
}
