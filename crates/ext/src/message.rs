use miette::MietteDiagnostic;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct Messages {
  pub source_code: String,
  pub diagnostics: Vec<MietteDiagnostic>,
}

impl Messages {
  pub fn new(source_code: String, diagnostics: Vec<MietteDiagnostic>) -> Self {
    Self {
      source_code,
      diagnostics,
    }
  }

  pub fn render(&self) {
    for diagnostic in &self.diagnostics {
      let report =
        miette::Report::new(diagnostic.to_owned()).with_source_code(self.source_code.clone());
      eprintln!("{:?}", report);
    }
  }
}
