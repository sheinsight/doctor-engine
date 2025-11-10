#![recursion_limit = "256"]

use doctor_core::Ignore;
use doctor_lint::{GlobalValue, Globals, LintValidator, inner::Category20250601Inner};
use oxc::diagnostics::{
  DiagnosticService, Error, GraphicalReportHandler,
  reporter::{DiagnosticReporter, DiagnosticResult},
};
use oxc_linter::{
  AllowWarnDeny, ConfigStore, ConfigStoreBuilder, ExternalPluginStore, FixKind, LintOptions,
  LintRunner, LintServiceOptions, Linter,
};
use rustc_hash::FxHashMap;

use std::{
  path::{Path, PathBuf},
  sync::Arc,
  time::Instant,
};

#[derive(Debug)]
pub struct MyReporter {
  handler: GraphicalReportHandler,
  diagnostics: Vec<Error>,
}

impl Default for MyReporter {
  fn default() -> Self {
    Self {
      handler: GraphicalReportHandler::new(),
      diagnostics: Vec::new(),
    }
  }
}

impl DiagnosticReporter for MyReporter {
  fn render_error(&mut self, error: Error) -> Option<String> {
    // 收集诊断信息
    self.diagnostics.push(error);
    None
  }

  fn finish(&mut self, _result: &DiagnosticResult) -> Option<String> {
    let mut output = String::new();

    // 渲染所有诊断信息
    for diagnostic in &self.diagnostics {
      self
        .handler
        .render_report(&mut output, diagnostic.as_ref())
        .unwrap();
    }

    Some(output)
  }
}

#[allow(unused)]
fn h() -> anyhow::Result<()> {
  let reporter = MyReporter::default();

  let (mut diagnostic_service, tx_error) = DiagnosticService::new(Box::new(reporter));

  let cwd = "/Users/10015448/Git/metric-front";
  // 2. 配置并创建 LintRunner
  let options = LintServiceOptions::new(PathBuf::from(cwd));

  // 1. 创建 ExternalPluginStore（通常为空）
  let mut external_plugin_store = ExternalPluginStore::default();

  let category = Category20250601Inner::default();

  let config =
    ConfigStoreBuilder::from_oxlintrc(true, category.into(), None, &mut external_plugin_store)?
      .build(&external_plugin_store)?;

  // 3. 创建 ConfigStore
  let config_store = ConfigStore::new(config, FxHashMap::default(), external_plugin_store);

  // 4. 创建 Linter
  let linter = Linter::new(
    LintOptions::default(),
    config_store,
    None, // external_linter（JS 插件）
  )
  .with_fix(FixKind::None) // 可选：启用自动修复
  .with_report_unused_directives(Some(AllowWarnDeny::Warn)); // 可选

  let lint_runner = LintRunner::builder(options, linter)
    .with_type_aware(true)
    .build()
    .map_err(anyhow::Error::msg)?;

  let files = vec![
    Arc::from(Path::new("/Users/10015448/Git/metric-front/src/index.tsx").as_os_str()),
    Arc::from(Path::new("/Users/10015448/Git/metric-front/src/pages/npm/index.tsx").as_os_str()),
    Arc::from(Path::new("/Users/10015448/Git/metric-front/src/pages/metric/index.tsx").as_os_str()),
  ];

  // 3. 执行 linting（错误通过 tx_error 异步发送）

  match lint_runner.lint_files(&files, tx_error.clone(), None) {
    Ok(lint_runner) => {
      // 可选：报告未使用的禁用指令
      lint_runner.report_unused_directives(Some(AllowWarnDeny::Warn), &tx_error);
    }
    Err(err) => {
      eprintln!("Linting failed: {}", err);
    }
  }

  // 4. 关闭发送器（必须！）
  drop(tx_error);

  // 5. 接收并处理所有诊断信息
  let result = diagnostic_service.run(&mut std::io::stdout());

  // 6. 检查结果
  println!(
    "Errors: {}, Warnings: {}, Result: {:?}",
    result.errors_count(),
    result.warnings_count(),
    result
  );
  Ok(())
}

#[allow(unused)]
fn a() -> anyhow::Result<()> {
  let start_time = Instant::now();

  let cwd = "/Users/10015448/Git/metric-front";

  eprintln!("1--->>>");

  let category = Category20250601Inner::default();

  // let config = category.get_config();
  // std::fs::write("demo.json", serde_json::to_string_pretty(&config).unwrap()).unwrap();

  // let category = Category::V20250601Inner(category);

  eprintln!("2--->>> 规范 ");

  let mut globals = Globals::default();

  globals.insert("a".to_string(), GlobalValue::Writable);

  let rc = category.into();

  std::fs::write("oxlintrc.json", serde_json::to_string_pretty(&rc).unwrap())?;

  eprintln!("3--->>>");

  let ignore = Ignore(vec!["**/node_modules/**".to_string()]);

  let linter_runner = LintValidator::builder()
    .cwd(cwd.to_string().into())
    .ignore(ignore)
    .with_show_report(true)
    .oxlintrc(rc)
    .build();

  eprintln!("4--->>>");

  let res = linter_runner.run();

  eprintln!("5--->>>{:#?}", res.is_err());

  let duration = start_time.elapsed();
  eprintln!("Total execution time: {:?}", duration);

  Ok(())
}

fn main() -> anyhow::Result<()> {
  h()?;
  // a()?;
  Ok(())
}
