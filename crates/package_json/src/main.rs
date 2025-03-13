use miette::Result;
use package_json::{error::NoNameError, package_json::PackageJson};

#[tokio::main]
async fn main() -> Result<()> {
  let src = r#"{
    "private": true,
    "packageManager": "npm@10.0.0",
    "dependencies": {
      "react": "npm:react@18.0.0"
    }
}"#;

  let package_json: PackageJson = serde_json::from_str(src).unwrap();

  println!("{:?}", package_json.get_dependencies().unwrap());

  //   let package_json = NoNameError::try_from(src)?;

  //   Ok(())

  //   let path = JsonPath::parse("$.private").unwrap();

  //   let package_json = serde_json::from_str(src).unwrap();

  //   let location = path.query_located(&package_json);

  //   println!("{:?}", location.first().unwrap());

  //   let offset = location.map(|loc| loc.offset()).unwrap_or(0);

  let error = NoNameError::new("test.json", src);

  //   let error = PrivateValueNotMatchError::new("test.json", src, (0, 0).into());

  Err(error.into())
}
