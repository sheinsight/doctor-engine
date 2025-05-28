use serde_json::{Map, Value, json};

use crate::ext::RuleGetter;

pub struct EslintRuleGetter;

impl Default for EslintRuleGetter {
  fn default() -> Self {
    Self {}
  }
}

impl RuleGetter for EslintRuleGetter {
  fn get_def(&self) -> Map<String, Value> {
    json!({
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
        "eslint/no-fallthrough":[2,{
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
        "eslint/valid-typeof":[2]
    })
    .as_object()
    .map_or(Map::new(), |map| map.to_owned())
  }
}
