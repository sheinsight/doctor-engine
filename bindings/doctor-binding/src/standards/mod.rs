use messages::JsMessages;
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
#[derive(Clone, Debug)]
pub struct JsRenderOpts {
  pub with_dashboard: Option<bool>,
  pub max_render_count: Option<u32>,
  pub quiet: Option<bool>,
}

impl Default for JsRenderOpts {
  fn default() -> Self {
    Self {
      with_dashboard: Some(true),
      max_render_count: None,
      quiet: Some(false),
    }
  }
}

impl Into<doctor::standards::RenderOpts> for JsRenderOpts {
  fn into(self) -> doctor::standards::RenderOpts {
    doctor::standards::RenderOpts {
      with_dashboard: self.with_dashboard.unwrap_or(false),
      max_render_count: self.max_render_count,
      ..Default::default()
    }
  }
}

#[napi]
pub struct Standards {
  #[napi(skip)]
  pub standards: doctor::standards::Standards,

  opts: Option<JsRenderOpts>,
}

impl Standards {
  // 简单的辅助函数，避免生命周期问题
  fn to_napi_error(err: doctor_core::ValidatorError) -> napi::Error {
    napi::Error::new(napi::Status::GenericFailure, err.to_string())
  }

  fn convert_messages(messages: Vec<doctor_core::Messages>) -> Vec<JsMessages> {
    messages.into_iter().map(JsMessages::from).collect()
  }

  fn render_messages(&self, messages: &Vec<doctor_core::Messages>) {
    if let Some(opts) = self.opts.as_ref() {
      let quiet = opts.quiet.unwrap_or(false);
      if !quiet {
        self.standards.render(messages, opts.clone().into());
      }
    }
  }
}

#[napi]
impl Standards {
  #[napi(factory)]
  pub fn create(cwd: String, opts: Option<JsRenderOpts>) -> Standards {
    let standards = doctor::standards::Standards::create(cwd);
    Standards { standards, opts }
  }

  #[napi]
  pub async fn validate_npmrc(&self) -> Result<Vec<JsMessages>> {
    let res = self
      .standards
      .validate_npmrc()
      .map_err(Self::to_napi_error)?;

    self.render_messages(&res);

    Ok(Self::convert_messages(res))
  }

  #[napi]
  pub async fn validate_node_version(&self) -> Result<Vec<JsMessages>> {
    let res = self
      .standards
      .validate_node_version()
      .map_err(Self::to_napi_error)?;

    self.render_messages(&res);

    Ok(Self::convert_messages(res))
  }

  #[napi]
  pub async fn validate_package_json(&self) -> Result<Vec<JsMessages>> {
    let res = self
      .standards
      .validate_package_json()
      .map_err(Self::to_napi_error)?;

    self.render_messages(&res);

    Ok(Self::convert_messages(res))
  }

  #[napi]
  pub async fn validate_lint(&self) -> Result<Vec<JsMessages>> {
    let res = self
      .standards
      .validate_lint()
      .map_err(Self::to_napi_error)?;

    self.render_messages(&res);

    Ok(Self::convert_messages(res))
  }

  #[napi]
  pub async fn validate_all(&self) -> Result<Vec<JsMessages>> {
    let res = self.standards.validate_all().map_err(Self::to_napi_error)?;

    self.render_messages(&res);

    Ok(Self::convert_messages(res))
  }
}
