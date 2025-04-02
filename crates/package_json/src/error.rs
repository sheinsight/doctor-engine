use doctor_ext::define_errors;

define_errors! {
  PackageJsonValidatorError{
    #[message = "missing name field in {config_path}"]
    MissingNameErr{
      config_path: String,
    },

    #[message = "missing private field in {config_path}"]
    MissingPrivateErr{
      config_path: String,
    },

    #[message = "no matched private field , expect {expect} but actual {actual} in {config_path}"]
    NoMatchedPrivateErr{
      config_path: String,
      expect: bool,
      actual: bool,
    },

    #[message = "missing package manager field in {config_path}"]
    MissingPackageManagerErr{
      config_path: String,
    },

    #[message = "io error: {path} {source}"]
    IoErr{
      path: String,
      source: std::io::Error,
    },

    #[message = "parse error: {path} {source}"]
    ParseErr{
      path: String,
      source: serde_json::Error,
    },
  }
}

define_errors! {
  VersionError{
    #[message = "npm alias parser error: {version}"]
    AliasParserErr{
      version: String,
    },
  }
}
// #[derive(Error, Debug, Diagnostic)]
// pub enum PackageJsonValidatorError {
//   #[error("npm alias parser error: {version}")]
//   NpmAliasParserError { version: String },

//   #[error("{0} no name field")]
//   NoNameError(String),

//   #[error("{0} not found private field")]
//   NoPrivateError(String),

//   #[error("{path} no matched private field , expect {expect_value} but actual {actual_value}")]
//   NoMatchedPrivateError {
//     path: String,
//     expect_value: bool,
//     actual_value: bool,
//   },

//   #[error("{0} no package manager field")]
//   NoPackageManagerError(String),

//   #[error("{path} {error}")]
//   IoError {
//     path: String,
//     #[source]
//     error: std::io::Error,
//   },

//   #[error("{path} {error}")]
//   ParseError {
//     path: String,
//     #[source]
//     error: serde_json::Error,
//   },
// }
