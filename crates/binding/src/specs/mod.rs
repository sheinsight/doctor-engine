use doctor::core::{Messages, ValidatorError};
use js_messages::JsMessages;
use napi::Result;
use napi_derive::napi;

use crate::specs::raw_specifications_render_opts::RawSpecificationsRenderOpts;

mod js_diagnostics;
mod js_labeled_span;
mod js_location;
mod js_messages;
mod js_position;
mod js_severity;
mod js_source_span;
mod raw_specifications_render_opts;

#[napi]
pub struct JsSpecifications {
  #[napi(skip)]
  pub standards: doctor::specs::Specifications,

  opts: Option<RawSpecificationsRenderOpts>,
}

impl JsSpecifications {
  // 简单的辅助函数，避免生命周期问题
  fn to_napi_error(err: ValidatorError) -> napi::Error {
    napi::Error::new(napi::Status::GenericFailure, err.to_string())
  }

  fn convert_messages(messages: Vec<Messages>) -> Vec<JsMessages> {
    messages.into_iter().map(JsMessages::from).collect()
  }

  fn render_messages(&self, messages: &Vec<Messages>) {
    if let Some(opts) = self.opts.as_ref() {
      let quiet = opts.quiet.unwrap_or(false);
      if !quiet {
        let mut writer = doctor::specs::ConsoleWriter::default();
        self
          .standards
          .render_with_writer(messages, &mut writer, opts.clone().into());
      }
    }
  }
}

#[napi]
impl JsSpecifications {
  #[napi(factory)]
  pub fn create(cwd: String, opts: Option<RawSpecificationsRenderOpts>) -> JsSpecifications {
    let standards = doctor::specs::Specifications::create(cwd);
    JsSpecifications { standards, opts }
  }

  #[napi]
  pub async fn fix_lint(&self) -> Result<Vec<JsMessages>> {
    let res = self.standards.fix_lint().map_err(Self::to_napi_error)?;
    self.render_messages(&res);
    Ok(Self::convert_messages(res))
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
  pub async fn validate_syntax(&self) -> Result<Vec<JsMessages>> {
    let res = self
      .standards
      .validate_syntax()
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
