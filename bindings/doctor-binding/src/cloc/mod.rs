use napi::Result;
use napi_derive::napi;
mod lang_stats;
mod lang_type;
mod opts;

pub use lang_stats::*;
pub use lang_type::*;
pub use opts::*;

#[napi]
pub fn get_lang_stats(paths: Vec<String>, opts: Option<Opts>) -> Result<Vec<LangStats>> {
  let stats = doctor_cloc::get_lang_stats(&paths, opts.map(Into::into))
    .into_iter()
    .map(LangStats::from)
    .collect::<Vec<_>>();

  Ok(stats)
}
