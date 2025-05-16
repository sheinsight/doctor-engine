use std::path::PathBuf;

use ignore::DirEntry;

pub const VALID_EXTENSIONS: [&str; 8] = ["js", "mjs", "cjs", "jsx", "ts", "mts", "cts", "tsx"];

#[derive(Clone)]
pub struct Extensions(pub Vec<&'static str>);

impl Default for Extensions {
  fn default() -> Self {
    Self(VALID_EXTENSIONS.to_vec())
  }
}

fn is_wanted_entry(dir_entry: &DirEntry, extensions: &Extensions) -> bool {
  let Some(file_type) = dir_entry.file_type() else {
    return false;
  };
  if file_type.is_dir() {
    return false;
  }
  let Some(file_name) = dir_entry.path().file_name() else {
    return false;
  };
  if [".min.", "-min.", "_min."]
    .iter()
    .any(|e| file_name.to_string_lossy().contains(e))
  {
    return false;
  }
  let Some(extension) = dir_entry.path().extension() else {
    return false;
  };
  let extension = extension.to_string_lossy();
  extensions.0.contains(&extension.as_ref())
}

fn hello(paths: &[PathBuf]) {
  let mut inner = ignore::WalkBuilder::new(
    paths
      .iter()
      .next()
      .expect("Expected paths parameter to Walk::new() to contain at least one path."),
  );

  for path in paths {
    let mut o = ignore::overrides::OverrideBuilder::new(path);

    o.add("!**/soapis/*").unwrap();
    o.add("!**/*/node_modules/*").unwrap();
    o.add("!**/*/dist/*").unwrap();
    o.add("!**/*/build/*").unwrap();
    o.add("!**/*/coverage/*").unwrap();
    o.add("!**/*/*.d.ts").unwrap();
    o.add("!**/*/*.min.js").unwrap();
    o.add("!**/*/*.min.css").unwrap();

    inner.overrides(o.build().unwrap());
  }

  // inner
  //   .build_parallel()
  //   .visit(&mut FnBuilder { builder: mkf });

  let walker = inner.build();

  let mut count = 0;

  for entry in walker {
    if let Ok(entry) = entry {
      if is_wanted_entry(&entry, &Extensions::default()) {
        println!("{:?}", entry);
        count += 1;
      }
    }
  }

  println!("count--->>> {}", count);
}

#[cfg(test)]
mod tests {
  use wax::Glob;

  use super::*;

  #[test]
  fn test() {
    let paths = vec![PathBuf::from("/Users/10015448/Git/drawio_ui")];
    hello(&paths);
  }

  #[test]
  fn test_walk_parallel() {
    let glob = Glob::new("**/*.js").unwrap();
    let ignore = vec![
      "**/soapis/*",
      "node_modules",
      "**/node_modules/*",
      "**/dist/*",
      "**/build/*",
      "**/coverage/*",
      "**/*.d.ts",
      "**/*.min.js",
      "**/*.min.css",
    ];
    let entries = glob
      .walk(&PathBuf::from("/Users/10015448/Git/drawio_ui"))
      .not(ignore)
      .unwrap();
    let mut count = 0;
    for entry in entries {
      // println!("{:?}", entry?);
      if let Ok(entry) = entry {
        let p = entry.path();
        if p.is_file() {
          println!("{:?}", p);
          count += 1;
        }
      }
    }
    println!("count--->>> {}", count);
  }
}
