use serde_json::{Map, Value, json};

use crate::rule_getter::RuleGetter;

pub struct UnicornRuleGetter;

impl Default for UnicornRuleGetter {
  fn default() -> Self {
    Self {}
  }
}

impl RuleGetter for UnicornRuleGetter {
  fn get_def(&self) -> Map<String, Value> {
    json!({
        "unicorn/new-for-builtins":[2],
        "unicorn/no-instanceof-array":[2],
        "unicorn/no-invalid-remove-event-listener":[2],
        "unicorn/no-thenable":[2],
        "unicorn/no-unreadable-array-destructuring":[2],
        "unicorn/require-array-join-separator":[2],
        "unicorn/require-number-to-fixed-digits-argument":[2]
    })
    .as_object()
    .map_or(Map::new(), |map| map.to_owned())
  }
}
