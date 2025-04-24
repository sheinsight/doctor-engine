use std::sync::Once;

use env_logger::{Builder, Env};
use napi_derive::napi;

use strum_macros::Display;
static INIT: Once = Once::new();

#[derive(Debug, Clone, Copy, Display)]
#[strum(serialize_all = "lowercase")]
#[napi]
pub enum LogLevel {
  Error,
  Warn,
  Info,
  Debug,
  Trace,
}

#[napi]
pub fn initialize_logger(level: Option<LogLevel>) {
  INIT.call_once(|| {
    let level = level.unwrap_or(LogLevel::Warn);

    let default_filter = [
      "doctor",
      "doctor_ext",
      "doctor_lint",
      "doctor_node",
      "doctor_npmrc",
      "doctor_package_json",
      "doctor_walk_parallel",
    ]
    .into_iter()
    .map(|item| format!("{item}={level}"))
    .collect::<Vec<String>>()
    .join(",");

    let default_filter = format!("error,{default_filter}",);

    Builder::from_env(
      Env::default().default_filter_or(&default_filter), // <--- 设置默认表达式
    )
    .format_timestamp(None)
    // .format_target(false) // from_env 通常能处理 target
    .init();
  });
}
