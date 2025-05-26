use messages::NapiMessages;
use napi::Result;
use napi_derive::napi;

mod diagnostics;
mod labeled_span;
mod location;
mod messages;
mod position;
mod severity;
mod source_span;

#[napi(object)]
pub struct VerifyStandardsOptions {
  pub verbose: Option<bool>,
  pub max_render_count: Option<u32>,
  pub with_dashboard: Option<bool>,
}

impl Into<doctor::standards::VerifyStandardsOptions> for VerifyStandardsOptions {
  fn into(self) -> doctor::standards::VerifyStandardsOptions {
    doctor::standards::VerifyStandardsOptions::builder()
      .max_render_count(self.max_render_count.unwrap_or(100) as usize)
      .with_dashboard(self.with_dashboard.unwrap_or(true))
      .build()
  }
}

impl Default for VerifyStandardsOptions {
  fn default() -> Self {
    Self {
      verbose: None,
      max_render_count: Some(100),
      with_dashboard: Some(true),
    }
  }
}

#[napi]
pub async fn verify_standards(
  cwd: String,
  opts: Option<VerifyStandardsOptions>,
) -> Result<Vec<NapiMessages>> {
  let opts = opts.unwrap_or_default();

  let messages = doctor::standards::verify_standards(cwd, opts.into())
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e))?;

  Ok(messages.into_iter().map(|m| m.into()).collect())
}
