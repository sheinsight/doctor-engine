use std::path::PathBuf;

use doctor_core::{Messages, ValidatorError};
use doctor_lint::Sfconfig;

use crate::MessagesDashboard;

pub mod register;

pub struct RenderOpts {
  pub with_dashboard: bool,
  pub max_render_count: Option<u32>,
}

impl Default for RenderOpts {
  fn default() -> Self {
    Self {
      with_dashboard: true,
      max_render_count: None,
    }
  }
}

pub struct Standards {
  cwd: PathBuf,
}

impl Standards {
  pub fn create(cwd: String) -> Standards {
    let cwd = PathBuf::from(&cwd);
    Standards { cwd }
  }

  pub fn validate_npmrc(&self) -> Result<Vec<Messages>, ValidatorError> {
    let file = self.cwd.join(".npmrc");
    let npmrc_builder = register::register_npmrc(file);
    let message = npmrc_builder.validate()?;
    Ok(message)
  }

  pub fn validate_node_version(&self) -> Result<Vec<Messages>, ValidatorError> {
    let file = self.cwd.join(".node-version");
    let node_version_builder = register::register_node_version(file);
    let message = node_version_builder.validate()?;
    Ok(message)
  }

  pub fn validate_package_json(&self) -> Result<Vec<Messages>, ValidatorError> {
    let file = self.cwd.join("package.json");
    let package_json_builder = register::register_package_json(file);
    let message = package_json_builder.validate()?;
    Ok(message)
  }

  pub fn validate_lint(&self) -> Result<Vec<Messages>, ValidatorError> {
    let file = self.cwd.join(".sfconfig").join("spec.json");
    let sfconfig = Sfconfig::parse(file)?;
    let lint_builder = register::register_lint(self.cwd.clone(), sfconfig);
    let message = lint_builder.validate()?;
    Ok(message)
  }

  pub fn validate_all(&self) -> Result<Vec<Messages>, ValidatorError> {
    let mut messages = Vec::new();
    messages.extend(self.validate_npmrc()?);
    messages.extend(self.validate_node_version()?);
    messages.extend(self.validate_package_json()?);
    messages.extend(self.validate_lint()?);
    Ok(messages)
  }

  pub fn render(&self, messages: &Vec<Messages>, opts: RenderOpts) {
    miette::set_hook(Box::new(|_| {
      Box::new(
        miette::MietteHandlerOpts::new()
          .unicode(true)
          .force_graphical(true)
          .context_lines(10)
          .tab_width(2)
          .break_words(true)
          .build(),
      )
    }))
    .unwrap();

    let messages: Vec<Messages> = messages
      .iter()
      .filter(|message| !message.is_empty())
      .cloned()
      .collect();

    if messages.is_empty() {
      println!("🚀 Ship it! Everything looks perfect.");
    }

    if let Some(max_render_count) = opts.max_render_count {
      let end = max_render_count.min(messages.len() as u32) as usize;
      let render_messages = messages.get(0..end).unwrap_or(&messages);
      for message in render_messages {
        message.render();
      }
    } else {
      for message in messages.iter() {
        message.render();
      }
    }

    if opts.with_dashboard {
      let dashboard = MessagesDashboard::new(&messages);
      dashboard.render();
    }
  }
}
