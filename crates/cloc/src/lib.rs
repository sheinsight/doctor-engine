use std::path::Path;

use doctor_core::Ignore;
use tokei::{Config, Languages, Sort};

mod opts;
mod stats;

pub use opts::Opts;
pub use stats::Stats;

pub fn cloc(paths: &[impl AsRef<Path>], opts: Option<Opts>) -> Vec<Stats> {
  let config = Config {
    sort: Some(Sort::Code),
    ..Config::default()
  };
  let mut languages = Languages::new();

  let mut def_ignore = Ignore::default();

  let ignore = opts.map(|o| o.ignore).unwrap_or_default();

  def_ignore.extend(ignore.iter().cloned());

  languages.get_statistics(
    &paths,
    &def_ignore.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
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
    let opts = Opts {
      ignore: Ignore::from(&["**/node_modules/**", "node_modules"]),
    };

    let lang_stats = cloc(&["/Users/10015448/Git/csp-new"], Some(opts));

    println!("{:#?}", lang_stats);
  }
}
