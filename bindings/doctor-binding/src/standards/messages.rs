use doctor_core::Messages;
use napi_derive::napi;

use super::diagnostics::NapiDiagnostics;

#[napi(object)]
pub struct NapiMessages {
  // pub source_code: String,
  pub source_path: String,
  pub diagnostics: Vec<NapiDiagnostics>,
}

impl From<Messages> for NapiMessages {
  fn from(messages: Messages) -> Self {
    let source_code = messages.source_code.clone();
    NapiMessages {
      // source_code: messages.source_code,
      source_path: messages.source_path,
      diagnostics: messages
        .diagnostics
        .into_iter()
        .map(|d| (source_code.clone(), d))
        .map(NapiDiagnostics::from)
        .collect(),
    }
  }
}
