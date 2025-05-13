use messages::NapiMessages;
use napi::Result;
use napi_derive::napi;

mod diagnostics;
mod labeled_span;
mod messages;
mod severity;
mod source_span;

#[napi(object)]
pub struct DoctorOptions {
  pub verbose: Option<bool>,
  pub max_render_count: Option<u32>,
  pub with_dashboard: Option<bool>,
}

impl Into<doctor::DoctorOptions> for DoctorOptions {
  fn into(self) -> doctor::DoctorOptions {
    doctor::DoctorOptions::builder()
      .max_render_count(self.max_render_count.unwrap_or(100) as usize)
      .with_dashboard(self.with_dashboard.unwrap_or(true))
      .build()
  }
}

impl Default for DoctorOptions {
  fn default() -> Self {
    Self {
      verbose: None,
      max_render_count: Some(100),
      with_dashboard: Some(true),
    }
  }
}

#[napi]
pub async fn doctor(cwd: String, opts: Option<DoctorOptions>) -> Result<Vec<NapiMessages>> {
  let opts = opts.unwrap_or_default();

  let messages = doctor::doctor(cwd, opts.into())
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e))?;

  Ok(messages.into_iter().map(|m| m.into()).collect())
}
