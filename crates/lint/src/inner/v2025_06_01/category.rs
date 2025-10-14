use oxc_linter::Oxlintrc;
use serde_json::json;
use typed_builder::TypedBuilder;

use crate::{
  Globals, LintMode,
  common::environments::EnvironmentFlags,
  config::{ReactConfig, TypescriptConfig},
  ext::CategoryGetter,
};

#[derive(Debug, Clone, TypedBuilder)]
pub struct Category20250601Inner {
  #[builder(default = Some(ReactConfig::default()), setter(strip_option))]
  pub react: Option<ReactConfig>,
  #[builder(default = Some(TypescriptConfig::default()), setter(strip_option))]
  pub typescript: Option<TypescriptConfig>,
  #[builder(default = LintMode::Development)]
  pub mode: LintMode,
  #[builder(default = EnvironmentFlags::default())]
  pub envs: EnvironmentFlags,
  #[builder(default = Globals::default())]
  pub globals: Globals,
}

impl Default for Category20250601Inner {
  fn default() -> Self {
    Self::builder().build()
  }
}

impl Into<Oxlintrc> for Category20250601Inner {
  fn into(self) -> Oxlintrc {
    self.get_config()
  }
}

impl CategoryGetter for Category20250601Inner {
  fn get_config(&self) -> Oxlintrc {
    let config = json!({
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
            "react/react-in-jsx-scope": [match &self.react {
              Some(react) if react.runtime == crate::ReactRuntime::Automatic => 0,
              _ => 2
            }],
          }
        }
      ],
      "ignorePatterns":[]
    });

    let config = serde_json::from_value::<Oxlintrc>(config).unwrap();

    config
  }

  // fn get_def(&self) -> Map<String, Value> {
  //   let mut merged = Map::new();

  //   let eslint = EslintRuleGetter::default();
  //   let oxc = OxcRuleGetter::default();
  //   let promise = PromiseRuleGetter::default();
  //   let unicorn = UnicornRuleGetter::default();

  //   merged.extend(eslint.get_def());
  //   merged.extend(oxc.get_def());
  //   merged.extend(promise.get_def());
  //   merged.extend(unicorn.get_def());

  //   merged
  // }

  // fn get_ts_override(&self) -> Value {
  //   if let Some(typescript) = &self.typescript {
  //     let typescript = TypescriptRuleGetter::default().with_config(typescript.clone());
  //     json!({
  //         "files": ["*.{ts,tsx,cts,mts}"],
  //         "plugins": LintPlugins::TYPESCRIPT,
  //         "rules": typescript.get_def()
  //     })
  //   } else {
  //     json!({
  //       "files": ["*.{ts,tsx,cts,mts}"],
  //       "plugins": LintPlugins::TYPESCRIPT,
  //     })
  //   }
  // }

  // fn get_react_override(&self) -> Value {
  //   if let Some(react) = &self.react {
  //     let react = ReactRuleGetter::default().with_runtime(react.runtime.clone());
  //     json!({
  //         "files": ["*.{jsx,tsx}"],
  //         "plugins": LintPlugins::REACT,
  //         "rules": react.get_def()
  //     })
  //   } else {
  //     json!({
  //       "files": ["*.{jsx,tsx}"],
  //       "plugins": LintPlugins::REACT,
  //     })
  //   }
  // }

  // fn get_jest_override(&self) -> Value {
  //   json!({
  //       "files": [
  //           "*.{test,spec}.{js,jsx,ts,tsx}",
  //           "**/{test,tests,spec,specs}/**",
  //       ],
  //       "plugins": LintPlugins::JEST,
  //       "env": EnvironmentFlags::Jest | EnvironmentFlags::Es2024,
  //       "rules": JestRuleGetter::default().get_def()
  //   })
  // }

  // fn get_def_plugins(&self) -> oxc_linter::LintPlugins {
  //   let mut builtin = LintPlugins::ESLINT
  //     | LintPlugins::UNICORN
  //     | LintPlugins::IMPORT
  //     | LintPlugins::PROMISE
  //     | LintPlugins::OXC
  //     | LintPlugins::JEST;

  //   if self.typescript.is_some() {
  //     builtin |= LintPlugins::TYPESCRIPT
  //   }

  //   if self.react.is_some() {
  //     builtin |= LintPlugins::REACT | LintPlugins::REACT_PERF
  //   }

  //   builtin
  // }
}
