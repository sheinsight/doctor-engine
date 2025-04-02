use doctor_ext::define_errors;

define_errors! {
  NodeVersionValidatorError{
    #[message = "not found {config_path}"]
    NotFoundErr{
      config_path: String,
    },
    #[message = "invalid version {version} in {config_path}"]
    InvalidErr{
      config_path: String,
      version:String,
    },
    #[message = "unknown error {source}"]
    UnknowErr{
      source: Box<dyn std::error::Error>,
    }
  }
}
