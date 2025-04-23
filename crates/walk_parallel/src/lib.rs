use ::ignore::DirEntry;
use extensions::Extensions;
use rayon::prelude::*;
use std::{
  fs,
  io::Read,
  path::{Path, PathBuf},
};
use typed_builder::TypedBuilder;

// mod demo;
mod error;
mod extensions;
mod walk_ignore;

pub use error::WalkError;
pub use walk_ignore::WalkIgnore;

#[derive(Debug, Clone, TypedBuilder)]
pub struct WalkParallelJs {
  cwd: PathBuf,

  #[builder(default = WalkIgnore::default())]
  pub ignore: WalkIgnore,
}

impl WalkParallelJs {
  fn is_wanted_entry(&self, dir_entry: &DirEntry, extensions: &Extensions) -> bool {
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

  pub fn walk<F, R>(&self, f: F) -> Result<Vec<Result<R, WalkError>>, WalkError>
  where
    F: Fn(PathBuf) -> Result<R, WalkError> + Send + Sync,
    R: Send + Sync,
  {
    let mut inner = ignore::WalkBuilder::new(&self.cwd);

    let mut r#override = ignore::overrides::OverrideBuilder::new(&self.cwd);

    for pattern in &self.ignore.0 {
      r#override.add(format!("!{pattern}").as_str()).unwrap();
    }

    inner.overrides(r#override.build().unwrap());

    let walker = inner.build();

    let ext = Extensions::default();
    let res = walker
      .par_bridge()
      .filter_map(Result::ok)
      .filter(|entry| self.is_wanted_entry(entry, &ext))
      .map(|entry| entry.path().to_owned())
      .filter(|path| path.is_file())
      .filter(|path| {
        if is_ts_video(path) {
          return false;
        }

        // 大于 1mb 的过滤
        if let Ok(metadata) = std::fs::metadata(path) {
          return metadata.len() < 1024 * 1024;
        } else {
          return false;
        }
      })
      .map(f)
      .collect::<Vec<Result<R, WalkError>>>();

    Ok(res)
  }
}

pub fn is_ts_video(path: &Path) -> bool {
  if let Ok(mut file) = fs::File::open(path) {
    let mut buffer = [0; 4];
    if file.read_exact(&mut buffer).is_ok() {
      // 检查多个包的同步字节
      return buffer[0] == 0x47 && buffer[188] == 0x47 && buffer[376] == 0x47;
    }
  }
  false
}
