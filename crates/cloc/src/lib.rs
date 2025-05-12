use std::path::Path;

use tokei::{Config, LanguageType, Languages};

#[derive(Debug)]
pub struct LanguageStatistics {
  pub language: LanguageType,
  pub code: u32,
  pub comments: u32,
  pub blanks: u32,
}

pub fn get_languages_statistics<P: AsRef<Path>>(
  path: &[P],
) -> Result<Vec<LanguageStatistics>, String> {
  let config = Config::default();
  let mut languages = Languages::new();

  languages.get_statistics(
    path,
    &[
      ".git",
      "node_modules",
      "**/node_modules/**",
      "**/dist/**",
      "**/build/**",
      "**/target/**",
    ],
    &config,
  );

  let languages_statistics = languages
    .into_iter()
    .map(|(lang, stats)| LanguageStatistics {
      language: lang,
      code: stats.code as u32,
      comments: stats.comments as u32,
      blanks: stats.blanks as u32,
    })
    .collect::<Vec<_>>();

  Ok(languages_statistics)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_count_lines() {
    let languages_statistics = get_languages_statistics(&["/Users/10015448/Git/csp-new"]).unwrap();

    println!("{:#?}", languages_statistics);
  }
}
