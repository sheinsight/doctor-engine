use std::ops::{Deref, DerefMut};

use miette::MietteDiagnostic;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Clone, Debug)]
pub struct Messages {
  #[builder(default = String::new())]
  pub source_code: String,
  #[builder(default = String::new())]
  pub source_path: String,
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
  pub fn render(&self) -> Vec<String> {
    let mut reports = Vec::new();

    for diagnostic in &self.diagnostics {
      let source = miette::NamedSource::new(self.source_path.clone(), self.source_code.clone());
      let report = miette::Report::new(diagnostic.to_owned()).with_source_code(source);
      let report_str = format!("{:?}", report);
      reports.push(report_str.clone());
      eprintln!("{}", report_str);
    }

    return reports;
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
