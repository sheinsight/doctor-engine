#![recursion_limit = "256"]

use doctor_core::Ignore;
use doctor_lint::{
  Category, EnvironmentFlags, GlobalValue, Globals, LintMode, LintValidator,
  config::OxlintrcBuilder, inner::Category20250601Inner,
};
use oxc::diagnostics::{
  DiagnosticService, Error, GraphicalReportHandler,
  reporter::{DiagnosticReporter, DiagnosticResult},
};
use oxc_linter::{
  AllowWarnDeny, ConfigStore, ConfigStoreBuilder, ExternalPluginStore, FixKind, LintOptions,
  LintRunner, LintServiceOptions, Linter, Oxlintrc,
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

fn h() -> anyhow::Result<()> {
  let reporter = Box::new(oxc::diagnostics::GraphicalReportHandler::new().with_links(true));

  let (mut diagnostic_service, tx_error) = DiagnosticService::new(Box::new(MyReporter::default()));

  let cwd = "/Users/10015448/Git/metric-front";
  // 2. 配置并创建 LintRunner
  let options = LintServiceOptions::new(PathBuf::from(cwd));

  // 1. 创建 ExternalPluginStore（通常为空）
  let mut external_plugin_store = ExternalPluginStore::default();

  // 2. 使用 ConfigStoreBuilder 构建配置
  let config = serde_json::json!({
    "plugins": ["eslint", "typescript", "unicorn", "react", "oxc"],
    "categories": {
      "correctness": "off",
      "suspicious": "off",
      "pedantic": "off",
      "style": "off",
      "restriction": "off",
      "perf": "off",
      "nursery": "off"
    },
    "rules": {
      // eslint
      "eslint/constructor-super": [2],
      "eslint/for-direction":[2],
      "eslint/getter-return": [2, { "allowImplicit": true }],
      "eslint/no-async-promise-executor": [2],
      "eslint/no-case-declarations":[2],
      "eslint/no-class-assign": [2],
      "eslint/no-compare-neg-zero": [2],
      "eslint/no-cond-assign": [2,"except-parens"],
      "eslint/no-const-assign":[2],
      "eslint/no-constant-binary-expression":[2],
      "eslint/no-constant-condition":[2],
      "eslint/no-control-regex":[2],
      "eslint/no-delete-var":[2],
      "eslint/no-dupe-class-members":[2],
      "eslint/no-dupe-else-if":[2],
      "eslint/no-dupe-keys":[2],
      "eslint/no-duplicate-case":[2],
      "eslint/no-empty":[2,{"allowEmptyCatch":true}],
      "eslint/no-empty-character-class":[2],
      "eslint/no-empty-pattern": [2],
      "eslint/no-ex-assign":[2],
      // TODO 因为有 BUG 所以临时关闭
      "eslint/no-fallthrough":[0,{
          "allowEmptyCase":true
      }],
      "eslint/no-func-assign":[2],
      "eslint/no-global-assign":[2,{"exceptions":[]}],
      "eslint/no-import-assign":[2],
      // // 实际上只要禁用了 var 的使用，就只剩函数的场景会触发，因为只有 var、function 才会牵扯到提升问题
      "eslint/no-inner-declarations":[2,"functions"],
      "eslint/no-invalid-regexp":[2,{"allowConstructorFlags":[]}],
      "eslint/no-irregular-whitespace":[2,{}],
      "eslint/no-loss-of-precision":[2],
      "eslint/no-new-native-nonconstructor":[2],
      "eslint/no-nonoctal-decimal-escape":[2],
      "eslint/no-obj-calls":[2],
      "eslint/no-prototype-builtins":[2],
      "eslint/no-redeclare":[2,{ "builtinGlobals": false }],
      "eslint/no-regex-spaces":[2],
      "eslint/no-self-assign":[2],
      "eslint/no-setter-return":[2],
      "eslint/no-shadow-restricted-names":[2],
      "eslint/no-sparse-arrays":[2],
      "eslint/no-this-before-super":[2],
      "eslint/no-unexpected-multiline":[2],
      "eslint/no-unreachable":[2],
      "eslint/no-unsafe-finally":[2],
      "eslint/no-unsafe-negation":[2,{"enforceForOrderingRelations":true}],
      "eslint/no-unsafe-optional-chaining":[2],
      "eslint/no-unused-labels":[2],
      "eslint/no-useless-catch":[2],
      "eslint/no-useless-escape":[2],
      "eslint/use-isnan":[2,{"enforceForIndexOf": true}],
      "eslint/valid-typeof":[2],
      // jest
      // oxc
      // promise
      // react
      // typescript
      // unicorn
      "unicorn/new-for-builtins":[2],
      "unicorn/no-instanceof-array":[2],
      "unicorn/no-invalid-remove-event-listener":[2],
      "unicorn/no-thenable":[2],
      "unicorn/no-unreadable-array-destructuring":[2],
      "unicorn/require-array-join-separator":[2],
      "unicorn/require-number-to-fixed-digits-argument":[2]
    },
    "settings":{},
    "env":{},
    "globals":{},
    "overrides":[
      {
        "files": ["**/*.{ts,tsx,cts,mts}"],
        "env": {},
        "globals": {},
        "plugins": [],
        "rules":{
          "typescript/no-duplicate-enum-values":[2],
          "typescript/no-extra-non-null-assertion": [2],
          "typescript/no-misused-new": [2],
          "typescript/no-non-null-asserted-optional-chain": [2],
          "typescript/no-unsafe-function-type":[2],
          "typescript/no-unsafe-declaration-merging":[2],
          "typescript/no-wrapper-object-types":[2],
          "typescript/prefer-namespace-keyword":[2],
        }
      },
      {
        "files": ["*.{jsx,tsx}"],
        "env": {},
        "globals": {},
        "plugins": [],
        "rules":{
          "react/jsx-no-duplicate-props":[2],
          "react/jsx-no-target-blank":[2,{
            "enforceDynamicLinks": "always",
            "warnOnSpreadAttributes":false,
            "allowReferrer":false,
            "links":true,
            "forms":false
          }],
          "react/jsx-no-undef":[2],
          "react/no-children-prop":[2],
          "react/no-danger-with-children":[2],
          "react/no-direct-mutation-state":[2],
          "react/no-is-mounted":[2],
          "react/no-string-refs":[2],
          "react/jsx-no-comment-textnodes":[2],
          "react/no-render-return-value":[2],
          "react/no-find-dom-node":[2],
          "react/require-render-return": [2],
          "react/no-unescaped-entities":[2],
          "react/react-in-jsx-scope": [0],
        }
      }
    ],
    "ignorePatterns":[]
  });

  let oxlintrc = serde_json::from_value::<Oxlintrc>(config).unwrap();

  let config = ConfigStoreBuilder::from_oxlintrc(true, oxlintrc, None, &mut external_plugin_store)?
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

  let files = vec![Arc::from(
    Path::new("/Users/10015448/Git/metric-front/src/index.tsx").as_os_str(),
  )];

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
