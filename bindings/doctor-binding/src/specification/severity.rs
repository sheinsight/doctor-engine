use napi_derive::napi;

#[napi(string_enum)]
pub enum Severity {
  Error,
  Warning,
  Advice,
}

impl From<miette::Severity> for Severity {
  fn from(severity: miette::Severity) -> Self {
    match severity {
      miette::Severity::Error => Severity::Error,
      miette::Severity::Warning => Severity::Warning,
      miette::Severity::Advice => Severity::Advice,
    }
  }
}
