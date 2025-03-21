use crate::inner::v2025_06_01::category::Category20250601Inner;

#[derive(Debug, Clone)]
pub enum Category {
  V20250601Inner(Category20250601Inner),
}
