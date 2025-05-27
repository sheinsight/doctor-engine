use napi_derive::napi;

#[derive(Debug, Clone)]
#[napi(object)]
pub struct JsOpts {
  pub ignore: Vec<String>,
}

impl Into<doctor_cloc::Opts> for JsOpts {
  fn into(self) -> doctor_cloc::Opts {
    doctor_cloc::Opts {
      ignore: self.ignore.into(),
    }
  }
}
