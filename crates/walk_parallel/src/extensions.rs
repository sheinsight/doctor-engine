pub const VALID_EXTENSIONS: [&str; 8] = ["js", "mjs", "cjs", "jsx", "ts", "mts", "cts", "tsx"];

#[derive(Clone, Debug)]
pub struct Extensions(pub Vec<&'static str>);

impl Default for Extensions {
  fn default() -> Self {
    Self(VALID_EXTENSIONS.to_vec())
  }
}
