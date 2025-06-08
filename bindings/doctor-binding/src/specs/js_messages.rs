use doctor_core::Messages;
use napi_derive::napi;

use super::js_diagnostics::JsDiagnostics;

#[napi(object)]
pub struct JsMessages {
  // pub source_code: String,
  pub source_path: String,
  pub diagnostics: Vec<JsDiagnostics>,
}

impl From<Messages> for JsMessages {
  fn from(messages: Messages) -> Self {
    let source_code = messages.source_code.clone();
    JsMessages {
      // source_code: messages.source_code,
      source_path: messages.source_path,
      diagnostics: messages
        .diagnostics
        .into_iter()
        .map(|d| (source_code.clone(), d))
        .map(JsDiagnostics::from)
        .collect(),
    }
  }
}
