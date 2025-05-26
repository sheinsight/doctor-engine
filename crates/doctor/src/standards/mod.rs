use std::path::{Path, PathBuf};

use doctor_core::{Messages, ValidatorError};
use doctor_lint::Sfconfig;
use typed_builder::TypedBuilder;

use crate::{MessagesDashboard, ValidatorScheduler};

mod register_lint;
mod register_node_version;
mod register_npmrc;
mod register_package_json;

#[derive(Debug, Clone, TypedBuilder)]
pub struct VerifyStandardsOptions {
  #[builder(default = None, setter(strip_option))]
  max_render_count: Option<usize>,
  #[builder(default = true)]
  with_dashboard: bool,
}

pub fn verify_standards<T: AsRef<Path>>(
  cwd: T,
  options: VerifyStandardsOptions,
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

  let sfconfig = Sfconfig::parse(cwd.join(".sfconfig").join("spec.json"))?;

  scheduler.push(register_npmrc::register_npmrc(cwd.join(".npmrc")));

  scheduler.push(register_node_version::register_node_version(
    cwd.join(".node-version"),
  ));

  scheduler.push(register_package_json::register_package_json(
    cwd.join("package.json"),
  ));

  scheduler.push(register_lint::register_lint(cwd, sfconfig));

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

  if messages.is_empty() {
    println!("🚀 Ship it! Everything looks perfect.");
  }

  Ok(messages)
}
