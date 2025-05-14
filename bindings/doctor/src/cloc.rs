use doctor_cloc::LanguageStatistics;
use napi::Result;
use napi_derive::napi;

#[napi]
pub fn get_languages_statistics(paths: Vec<String>) -> Result<Vec<LanguageStatistics>> {
  let languages_statistics = doctor_cloc::get_languages_statistics(&paths)
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e))?;
  Ok(languages_statistics)
}
