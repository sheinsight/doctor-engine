use oxc::span::SourceType;

pub fn hack_source_type_from_path(path: &std::path::Path) -> SourceType {
  match path.extension().and_then(|ext| ext.to_str()) {
    Some("ts") => SourceType::ts(),
    Some("mts") => SourceType::ts(),
    Some("cts") => SourceType::ts(),
    Some("tsx") => SourceType::tsx(),
    // Some("js") => SourceType::mjs(),
    Some("mjs") => SourceType::mjs(),
    Some("cjs") => SourceType::cjs(),
    Some("jsx") => SourceType::jsx(),
    _ => SourceType::jsx(),
  }
}
