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

// #[napi(object)]
// pub struct VerifyStandardsOptions {
//   pub verbose: Option<bool>,
//   pub max_render_count: Option<u32>,
//   pub with_dashboard: Option<bool>,
// }

// impl Into<doctor::standards::VerifyStandardsOptions> for VerifyStandardsOptions {
//   fn into(self) -> doctor::standards::VerifyStandardsOptions {
//     doctor::standards::VerifyStandardsOptions::builder()
//       .max_render_count(self.max_render_count.unwrap_or(100) as usize)
//       .with_dashboard(self.with_dashboard.unwrap_or(true))
//       .build()
//   }
// }

// impl Default for VerifyStandardsOptions {
//   fn default() -> Self {
//     Self {
//       verbose: None,
//       max_render_count: Some(100),
//       with_dashboard: Some(true),
//     }
//   }
// }

// #[napi]
// pub async fn verify_standards(
//   cwd: String,
//   opts: Option<VerifyStandardsOptions>,
// ) -> Result<Vec<NapiMessages>> {
//   let opts = opts.unwrap_or_default();

//   let messages = doctor::standards::verify_standards(cwd, opts.into())
//     .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e))?;

//   Ok(messages.into_iter().map(|m| m.into()).collect())
// }

#[napi]
pub struct Standards {
  #[napi(skip)]
  pub standards: doctor::standards::Standards,
}

#[napi(object)]
pub struct StandardsOpts {
  pub with_dashboard: Option<bool>,
  pub max_render_count: Option<u32>,
  pub quiet: Option<bool>,
}

#[napi]
impl Standards {
  #[napi(factory)]
  pub fn create(cwd: String, opts: Option<StandardsOpts>) -> Standards {
    let standards = doctor::standards::Standards::create(
      cwd,
      doctor::standards::StandardsOpts {
        with_dashboard: opts.as_ref().and_then(|o| o.with_dashboard).unwrap_or(true),
        quiet: opts.as_ref().and_then(|o| o.quiet).unwrap_or(false),
        max_render_count: opts
          .as_ref()
          .map(|o| o.max_render_count)
          .unwrap_or(Some(500)),
      },
    );
    Standards { standards }
  }

  fn convert_messages(&self, messages: Vec<doctor_core::Messages>) -> Vec<NapiMessages> {
    messages
      .into_iter()
      .map(NapiMessages::from)
      .collect::<Vec<_>>()
  }

  #[napi]
  pub async fn validate_npmrc(&self) -> Result<Vec<NapiMessages>> {
    let res = self
      .standards
      .validate_npmrc()
      .await
      .map(|items| self.convert_messages(items))
      .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e))?;

    Ok(res)
  }

  #[napi]
  pub async fn validate_node_version(&self) -> Result<Vec<NapiMessages>> {
    let res = self
      .standards
      .validate_node_version()
      .await
      .map(|items| self.convert_messages(items))
      .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e))?;

    Ok(res)
  }

  #[napi]
  pub async fn validate_package_json(&self) -> Result<Vec<NapiMessages>> {
    let res = self
      .standards
      .validate_package_json()
      .await
      .map(|items| self.convert_messages(items))
      .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e))?;

    Ok(res)
  }

  #[napi]
  pub async fn validate_lint(&self) -> Result<Vec<NapiMessages>> {
    let res = self
      .standards
      .validate_lint()
      .await
      .map(|items| self.convert_messages(items))
      .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e))?;

    Ok(res)
  }

  #[napi]
  pub async fn validate_all(&self) -> Result<Vec<NapiMessages>> {
    let res = self
      .standards
      .validate_all()
      .await
      .map(|items| self.convert_messages(items))
      .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e))?;

    Ok(res)
  }
}
