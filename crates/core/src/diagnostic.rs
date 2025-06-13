use std::ops::{Deref, DerefMut};

use miette::MietteDiagnostic;
use oxc::diagnostics::{OxcDiagnostic, Severity};

pub struct Diagnostic(pub MietteDiagnostic);

impl Into<MietteDiagnostic> for Diagnostic {
  fn into(self) -> MietteDiagnostic {
    self.0
  }
}

impl Deref for Diagnostic {
  type Target = MietteDiagnostic;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Diagnostic {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl From<MietteDiagnostic> for Diagnostic {
  fn from(value: MietteDiagnostic) -> Self {
    Diagnostic(value)
  }
}

impl From<OxcDiagnostic> for Diagnostic {
  fn from(oxc_diagnostics: OxcDiagnostic) -> Self {
    let mut diagnostic = MietteDiagnostic::new(oxc_diagnostics.message.to_string());

    if let Some(help) = &oxc_diagnostics.help {
      diagnostic = diagnostic.with_help(help.to_string());
    }

    let code = oxc_diagnostics
      .code
      .scope
      .as_ref()
      .map_or(String::from("unknown"), |s| s.to_string());

    let number = oxc_diagnostics
      .code
      .number
      .as_ref()
      .map_or(String::from("unknown"), |s| s.to_string());

    diagnostic = diagnostic.with_code(format!("{code}({number})"));

    match oxc_diagnostics.severity {
      Severity::Error => diagnostic = diagnostic.with_severity(miette::Severity::Error),
      Severity::Warning => diagnostic = diagnostic.with_severity(miette::Severity::Warning),
      Severity::Advice => diagnostic = diagnostic.with_severity(miette::Severity::Advice),
    }

    let labels = oxc_diagnostics.labels.as_ref().map(|labels| {
      labels
        .iter()
        .map(|label| {
          let label_text = label.label().map(|s| s.to_string());
          let miette_label = miette::LabeledSpan::new(label_text, label.offset(), label.len());
          miette_label
        })
        .collect::<Vec<_>>()
    });

    if let Some(labels) = labels {
      diagnostic = diagnostic.with_labels(labels);
    }

    if let Some(url) = &oxc_diagnostics.url {
      diagnostic = diagnostic.with_url(url.to_string());
    }

    Diagnostic(diagnostic)
  }
}
