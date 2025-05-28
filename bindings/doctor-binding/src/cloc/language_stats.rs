use napi_derive::napi;

use super::JsLanguageType;

#[derive(Debug)]
#[napi(object)]
pub struct JsLanguageStats {
  pub lang: JsLanguageType,
  pub code: u32,
  pub comments: u32,
  pub blanks: u32,
}

impl From<doctor_cloc::Stats> for JsLanguageStats {
  fn from(stats: doctor_cloc::Stats) -> Self {
    Self {
      lang: stats.lang.into(),
      code: stats.code as u32,
      comments: stats.comments as u32,
      blanks: stats.blanks as u32,
    }
  }
}
