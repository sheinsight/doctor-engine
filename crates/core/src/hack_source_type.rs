use oxc::span::SourceType;

pub fn hack_source_type_from_path(path: &std::path::Path) -> SourceType {
  match path.extension().and_then(|ext| ext.to_str()) {
    Some("ts") => SourceType::ts(),
    Some("tsx") => SourceType::tsx(),
    Some("jsx") => SourceType::jsx(),
    Some("cjs") => SourceType::cjs(),
    _ => SourceType::jsx(),
  }
}
