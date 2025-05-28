use doctor_core::Ignore;
use napi_derive::napi;

#[derive(Debug, Clone)]
#[napi(object)]
pub struct JsOpts {
  pub ignore: Option<Vec<String>>,
}

impl Into<doctor::cloc::Opts> for JsOpts {
  fn into(self) -> doctor::cloc::Opts {
    doctor::cloc::Opts {
      ignore: self.ignore.map(Ignore::from).unwrap_or_default(),
    }
  }
}
