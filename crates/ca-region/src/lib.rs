use ca_identity::RegionId;
use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Region {
    pub id: RegionId,
    pub capacity: u32,
    pub health: f64,
    pub latency_ms: u32,
}
#[derive(Debug, Error)]
pub enum RegionError {
    #[error("capacity, health or latency invalid")]
    Invalid,
}
impl Region {
    pub fn new(
        id: RegionId,
        capacity: u32,
        health: f64,
        latency_ms: u32,
    ) -> Result<Self, RegionError> {
        if capacity == 0 || !(0.0..=1.0).contains(&health) || latency_ms == 0 {
            return Err(RegionError::Invalid);
        }
        Ok(Self {
            id,
            capacity,
            health,
            latency_ms,
        })
    }
    pub fn available(&self) -> u32 {
        (f64::from(self.capacity) * self.health).floor() as u32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn health() {
        assert!(Region::new(RegionId::parse("east").unwrap(), 10, 1.2, 1).is_err());
    }
}
