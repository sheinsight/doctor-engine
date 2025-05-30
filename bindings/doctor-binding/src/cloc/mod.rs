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
pub fn cloc(paths: Vec<String>, opts: Option<JsClocOpts>) -> Result<Vec<JsLanguageStats>> {
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
