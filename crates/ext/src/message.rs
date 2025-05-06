use std::ops::{Deref, DerefMut};

use miette::MietteDiagnostic;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct Messages {
  #[builder(default = String::new())]
  pub source_code: String,
  #[builder(default = Vec::new())]
  pub diagnostics: Vec<MietteDiagnostic>,
}

impl Deref for Messages {
  type Target = Vec<MietteDiagnostic>;

  fn deref(&self) -> &Self::Target {
    &self.diagnostics
  }
}

impl DerefMut for Messages {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.diagnostics
  }
}

impl Messages {
  pub fn render(&self) {
    for diagnostic in &self.diagnostics {
      let report =
        miette::Report::new(diagnostic.to_owned()).with_source_code(self.source_code.clone());
      eprintln!("{:?}", report);
    }
  }

  pub fn has_error(&self) -> bool {
    self.diagnostics.iter().any(|d| {
      if let Some(severity) = d.severity {
        severity == miette::Severity::Error
      } else {
        false
      }
    })
  }
}
