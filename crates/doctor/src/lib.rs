mod scheduler;

use std::path::{Path, PathBuf};

use base64::{Engine, engine::general_purpose::STANDARD};
use doctor_ext::{Messages, ValidatorError};
use doctor_lint::{
  Category, EnvironmentFlags, LintMode, LintValidator, OxlintrcBuilder, Sfconfig,
  inner::Category20250601Inner,
};
use doctor_node::validator::NodeVersionValidator;
use doctor_npmrc::validator::NpmrcValidator;
use doctor_package_json::validator::{
  PackageJsonValidator, ValidateName, ValidatePackageManager, ValidatePrivate,
};

pub use scheduler::*;
use typed_builder::TypedBuilder;

const ENCODED: [&str; 36] = [
  "a", "H", "R", "0", "c", "H", "M", "6", "L", "y", "9", "u", "c", "G", "1", "q", "c", "y", "5",
  "z", "a", "G", "V", "p", "b", "m", "N", "v", "c", "n", "A", "u", "Y", "2", "4", "=",
];

fn decode_to_str(encoded: &str) -> String {
  let decoded = STANDARD.decode(encoded).unwrap();
  String::from_utf8(decoded).unwrap()
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct DoctorOptions {
  #[builder(default = None, setter(strip_option))]
  max_render_count: Option<usize>,
  #[builder(default = true)]
  with_dashboard: bool,
}

pub fn doctor<T: AsRef<Path>>(
  cwd: T,
  options: DoctorOptions,
) -> Result<Vec<Messages>, ValidatorError> {
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
  }))?;

  let mut scheduler = ValidatorScheduler::default();

  let cwd = PathBuf::from(cwd.as_ref());

  let text = decode_to_str(ENCODED.join("").as_str());

  scheduler.push(Box::new(
    NpmrcValidator::builder()
      .config_path(cwd.join(".npmrc"))
      .with_registry_url(vec![text])
      .build(),
  ));

  scheduler.push(Box::new(
    NodeVersionValidator::builder()
      .config_path(cwd.join(".node-version"))
      .with_valid_range(vec!["^16.13.0", "^18.12.0", "^20.9.0", "^22.11.0"])
      .build(),
  ));

  scheduler.push(Box::new(
    PackageJsonValidator::builder()
      .config_path(cwd.join("package.json"))
      .with_validate_name(ValidateName::Exist)
      .with_validate_private(ValidatePrivate::True)
      .with_validate_package_manager(ValidatePackageManager::Exist)
      .build(),
  ));

  let category = Category::V20250601Inner(Category20250601Inner::default());

  let sfconfig = Sfconfig::parse(cwd.join(".sfconfig").join("spec.json"))?;

  let rc = OxlintrcBuilder::default()
    .with_category(category)
    .with_globals(sfconfig.globals)
    .with_mode(LintMode::Production)
    .with_envs(EnvironmentFlags::default())
    .build()
    .unwrap();

  scheduler.push(Box::new(
    LintValidator::builder()
      .cwd(cwd)
      .ignore(sfconfig.ignore)
      .with_show_report(false)
      .oxlintrc(rc)
      .build(),
  ));

  let messages = scheduler
    .validator()
    .map_err(|e| ValidatorError::Unknown(Box::new(e)))?
    .into_iter()
    .filter(|message| !message.diagnostics.is_empty())
    .collect::<Vec<_>>();

  if let Some(max_render_count) = options.max_render_count {
    let end = max_render_count.min(messages.len());
    let render_messages = messages.get(0..end).unwrap_or(&messages);

    for message in render_messages {
      message.render();
    }
  } else {
    for message in messages.iter() {
      message.render();
    }
  }

  if options.with_dashboard {
    let dashboard = MessagesDashboard::new(&messages);
    dashboard.render();
  }

  Ok(messages)
}
