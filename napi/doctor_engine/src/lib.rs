mod lint;
mod log;

use std::collections::HashMap;

use napi::Result;

pub use lint::*;
pub use log::*;
use napi_derive::napi;

#[napi(object)]
pub struct DoctorOptions {
  pub verbose: Option<bool>,
  pub globals: Option<HashMap<String, String>>,
  pub ignore: Option<Vec<String>>,
}

#[napi]
pub async fn doctor(cwd: String, _options: DoctorOptions) -> Result<()> {
  let options = doctor::DoctorOptions::builder()
    .max_render_count(100)
    .with_dashboard(true)
    .build();
  let _messages =
    doctor::doctor(cwd, options).map_err(|e| napi::Error::new(napi::Status::GenericFailure, e))?;
  Ok(())
}
