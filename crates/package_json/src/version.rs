use crate::error::Error;

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
  pub version: String,
}

impl Version {
  pub fn new(value: String, operator: SemverOperator, protocol: Protocol, version: String) -> Self {
    Self {
      value,
      operator,
      protocol,
      version,
    }
  }

  fn parse_operator(version: &str) -> Result<SemverOperator, Error> {
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

  fn parse_protocol(version: &str) -> Result<Protocol, Error> {
    match version {
      s if s.starts_with("git") => Ok(Protocol::Git),
      s if s.starts_with("file") => Ok(Protocol::File),
      s if s.starts_with("http") => Ok(Protocol::Http),
      s if s.starts_with("https") => Ok(Protocol::Https),
      s if s.starts_with("npm:") => Ok(Protocol::Npm),
      _ => Ok(Protocol::None),
    }
  }

  fn parse_version(version: &str) -> Result<String, Error> {
    let version = version.rsplit("@").next().unwrap_or(version);
    Ok(version.to_string())
  }
}

impl TryFrom<String> for Version {
  type Error = Error;
  fn try_from(value: String) -> Result<Self, Self::Error> {
    let protocol = Self::parse_protocol(&value)?;
    let operator = Self::parse_operator(&value)?;
    let version = if protocol == Protocol::Npm {
      Self::parse_version(&value)?
    } else {
      value.clone()
    };
    Ok(Self::new(value, operator, protocol, version))
  }
}
