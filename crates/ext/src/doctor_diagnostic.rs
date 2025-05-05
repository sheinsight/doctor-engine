// pub struct Code {
//   pub scope: Option<String>,
//   pub number: Option<String>,
// }

// pub enum Severity {
//   Error,
//   Warning,
// }

// pub struct SourceSpan {
//   pub offset: usize,
//   pub length: usize,
// }

// pub struct LabelSpan {
//   pub label: Option<String>,
//   pub span: Option<SourceSpan>,
//   pub primary: bool,
// }

// pub struct DoctorDiagnostic {
//   pub message: String,
//   pub labels: Option<Vec<LabelSpan>>,
//   pub help: Option<String>,
//   pub severity: Severity,
//   pub code: Option<Code>,
//   pub url: Option<String>,
// }

#[cfg(test)]
mod tests {
  use miette::{LabeledSpan, MietteDiagnostic};

  #[test]
  fn test_doctor_diagnostic() {
    let _ = miette::set_hook(Box::new(|_| {
      Box::new(
        miette::MietteHandlerOpts::new()
          .terminal_links(true)
          .unicode(true)
          .context_lines(3)
          .tab_width(4)
          .break_words(true)
          .force_graphical(true)
          .build(),
      )
    }));

    let labels = vec![
      // LabeledSpan::at(1..10, "This is a test diagnostic".to_string()),
      LabeledSpan::at(50..55, "This is a test diagnostic".to_string()),
      LabeledSpan::new_primary_with_span(
        Some("This is a test diagnostic".to_string()),
        miette::SourceSpan::from((10, 20)),
      ),
    ];

    let source_code = r#"
      
      export const xxxx = window.baseName === 'xxxx'
        ? `${process.env.xxx}/xxx/xxx/xxx`
        : `${process.env.xxx}/xxx/xxx`;
    "#;

    let diagnostic = MietteDiagnostic::new("this is a test diagnostic")
      .with_labels(labels)
      .with_help("This is a test help".to_string())
      .with_code("test/code")
      .with_severity(miette::Severity::Error)
      .with_url("https://www.google.com".to_string());

    let report = miette::Report::new(diagnostic).with_source_code(source_code);

    println!("{:?}", report);

    // println!("{:?}", report);
  }
}
