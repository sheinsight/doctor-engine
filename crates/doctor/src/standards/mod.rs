use std::path::PathBuf;

use doctor_core::{Messages, ValidatorError};
use doctor_lint::Sfconfig;

use crate::MessagesDashboard;

pub mod register;

// #[derive(Debug, Clone, TypedBuilder)]
// pub struct VerifyStandardsOptions {
//   #[builder(default = None, setter(strip_option))]
//   max_render_count: Option<usize>,
//   #[builder(default = true)]
//   with_dashboard: bool,
// }

// pub fn verify_standards<T: AsRef<Path>>(
//   cwd: T,
//   options: VerifyStandardsOptions,
// ) -> Result<Vec<Messages>, ValidatorError> {
//   miette::set_hook(Box::new(|_| {
//     Box::new(
//       miette::MietteHandlerOpts::new()
//         .terminal_links(true)
//         .unicode(true)
//         .force_graphical(true)
//         .context_lines(3)
//         .tab_width(4)
//         .break_words(true)
//         .build(),
//     )
//   }))?;

//   let mut scheduler = ValidatorScheduler::default();

//   let cwd = PathBuf::from(cwd.as_ref());

//   let sfconfig = Sfconfig::parse(cwd.join(".sfconfig").join("spec.json"))?;

//   scheduler.push(register::register_npmrc(cwd.join(".npmrc")));

//   scheduler.push(register::register_node_version(cwd.join(".node-version")));

//   scheduler.push(register::register_package_json(cwd.join("package.json")));

//   scheduler.push(register::register_lint(cwd, sfconfig));

//   let messages = scheduler
//     .validator()
//     .map_err(|e| ValidatorError::Unknown(Box::new(e)))?
//     .into_iter()
//     .filter(|message| !message.diagnostics.is_empty())
//     .collect::<Vec<_>>();

//   if let Some(max_render_count) = options.max_render_count {
//     let end = max_render_count.min(messages.len());
//     let render_messages = messages.get(0..end).unwrap_or(&messages);

//     for message in render_messages {
//       message.render();
//     }
//   } else {
//     for message in messages.iter() {
//       message.render();
//     }
//   }

//   if options.with_dashboard {
//     let dashboard = MessagesDashboard::new(&messages);
//     dashboard.render();
//   }

//   if messages.is_empty() {
//     println!("🚀 Ship it! Everything looks perfect.");
//   }

//   Ok(messages)
// }

pub struct StandardsOpts {
  pub quiet: bool,
  pub with_dashboard: bool,
  pub max_render_count: Option<u32>,
}

impl Default for StandardsOpts {
  fn default() -> Self {
    Self {
      quiet: false,
      with_dashboard: true,
      max_render_count: None,
    }
  }
}

pub struct Standards {
  cwd: PathBuf,
  opts: StandardsOpts,
}

impl Standards {
  pub fn create(cwd: String, options: StandardsOpts) -> Standards {
    let cwd = PathBuf::from(&cwd);
    Standards { cwd, opts: options }
  }

  pub async fn validate_npmrc(&self) -> Result<Vec<Messages>, ValidatorError> {
    let file = self.cwd.join(".npmrc");
    let npmrc_builder = register::register_npmrc(file);
    let messages = npmrc_builder.validate()?;
    Ok(messages.into_iter().map(|m| m.into()).collect())
  }

  pub async fn validate_node_version(&self) -> Result<Vec<Messages>, ValidatorError> {
    let file = self.cwd.join(".node-version");
    let node_version_builder = register::register_node_version(file);
    let messages = node_version_builder.validate()?;
    Ok(messages)
  }

  pub async fn validate_package_json(&self) -> Result<Vec<Messages>, ValidatorError> {
    let file = self.cwd.join("package.json");
    let package_json_builder = register::register_package_json(file);
    let messages = package_json_builder.validate()?;
    Ok(messages.into_iter().map(|m| m.into()).collect())
  }

  pub async fn validate_lint(&self) -> Result<Vec<Messages>, ValidatorError> {
    let file = self.cwd.join(".sfconfig").join("spec.json");
    let sfconfig = Sfconfig::parse(file)?;
    let lint_builder = register::register_lint(self.cwd.clone(), sfconfig);
    let messages = lint_builder.validate()?;
    Ok(messages.into_iter().map(|m| m.into()).collect())
  }

  pub async fn validate_all(&self) -> Result<Vec<Messages>, ValidatorError> {
    let mut messages = Vec::new();
    messages.extend(self.validate_npmrc().await?);
    messages.extend(self.validate_node_version().await?);
    messages.extend(self.validate_package_json().await?);
    messages.extend(self.validate_lint().await?);

    let messages = messages
      .into_iter()
      .filter(|message| !message.is_empty())
      .collect::<Vec<_>>();

    if !self.opts.quiet {
      self.render(&messages);
    }

    Ok(messages)
  }

  fn render(&self, messages: &[Messages]) {
    if messages.is_empty() {
      println!("🚀 Ship it! Everything looks perfect.");
    }

    if let Some(max_render_count) = self.opts.max_render_count {
      let end = max_render_count.min(messages.len() as u32) as usize;
      let render_messages = messages.get(0..end).unwrap_or(messages);
      for message in render_messages {
        message.render();
      }
    } else {
      for message in messages {
        message.render();
      }
    }

    if self.opts.with_dashboard {
      let dashboard = MessagesDashboard::new(messages);
      dashboard.render();
    }
  }
}
