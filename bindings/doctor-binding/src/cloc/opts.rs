use doctor_core::Ignore;
use napi_derive::napi;

#[derive(Debug, Clone)]
#[napi(object)]
pub struct JsClocOpts {
  pub ignore: Option<Vec<String>>,
}

impl Into<doctor::cloc::Opts> for JsClocOpts {
  fn into(self) -> doctor::cloc::Opts {
    doctor::cloc::Opts {
      ignore: self.ignore.map(Ignore::from).unwrap_or_default(),
    }
  }
}
