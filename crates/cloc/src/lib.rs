use doctor_core::Ignore;
use tokei::{Config, Languages, Sort};

mod opts;
mod stats;

pub use opts::Opts;
pub use stats::Stats;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct LanguageStats {
  pub cwd: Vec<String>,
  #[builder(default = Ignore::default())]
  pub ignore: Ignore,
}

impl LanguageStats {
  pub fn stats(&self) -> Vec<Stats> {
    let config = Config {
      sort: Some(Sort::Code),
      ..Config::default()
    };
    let mut languages = Languages::new();

    let mut def_ignore = Ignore::default();

    def_ignore.extend(self.ignore.iter().cloned());

    languages.get_statistics(
      &self.cwd,
      &def_ignore.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
      &config,
    );

    let lang_stats = languages.into_iter().map(Stats::from).collect::<Vec<_>>();

    lang_stats
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_count_lines() {
    let ignore = Ignore::from(&["**/node_modules/**", "node_modules"]);

    let lang_stats = LanguageStats::builder()
      .cwd(vec!["/Users/10015448/Git/csp-new".to_string()])
      .ignore(ignore)
      .build()
      .stats();

    println!("{:#?}", lang_stats);
  }
}
