use doctor_cloc::Cloc;
use doctor_core::Ignore;
use napi::Result;
use napi_derive::napi;
mod language_stats;
mod language_type;
mod opts;

pub use language_stats::*;
pub use language_type::*;
pub use opts::*;

#[napi]
pub fn cloc(paths: Vec<String>, opts: Option<JsOpts>) -> Result<Vec<JsLanguageStats>> {
  let ignore = opts
    .and_then(|o| o.ignore)
    .map(Ignore::from)
    .unwrap_or_default();

  let stats = Cloc::builder().cwd(paths).ignore(ignore).build();

  let stats = stats
    .stats()
    .into_iter()
    .map(JsLanguageStats::from)
    .collect::<Vec<_>>();

  Ok(stats)
}
