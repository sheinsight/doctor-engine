use std::ops::{Deref, DerefMut};

use oxc_linter::Oxlintrc;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::{category::Category, inner::Category20250601Inner};

/**
 * 👍 1. 必须知道模块系统是什么 。 🤔 testing 需不需要独立配置 ？？？
 * 1.1 强制统一配置 esm 。
 *
 * 👍 2. 必须知道检查目标是 系统 还是 包，检测的机制不一样，例如对于 package.json 的检测。
 * 2.1 直接传给我
 *
 * 👍 3. 必须知道目标应用，例如是 vue 还是 react 还是 其他。
 * 3.1 部分自动感知，探知 deps 信息
 * 3.2 🤔 放开 global ？
 *
 * 👍 4. 对于测试系统的整合，似乎也要知道测试框架是什么。
 * 4.1 强制统一测试框架 ， 所以可以不开放 env 配置。
 *
 * 🚨 5. 要知道模式，例如是 development 还是 production。
 *
 *
 * 👍 6. 要知道是否启用 ts、默认启用
 *
 * 👍 7. 要知道 define。
 * 👍 8. 要知道 category，主要是用来区分版本信息的
 */

#[derive(Debug, Clone, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum GlobalValue {
  Writable,
  Readonly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Globals(pub FxHashMap<String, GlobalValue>);

impl Default for Globals {
  fn default() -> Self {
    Self(FxHashMap::default())
  }
}

impl Deref for Globals {
  type Target = FxHashMap<String, GlobalValue>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Globals {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

#[derive(Debug, Clone)]
pub struct OxlintrcBuilder {
  category: Category,
}

impl Default for OxlintrcBuilder {
  fn default() -> Self {
    Self {
      category: Category::V20250601Inner(Category20250601Inner::builder().build()),
    }
  }
}

impl OxlintrcBuilder {
  pub fn with_category(mut self, category: Category) -> Self {
    self.category = category;
    self
  }

  pub fn build(&self) -> Oxlintrc {
    let category = match &self.category {
      Category::V20250601Inner(category) => category.to_owned(),
    };

    category.into()
  }
}
