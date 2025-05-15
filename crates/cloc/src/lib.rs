use std::path::Path;

use tokei::{Config, Languages};

mod stats;

pub use stats::Stats;

pub fn get_languages_statistics<P: AsRef<Path>>(path: &[P]) -> Vec<Stats> {
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

  let lang_stats = languages.into_iter().map(Stats::from).collect::<Vec<_>>();

  lang_stats
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_count_lines() {
    let languages_statistics = get_languages_statistics(&["/Users/10015448/Git/csp-new"]);

    println!("{:#?}", languages_statistics);
  }
}
