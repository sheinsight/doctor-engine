use std::path::PathBuf;

use package_json::package_json::PackageJson;

#[test]
fn test_package_json() {
    let package_json =
        PackageJson::try_from(PathBuf::from("./tests/fixtures/package_1.json")).unwrap();
    assert_eq!(package_json.name, Some("test".to_string()));
}
