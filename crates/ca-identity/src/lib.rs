use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct ServiceId(String);
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct RegionId(String);
#[derive(Debug, Error)]
pub enum IdentityError {
    #[error("identifier must be lowercase alphanumeric or hyphen")]
    Invalid,
}
fn valid(v: &str) -> bool {
    !v.is_empty()
        && v.len() <= 64
        && v.bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}
impl ServiceId {
    pub fn parse(v: impl Into<String>) -> Result<Self, IdentityError> {
        let v = v.into();
        if valid(&v) {
            Ok(Self(v))
        } else {
            Err(IdentityError::Invalid)
        }
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl RegionId {
    pub fn parse(v: impl Into<String>) -> Result<Self, IdentityError> {
        let v = v.into();
        if valid(&v) {
            Ok(Self(v))
        } else {
            Err(IdentityError::Invalid)
        }
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_unsafe() {
        assert!(ServiceId::parse("../x").is_err());
        assert!(RegionId::parse("US East").is_err());
    }
}
