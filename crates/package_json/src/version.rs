use crate::error::{AliasParserErr, VersionError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemverOperator {
  // 1.2.3 精确版本
  Exact,
  // ^1.2.3  兼容版本
  Compatible,
  // ~1.2.3  补丁版本
  Patch,
  // >=1.2.3 大于等于
  Gte,
  // <=1.2.3 小于等于
  Lte,
  // >1.2.3  大于
  Gt,
  // <1.2.3  小于
  Lt,
  // 1.2.x 1.2.X 1.x.x 1.x.X
  Wildcard,
  // *
  Any,
}

impl Default for SemverOperator {
  fn default() -> Self {
    Self::Compatible
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Protocol {
  Npm,
  Git,
  File,
  Http,
  Https,
  None,
}

impl Default for Protocol {
  fn default() -> Self {
    Self::None
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
  pub value: String,
  pub operator: SemverOperator,
  pub protocol: Protocol,
  pub version: Option<String>,
}

impl Version {
  pub fn new(
    value: String,
    operator: SemverOperator,
    protocol: Protocol,
    version: Option<String>,
  ) -> Self {
    Self {
      value,
      operator,
      protocol,
      version,
    }
  }

  fn parse_operator(version: &str) -> Result<SemverOperator, VersionError> {
    match version {
      s if s.starts_with('^') => Ok(SemverOperator::Compatible),
      s if s.starts_with('~') => Ok(SemverOperator::Patch),
      s if s.starts_with(">=") => Ok(SemverOperator::Gte),
      s if s.starts_with("<=") => Ok(SemverOperator::Lte),
      s if s.starts_with('>') => Ok(SemverOperator::Gt),
      s if s.starts_with('<') => Ok(SemverOperator::Lt),
      s if s == "*" => Ok(SemverOperator::Any),
      s if s.contains(['x', 'X', '*']) => Ok(SemverOperator::Wildcard),
      _ => Ok(SemverOperator::Exact),
    }
  }

  fn parse_protocol(version: &str) -> Result<Protocol, VersionError> {
    match version {
      s if s.starts_with("git") => Ok(Protocol::Git),
      s if s.starts_with("file") => Ok(Protocol::File),
      s if s.starts_with("http") => Ok(Protocol::Http),
      s if s.starts_with("https") => Ok(Protocol::Https),
      s if s.starts_with("npm:") => Ok(Protocol::Npm),
      _ => Ok(Protocol::None),
    }
  }
}

impl TryFrom<String> for Version {
  type Error = VersionError;
  fn try_from(value: String) -> Result<Self, Self::Error> {
    let protocol = Self::parse_protocol(&value)?;
    let operator = Self::parse_operator(&value)?;

    let version = match protocol {
      Protocol::Npm => Some(
        value
          .rsplit("@")
          .next()
          .ok_or(
            AliasParserErr::builder()
              .version(value.clone())
              .build()
              .into(),
          )?
          .to_string(),
      ),
      Protocol::Git | Protocol::File | Protocol::Http | Protocol::Https => None,
      Protocol::None => Some(value.clone()),
    };

    Ok(Self::new(value, operator, protocol, version))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_operator() {
    let operator = Version::parse_operator("^1.2.3").unwrap();
    assert_eq!(operator, SemverOperator::Compatible);
  }

  #[test]
  fn test_parse_protocol() {
    let protocol = Version::parse_protocol("npm:react@18.0.0").unwrap();
    assert_eq!(protocol, Protocol::Npm);
  }

  #[test]
  fn test_try_from() {
    let version = Version::try_from("npm:react@18.0.0".to_string()).unwrap();
    assert_eq!(version.operator, SemverOperator::Exact);
    assert_eq!(version.protocol, Protocol::Npm);
    assert_eq!(version.version, Some("18.0.0".to_string()));
  }
}
