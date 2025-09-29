use oxc_linter::{BuiltinLintPlugins, LintPlugins};
use rustc_hash::FxHashSet;
use serde_json::{Map, Value, json};
use typed_builder::TypedBuilder;

use crate::{
  common::environments::EnvironmentFlags,
  config::{ReactConfig, TypescriptConfig},
  ext::{CategoryGetter, RuleGetter},
  inner::v2025_06_01::{
    jest::JestRuleGetter, react::ReactRuleGetter, typescript::TypescriptRuleGetter,
  },
};

use super::{
  eslint::EslintRuleGetter, oxc::OxcRuleGetter, promise::PromiseRuleGetter,
  unicorn::UnicornRuleGetter,
};

#[derive(Debug, Clone, TypedBuilder)]
pub struct Category20250601Inner {
  #[builder(default = Some(ReactConfig::default()), setter(strip_option))]
  pub react: Option<ReactConfig>,
  #[builder(default = Some(TypescriptConfig::default()), setter(strip_option))]
  pub typescript: Option<TypescriptConfig>,
}

impl Default for Category20250601Inner {
  fn default() -> Self {
    Self::builder().build()
  }
}

impl CategoryGetter for Category20250601Inner {
  fn get_def(&self) -> Map<String, Value> {
    let mut merged = Map::new();

    let eslint = EslintRuleGetter::default();
    let oxc = OxcRuleGetter::default();
    let promise = PromiseRuleGetter::default();
    let unicorn = UnicornRuleGetter::default();

    merged.extend(eslint.get_def());
    merged.extend(oxc.get_def());
    merged.extend(promise.get_def());
    merged.extend(unicorn.get_def());

    merged
  }

  fn get_ts_override(&self) -> Value {
    let plugins = LintPlugins::new(BuiltinLintPlugins::TYPESCRIPT, FxHashSet::default());
    if let Some(typescript) = &self.typescript {
      let typescript = TypescriptRuleGetter::default().with_config(typescript.clone());
      json!({
          "files": ["*.{ts,tsx,cts,mts}"],
          "plugins": plugins,
          "rules": typescript.get_def()
      })
    } else {
      json!({
        "files": ["*.{ts,tsx,cts,mts}"],
        "plugins": plugins,
      })
    }
  }

  fn get_react_override(&self) -> Value {
    let plugins = LintPlugins::new(BuiltinLintPlugins::REACT, FxHashSet::default());
    if let Some(react) = &self.react {
      let react = ReactRuleGetter::default().with_runtime(react.runtime.clone());
      json!({
          "files": ["*.{jsx,tsx}"],
          "plugins": plugins,
          "rules": react.get_def()
      })
    } else {
      json!({
        "files": ["*.{jsx,tsx}"],
        "plugins": plugins,
      })
    }
  }

  fn get_jest_override(&self) -> Value {
    let plugins = LintPlugins::new(BuiltinLintPlugins::JEST, FxHashSet::default());
    json!({
        "files": [
            "*.{test,spec}.{js,jsx,ts,tsx}",
            "**/{test,tests,spec,specs}/**",
        ],
        "plugins": plugins,
        "env": EnvironmentFlags::Jest | EnvironmentFlags::Es2024,
        "rules": JestRuleGetter::default().get_def()
    })
  }

  fn get_def_plugins(&self) -> oxc_linter::LintPlugins {
    let mut builtin = BuiltinLintPlugins::ESLINT
      | BuiltinLintPlugins::UNICORN
      | BuiltinLintPlugins::IMPORT
      | BuiltinLintPlugins::PROMISE
      | BuiltinLintPlugins::OXC
      | BuiltinLintPlugins::JEST;

    if self.typescript.is_some() {
      builtin |= BuiltinLintPlugins::TYPESCRIPT
    }

    if self.react.is_some() {
      builtin |= BuiltinLintPlugins::REACT | BuiltinLintPlugins::REACT_PERF
    }

    LintPlugins::new(builtin, FxHashSet::default())
  }
}
