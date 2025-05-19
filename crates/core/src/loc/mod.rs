mod location;
mod position;

pub use location::*;
pub use position::*;

pub fn get_source_location(source_code: String, offset: usize, len: usize) -> Location {
  Location::new(source_code, offset, len)
}
