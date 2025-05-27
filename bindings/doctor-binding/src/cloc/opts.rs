use doctor_walk::WalkIgnore;
use napi_derive::napi;

#[derive(Debug, Clone)]
#[napi(object)]
pub struct JsOpts {
  pub ignore: Option<Vec<String>>,
}

impl Into<doctor_cloc::Opts> for JsOpts {
  fn into(self) -> doctor_cloc::Opts {
    doctor_cloc::Opts {
      ignore: self.ignore.map(WalkIgnore::from).unwrap_or_default(),
    }
  }
}
