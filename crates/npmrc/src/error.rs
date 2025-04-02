use doctor_ext::define_errors;

define_errors! {
  NpmrcValidatorError {
    #[message = "expected: {expect}, actual: {actual} in {config_path}"]
    MatchedFailErr {
      config_path: String,
      expect: String,
      actual: String,
    },

    #[message = "build config {config_path} failed, {source}"]
    BuildConfigErr {
      config_path: String,
      source: config::ConfigError,
    },

    #[message = "field {field} not found in {config_path} , {source}"]
    NotFoundFieldErr {
      field: String,
      config_path: String,
      source: config::ConfigError,
    },

    #[message = "unknown error {source}"]
    UnknownErr {
      source: Box<dyn std::error::Error>,
    },
  }
}
