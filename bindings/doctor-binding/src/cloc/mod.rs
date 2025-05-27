use doctor_cloc::LanguageStats;
use doctor_walk::WalkIgnore;
use napi::Result;
use napi_derive::napi;
mod lang_stats;
mod lang_type;
mod opts;

pub use lang_stats::*;
pub use lang_type::*;
pub use opts::*;

#[napi]
pub fn get_lang_stats(paths: Vec<String>, opts: Option<JsOpts>) -> Result<Vec<JsLangStats>> {
  let ignore = opts
    .and_then(|o| o.ignore)
    .map(WalkIgnore::from)
    .unwrap_or_default();

  let stats = LanguageStats::builder().cwd(paths).ignore(ignore).build();

  let stats = stats
    .stats()
    .into_iter()
    .map(JsLangStats::from)
    .collect::<Vec<_>>();

  Ok(stats)
}
