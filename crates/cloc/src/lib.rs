mod language_statistics;
mod language_type;

use std::path::Path;

use tokei::{Config, Languages};

pub use language_statistics::*;
pub use language_type::*;

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
    .map(LanguageStatistics::from)
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
