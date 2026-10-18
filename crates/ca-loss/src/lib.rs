use ca_identity::{RegionId, ServiceId};
use ca_scenario::{Scenario, ScenarioSet};
use ca_service::Service;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlanShape {
    pub moved: BTreeMap<ServiceId, RegionId>,
    pub tier0_violation: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LossVector(pub Vec<f64>);
#[derive(Debug, Error)]
pub enum LossError {
    #[error("scenario set or service inventory empty")]
    Empty,
}
pub struct LossModel;
impl LossModel {
    pub fn evaluate(
        services: &BTreeMap<ServiceId, Service>,
        scenarios: &ScenarioSet,
        plan: &PlanShape,
    ) -> Result<LossVector, LossError> {
        if services.is_empty() || scenarios.is_empty() {
            return Err(LossError::Empty);
        }
        Ok(LossVector(
            scenarios
                .0
                .iter()
                .map(|s| Self::one(services, s, plan))
                .collect(),
        ))
    }
    fn one(services: &BTreeMap<ServiceId, Service>, s: &Scenario, plan: &PlanShape) -> f64 {
        let moved_penalty = plan
            .moved
            .keys()
            .map(|id| {
                services
                    .get(id)
                    .map_or(0.0, |v| f64::from(4 - v.tier) * 0.8)
            })
            .sum::<f64>();
        let tier0 = if plan.tier0_violation { 100.0 } else { 0.0 };
        tier0 + moved_penalty + (s.latency_multiplier - 1.0) * 2.0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tier0_dominates() {
        let p = PlanShape {
            moved: BTreeMap::new(),
            tier0_violation: true,
        };
        let s = Scenario {
            id: "x".into(),
            weight: 1.,
            region_capacity: BTreeMap::new(),
            latency_multiplier: 1.,
        };
        assert!(LossModel::one(&BTreeMap::new(), &s, &p) > 90.);
    }
}
