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

    #[message = "parse error: {path} {source}"]
    ParseErr{
      path: String,
      source: std::io::Error,
    },
  }
}

define_errors! {
  VersionError {
    #[message = "npm alias parser error: {version}"]
    AliasParserErr{
      version: String,
    },
  }
}
