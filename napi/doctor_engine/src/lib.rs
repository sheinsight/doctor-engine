mod lint;
mod log;

use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

use base64::{Engine, engine::general_purpose::STANDARD};
use doctor::ext::*;
use doctor::lint::inner::Category20250601Inner;
use doctor::lint::{
  Category, EnvironmentFlags, GlobalValue, Globals, LintMode, LinterRunner, OxlintrcBuilder,
};
use doctor::validator::{
  NodeVersionValidator, NpmrcValidator, PackageJsonValidator, ValidatePrivate,
};

use doctor::walk_parallel::WalkIgnore;
pub use lint::*;
pub use log::*;
use napi_derive::napi;

// edition 2021
#[napi(object)]
pub struct DoctorOptions {
  pub verbose: bool,
  pub globals: Option<HashMap<String, String>>,
  pub ignore: Option<Vec<String>>,
}

fn decode_to_str(encoded: &str) -> String {
  let decoded = STANDARD.decode(encoded).unwrap();
  String::from_utf8(decoded).unwrap()
}

#[napi]
pub fn doctor(cwd: String, options: DoctorOptions) {
  miette::set_hook(Box::new(|_| {
    Box::new(
      miette::MietteHandlerOpts::new()
        .terminal_links(true)
        .unicode(true)
        .force_graphical(true)
        .context_lines(3)
        .tab_width(4)
        .break_words(true)
        .build(),
    )
  }))
  .unwrap();

  let cwd = PathBuf::from(cwd);

  let encoded = vec![
    "a", "H", "R", "0", "c", "H", "M", "6", "L", "y", "9", "u", "c", "G", "1", "q", "c", "y", "5",
    "z", "a", "G", "V", "p", "b", "m", "N", "v", "c", "n", "A", "u", "Y", "2", "4", "=",
  ];

  let text = decode_to_str(encoded.join("").as_str());

  let npmrc_validator = NpmrcValidator::builder()
    .config_path(cwd.join(".npmrc"))
    .with_registry_url(vec![text.as_str()])
    .build();

  let result = npmrc_validator.validate();

  if let Err(e) = result {
    eprintln!("{:?}", miette::Report::new(e));
  }

  let node_version_validator = NodeVersionValidator::builder()
    .config_path(cwd.join(".node-version"))
    .build();

  let result = node_version_validator.validate();

  if let Err(e) = result {
    eprintln!("{:?}", miette::Report::new(e));
  }

  let package_json_validator = PackageJsonValidator::builder()
    .config_path(cwd.join("package.json"))
    .with_validate_private(ValidatePrivate::True)
    .build();

  let result = package_json_validator.validate();

  if let Err(e) = result {
    eprintln!("{:?}", miette::Report::new(e));
  }

  let category = Category::V20250601Inner(Category20250601Inner::default());

  let mut globals = Globals::default();

  if let Some(opt_globals) = options.globals {
    for (key, value) in opt_globals {
      let global_value = GlobalValue::from_str(value.as_str()).unwrap();
      globals.insert(key.to_string(), global_value);
    }
  }

  let rc = OxlintrcBuilder::default()
    .with_category(category)
    .with_globals(globals)
    .with_mode(LintMode::Production)
    .with_envs(EnvironmentFlags::default())
    .build()
    .unwrap();

  std::fs::write("oxlintrc.json", serde_json::to_string_pretty(&rc).unwrap()).unwrap();

  eprintln!("3--->>>");

  let mut ignore = WalkIgnore(vec!["**/node_modules/**".to_string()]);

  if let Some(opt_ignore) = options.ignore {
    ignore.extend(opt_ignore.into_iter());
  }

  let linter_runner = LinterRunner::builder()
    .cwd(cwd)
    .ignore(ignore)
    .with_show_report(false)
    .oxlintrc(rc)
    .build();

  let res = linter_runner.run();

  if let Err(e) = res {
    println!("{:?}", e);
  }
}
