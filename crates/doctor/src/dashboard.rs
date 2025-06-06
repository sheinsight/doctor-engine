use std::collections::HashMap;

use doctor_core::Messages;
use tabled::{Table, Tabled};

#[derive(Tabled)]
pub struct Row {
  #[tabled(rename = "name")]
  pub name: String,
  #[tabled(rename = "count")]
  pub count: usize,
}

pub struct MessagesDashboard<'a>(&'a [Messages]);

impl<'a> MessagesDashboard<'a> {
  pub fn new(messages: &'a [Messages]) -> Self {
    Self(messages)
  }

  pub fn render(&self) -> Vec<String> {
    let mut reports = Vec::new();

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
      let table_str = format!("{}", table);
      reports.push(table_str);
      println!("{}", table);
    }

    return reports;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_render_empty_when_messages_is_empty() {
    let dashboard = MessagesDashboard::new(&[]);
    dashboard.render();
  }
}
