use std::{collections::HashMap, path::PathBuf};

use package_json::package_json::PackageJson;

#[test]
fn test_package_json() {
  let package_json =
    PackageJson::try_from(PathBuf::from("./tests/fixtures/package_1.json")).unwrap();
  assert_eq!(package_json.name, Some("test".to_string()));
  assert_eq!(package_json.version, Some("1.0.0".to_string()));
  assert_eq!(package_json.private, Some(true));

  let mut scripts = HashMap::new();
  let k = "test".to_string();
  let v = "echo \"Error: no test specified\" && exit 1".to_string();
  scripts.insert(k, v);

  assert_eq!(package_json.scripts, Some(scripts));
}
