use oxc_linter::Oxlintrc;

pub trait CategoryGetter {
  fn get_config(&self) -> Oxlintrc;
}
