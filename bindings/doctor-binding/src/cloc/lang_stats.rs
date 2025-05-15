use napi_derive::napi;

use super::LangType;

#[derive(Debug)]
#[napi(object)]
pub struct LangStats {
  pub lang: LangType,
  pub code: u32,
  pub comments: u32,
  pub blanks: u32,
}

impl From<doctor_cloc::Stats> for LangStats {
  fn from(stats: doctor_cloc::Stats) -> Self {
    Self {
      lang: stats.lang.into(),
      code: stats.code as u32,
      comments: stats.comments as u32,
      blanks: stats.blanks as u32,
    }
  }
}
