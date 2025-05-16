use napi_derive::napi;

#[derive(Debug, Clone)]
#[napi(object)]
pub struct Opts {
  pub ignore: Vec<String>,
}

impl Into<doctor_cloc::Opts> for Opts {
  fn into(self) -> doctor_cloc::Opts {
    doctor_cloc::Opts {
      ignore: self.ignore.into(),
    }
  }
}
