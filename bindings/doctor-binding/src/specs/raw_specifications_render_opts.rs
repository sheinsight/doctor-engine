use doctor::specs::SpecificationsRenderOpts;
use napi_derive::napi;

#[napi(object)]
#[derive(Clone, Debug)]
pub struct RawSpecificationsRenderOpts {
  pub with_dashboard: Option<bool>,
  pub max_render_count: Option<u32>,
  pub quiet: Option<bool>,
}

impl Default for RawSpecificationsRenderOpts {
  fn default() -> Self {
    Self {
      with_dashboard: Some(true),
      max_render_count: None,
      quiet: Some(false),
    }
  }
}

impl Into<SpecificationsRenderOpts> for RawSpecificationsRenderOpts {
  fn into(self) -> SpecificationsRenderOpts {
    SpecificationsRenderOpts {
      with_dashboard: self.with_dashboard.unwrap_or(false),
      max_render_count: self.max_render_count,
      ..Default::default()
    }
  }
}
