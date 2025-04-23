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
        if is_minified_by_characteristics(path) {
          return false;
        }

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
    let mut buffer = [0; 188 * 3];
    if file.read_exact(&mut buffer).is_ok() {
      // 检查多个包的同步字节
      return buffer[0] == 0x47 && buffer[188] == 0x47 && buffer[376] == 0x47;
    }
  }
  false
}

fn is_minified_by_characteristics(path: &Path) -> bool {
  let Some(file_name) = path.file_name() else {
    return false;
  };

  if [".min.", "-min.", "_min."]
    .iter()
    .any(|e| file_name.to_string_lossy().contains(e))
  {
    return false;
  }

  if let Ok(content) = fs::read_to_string(path) {
    if content.is_empty() {
      return false;
    }

    // 1. 检查平均行长度
    let lines: Vec<&str> = content.lines().collect();
    let avg_line_length =
      lines.iter().map(|line| line.len() as f64).sum::<f64>() / lines.len() as f64;

    // 2. 检查分号后面是否紧跟其他字符（压缩代码常见特征）
    let semicolon_packed = content.contains(";var")
      || content.contains(";function")
      || content.contains(";const")
      || content.contains(";let");

    // 3. 检查是否存在很长的行（超过1000字符）
    let has_long_lines = lines.iter().any(|line| line.len() > 1000);

    // 4. 检查空白字符比例
    let whitespace_ratio =
      content.chars().filter(|c| c.is_whitespace()).count() as f64 / content.len() as f64;

    // 5. 检查是否包含 sourceMappingURL（压缩文件常有）
    let has_source_map = content.contains("sourceMappingURL");

    // 组合多个特征进行判断
    avg_line_length > 500.0
      || (semicolon_packed && has_long_lines)
      || whitespace_ratio < 0.1
      || has_source_map
  } else {
    false
  }
}
