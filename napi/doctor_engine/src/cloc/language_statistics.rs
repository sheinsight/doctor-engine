use napi_derive::napi;

use super::language_type::NapiLanguageType;

#[napi(object)]
pub struct NapiLanguageStatistics {
  pub language: NapiLanguageType,
  pub code: u32,
  pub comments: u32,
  pub blanks: u32,
}
