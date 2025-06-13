use oxc::diagnostics::OxcDiagnostic;

#[derive(Debug)]
pub struct FileDiagnostic {
  pub file_path: String,
  pub diagnostics: Vec<OxcDiagnostic>,
}
