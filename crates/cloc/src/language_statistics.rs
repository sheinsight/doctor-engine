use crate::language_type::LanguageType;

#[derive(Debug)]
#[cfg_attr(feature = "napi", napi_derive::napi(object))]
pub struct LanguageStatistics {
  pub language: LanguageType,
  pub code: u32,
  pub comments: u32,
  pub blanks: u32,
}

impl From<(tokei::LanguageType, tokei::Language)> for LanguageStatistics {
  fn from((lang, stats): (tokei::LanguageType, tokei::Language)) -> Self {
    Self {
      language: lang.into(),
      code: stats.code as u32,
      comments: stats.comments as u32,
      blanks: stats.blanks as u32,
    }
  }
}
