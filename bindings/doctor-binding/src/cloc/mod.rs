use napi::Result;
use napi_derive::napi;
mod lang_stats;
mod lang_type;

pub use lang_stats::*;
pub use lang_type::*;

#[napi]
pub fn get_lang_stats(paths: Vec<String>) -> Result<Vec<LangStats>> {
  let stats = doctor_cloc::get_languages_statistics(&paths)
    .into_iter()
    .map(LangStats::from)
    .collect::<Vec<_>>();

  Ok(stats)
}
