pub fn hack_source_type_from_path(path: &std::path::Path) -> oxc_span::SourceType {
  match path.extension().and_then(|ext| ext.to_str()) {
    Some("ts") => oxc_span::SourceType::ts(),
    Some("tsx") => oxc_span::SourceType::tsx(),
    Some("jsx") => oxc_span::SourceType::jsx(),
    Some("cjs") => oxc_span::SourceType::cjs(),
    _ => oxc_span::SourceType::jsx(),
  }
}
