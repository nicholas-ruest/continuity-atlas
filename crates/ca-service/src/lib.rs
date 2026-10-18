use ca_identity::ServiceId;
use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Service {
    pub id: ServiceId,
    pub tier: u8,
    pub min_healthy_regions: u8,
    pub demand: u32,
    pub latency_budget_ms: u32,
}
#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("tier must be 0..3 and capacities nonzero")]
    Invalid,
}
impl Service {
    pub fn new(
        id: ServiceId,
        tier: u8,
        min_healthy_regions: u8,
        demand: u32,
        latency_budget_ms: u32,
    ) -> Result<Self, ServiceError> {
        if tier > 3 || min_healthy_regions == 0 || demand == 0 || latency_budget_ms == 0 {
            return Err(ServiceError::Invalid);
        }
        Ok(Self {
            id,
            tier,
            min_healthy_regions,
            demand,
            latency_budget_ms,
        })
    }
    pub fn required_capacity(&self) -> u32 {
        self.demand
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn invariant() {
        assert!(Service::new(ServiceId::parse("pay").unwrap(), 4, 1, 1, 1).is_err());
    }
}
