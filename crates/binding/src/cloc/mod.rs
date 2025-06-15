use doctor::core::Ignore;
use napi::Result;
use napi_derive::napi;
mod js_language_stats;
mod js_language_type;
mod raw_opts;

pub use js_language_stats::*;
pub use js_language_type::*;
pub use raw_opts::*;

#[napi]
pub fn cloc(paths: Vec<String>, opts: Option<RawClocOpts>) -> Result<Vec<JsLanguageStats>> {
  let ignore = opts
    .and_then(|o| o.ignore)
    .map(Ignore::from)
    .unwrap_or_default();

  let stats = doctor::cloc::cloc(&paths, Some(doctor::cloc::Opts { ignore }));

  let stats = stats
    .into_iter()
    .map(JsLanguageStats::from)
    .collect::<Vec<_>>();

  Ok(stats)
}
