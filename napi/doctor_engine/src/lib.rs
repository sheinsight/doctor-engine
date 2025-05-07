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
pub async fn doctor(cwd: String, options: DoctorOptions) -> Result<()> {
  let messages =
    doctor::doctor(cwd).map_err(|e| napi::Error::new(napi::Status::GenericFailure, e))?;
  Ok(())
}
