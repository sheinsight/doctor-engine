use std::path::Path;

use doctor_walk::WalkIgnore;
use tokei::{Config, Languages, Sort};

mod opts;
mod stats;

pub use opts::Opts;
pub use stats::Stats;

pub fn get_lang_stats<P: AsRef<Path>>(path: &[P], opts: Option<Opts>) -> Vec<Stats> {
  let config = Config {
    sort: Some(Sort::Code),
    ..Config::default()
  };
  let mut languages = Languages::new();

  let mut def_ignore = WalkIgnore::default();

  let i = opts.unwrap_or_default().ignore;

  def_ignore.extend(i.iter().cloned());

  languages.get_statistics(
    path,
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
    let lang_stats = get_lang_stats(&["/Users/10015448/Git/csp-new"], None);

    println!("{:#?}", lang_stats);
  }
}
