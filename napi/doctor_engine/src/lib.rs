mod lint;
mod log;

use std::collections::HashMap;

use doctor_ext::Messages;
use miette::{LabeledSpan, MietteDiagnostic, Severity, SourceSpan};
use napi::Result;

pub use lint::*;
pub use log::*;
use napi_derive::napi;

#[napi(object)]
pub struct DoctorOptions {
  pub verbose: Option<bool>,
  pub globals: Option<HashMap<String, String>>,
  pub ignore: Option<Vec<String>>,
}

#[napi(object)]
pub struct NapiSourceSpan {
  pub offset: u32,
  pub length: u32,
}

impl From<SourceSpan> for NapiSourceSpan {
  fn from(span: SourceSpan) -> Self {
    NapiSourceSpan {
      offset: span.offset() as u32,
      length: span.len() as u32,
    }
  }
}

#[napi(object)]
pub struct NapiLabeledSpan {
  pub label: Option<String>,
  pub span: NapiSourceSpan,
  pub primary: bool,
}

impl From<LabeledSpan> for NapiLabeledSpan {
  fn from(span: LabeledSpan) -> Self {
    NapiLabeledSpan {
      label: span.label().map(|s| s.to_string()),
      span: NapiSourceSpan::from(span.inner().to_owned()),
      primary: span.primary(),
    }
  }
}

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

#[napi(object)]
pub struct NapiDiagnostics {
  pub message: String,
  pub code: Option<String>,
  pub severity: Option<NapiSeverity>,
  pub help: Option<String>,
  pub url: Option<String>,
  pub labels: Option<Vec<NapiLabeledSpan>>,
}

impl From<MietteDiagnostic> for NapiDiagnostics {
  fn from(diagnostic: MietteDiagnostic) -> Self {
    NapiDiagnostics {
      message: diagnostic.message,
      code: diagnostic.code,
      severity: diagnostic.severity.map(|s| s.into()),
      help: diagnostic.help,
      url: diagnostic.url,
      labels: diagnostic
        .labels
        .map(|labels| labels.into_iter().map(|l| l.into()).collect()),
    }
  }
}

#[napi(object)]
pub struct NapiMessages {
  pub source_code: String,
  pub source_path: String,
  pub diagnostics: Vec<NapiDiagnostics>,
}

impl From<Messages> for NapiMessages {
  fn from(messages: Messages) -> Self {
    NapiMessages {
      source_code: messages.source_code,
      source_path: messages.source_path,
      diagnostics: messages.diagnostics.into_iter().map(|d| d.into()).collect(),
    }
  }
}

#[napi]
pub async fn doctor(cwd: String, _options: DoctorOptions) -> Result<Vec<NapiMessages>> {
  let options = doctor::DoctorOptions::builder()
    .max_render_count(100)
    .with_dashboard(true)
    .build();
  let messages =
    doctor::doctor(cwd, options).map_err(|e| napi::Error::new(napi::Status::GenericFailure, e))?;
  Ok(messages.into_iter().map(|m| m.into()).collect())
}
