use crate::language_type::LanguageType;

#[derive(Debug)]
#[cfg_attr(feature = "napi", napi_derive::napi(object))]
pub struct LanguageStatistics {
  pub language: LanguageType,
  pub code: u32,
  pub comments: u32,
  pub blanks: u32,
}
