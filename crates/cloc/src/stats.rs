use tokei::{Language, LanguageType};

#[derive(Debug)]
pub struct Stats {
  pub lang: LanguageType,
  pub code: u32,
  pub comments: u32,
  pub blanks: u32,
}

impl From<(LanguageType, Language)> for Stats {
  fn from((lang, stats): (LanguageType, Language)) -> Self {
    Self {
      lang,
      code: stats.code as u32,
      comments: stats.comments as u32,
      blanks: stats.blanks as u32,
    }
  }
}
