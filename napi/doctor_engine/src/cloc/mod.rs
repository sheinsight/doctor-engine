use language_statistics::NapiLanguageStatistics;
use napi::Result;
use napi_derive::napi;

mod language_statistics;
mod language_type;

#[napi]
pub fn get_languages_statistics(paths: Vec<String>) -> Result<Vec<NapiLanguageStatistics>> {
  let languages_statistics = doctor_cloc::get_languages_statistics(&paths)
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e))?;

  let languages_statistics = languages_statistics
    .into_iter()
    .map(|language_statistics| NapiLanguageStatistics {
      language: language_statistics.language.into(),
      code: language_statistics.code,
      comments: language_statistics.comments,
      blanks: language_statistics.blanks,
    })
    .collect::<Vec<_>>();

  Ok(languages_statistics)
}
