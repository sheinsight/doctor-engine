use std::path::PathBuf;

use doctor_core::{Messages, ValidatorError};
use doctor_lint::Sfconfig;

use crate::MessagesDashboard;

mod register;

pub struct SpecificationsRenderOpts {
  pub with_dashboard: bool,
  pub max_render_count: Option<u32>,
}

impl Default for SpecificationsRenderOpts {
  fn default() -> Self {
    Self {
      with_dashboard: true,
      max_render_count: None,
    }
  }
}

pub struct Specifications {
  cwd: PathBuf,
}

impl Specifications {
  pub fn create(cwd: String) -> Specifications {
    let cwd = PathBuf::from(&cwd);
    Specifications { cwd }
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

  pub fn validate_syntax(&self) -> Result<Vec<Messages>, ValidatorError> {
    let file = self.cwd.join(".sfconfig").join("spec.json");
    let sfconfig = Sfconfig::parse(file)?;
    let syntax_builder = register::register_syntax(self.cwd.clone(), sfconfig);
    let message = syntax_builder.validate()?;
    Ok(message)
  }

  pub fn validate_all(&self) -> Result<Vec<Messages>, ValidatorError> {
    let mut messages = Vec::new();
    messages.extend(self.validate_npmrc()?);
    messages.extend(self.validate_node_version()?);
    messages.extend(self.validate_package_json()?);
    messages.extend(self.validate_lint()?);
    messages.extend(self.validate_syntax()?);
    Ok(messages)
  }

  pub fn render(&self, messages: &Vec<Messages>, opts: SpecificationsRenderOpts) -> Vec<String> {
    let _ = miette::set_hook(Box::new(|_| {
      Box::new(
        miette::MietteHandlerOpts::new()
          .unicode(true)
          .force_graphical(true)
          .context_lines(5)
          .tab_width(2)
          .break_words(true)
          .build(),
      )
    }));

    let messages: Vec<Messages> = messages
      .iter()
      .filter(|message| !message.is_empty())
      .cloned()
      .collect();

    let mut reports = Vec::new();

    if messages.is_empty() {
      let success_str = "🚀 Ship it! Everything looks perfect.";
      reports.push(success_str.to_string());
      println!("{}", success_str);
    }

    if let Some(max_render_count) = opts.max_render_count {
      let end = max_render_count.min(messages.len() as u32) as usize;
      let render_messages = messages.get(0..end).unwrap_or(&messages);
      for message in render_messages {
        reports.extend(message.render());
      }
    } else {
      for message in messages.iter() {
        reports.extend(message.render());
      }
    }

    if opts.with_dashboard {
      let dashboard = MessagesDashboard::new(&messages);
      reports.extend(dashboard.render());
    }

    return reports;
  }
}
