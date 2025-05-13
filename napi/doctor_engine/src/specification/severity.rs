use miette::Severity;
use napi_derive::napi;

#[napi(string_enum)]
pub enum NapiSeverity {
  Error,
  Warning,
  Advice,
}

impl From<Severity> for NapiSeverity {
  fn from(severity: Severity) -> Self {
    match severity {
      Severity::Error => NapiSeverity::Error,
      Severity::Warning => NapiSeverity::Warning,
      Severity::Advice => NapiSeverity::Advice,
    }
  }
}
