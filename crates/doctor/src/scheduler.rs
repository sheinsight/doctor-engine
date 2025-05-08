use std::{
  collections::HashMap,
  ops::{Deref, DerefMut},
};

use doctor_ext::{Messages, Validator, ValidatorError};
use tabled::{Table, Tabled};

#[derive(Tabled)]
pub struct Row {
  #[tabled(rename = "name")]
  pub name: String,
  #[tabled(rename = "count")]
  pub count: usize,
}

pub struct MessagesDashboard<'a>(&'a Vec<Messages>);

impl<'a> MessagesDashboard<'a> {
  pub fn new(messages: &'a Vec<Messages>) -> Self {
    Self(messages)
  }

  pub fn render(&self) {
    let mut count_map = HashMap::new();
    for msg in self.0 {
      if msg.has_error() {
        for item in &msg.diagnostics {
          *count_map
            .entry(item.code.clone().unwrap_or("unknown".to_string()))
            .or_insert(0) += 1;
        }
      }
    }
    let mut ts = vec![];

    for (key, value) in count_map {
      ts.push(Row {
        name: key,
        count: value,
      });
    }

    if !ts.is_empty() {
      let table = Table::new(ts);
      println!("{}", table);
    }
  }
}

pub struct ValidatorScheduler(Vec<Box<dyn Validator>>);

impl Default for ValidatorScheduler {
  fn default() -> Self {
    Self(vec![])
  }
}

impl Deref for ValidatorScheduler {
  type Target = Vec<Box<dyn Validator>>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for ValidatorScheduler {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl ValidatorScheduler {
  pub fn validator(&self) -> Result<Vec<Messages>, ValidatorError> {
    let mut messages = vec![];

    for validator in self.iter() {
      let result = validator.validate()?;
      messages.extend(result.into_iter());
    }

    Ok(messages)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_render_empty_when_messages_is_empty() {
    let scheduler = ValidatorScheduler::default();
    let messages = scheduler.validator().unwrap();
    let dashboard = MessagesDashboard::new(&messages);
    dashboard.render();
  }
}
