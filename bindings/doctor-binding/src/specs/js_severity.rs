use napi_derive::napi;

#[napi(string_enum)]
pub enum JsSeverity {
  Error,
  Warning,
  Advice,
}

impl From<miette::Severity> for JsSeverity {
  fn from(severity: miette::Severity) -> Self {
    match severity {
      miette::Severity::Error => JsSeverity::Error,
      miette::Severity::Warning => JsSeverity::Warning,
      miette::Severity::Advice => JsSeverity::Advice,
    }
  }
}
