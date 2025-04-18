use oxc_diagnostics::OxcDiagnostic;

use crate::common::named_source;

#[derive(Debug)]
pub struct FileDiagnostic {
  pub path_with_source: named_source::PathWithSource,
  pub diagnostics: Vec<OxcDiagnostic>,
}
