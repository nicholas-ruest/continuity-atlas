use ca_dro::{DroError, RobustRanker, RobustScore};
use ca_identity::{RegionId, ServiceId};
use ca_loss::{LossModel, PlanShape};
use ca_policy::PolicyDecision;
use ca_region::Region;
use ca_service::Service;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FailoverPlan {
    pub id: String,
    pub moves: BTreeMap<ServiceId, RegionId>,
    pub tier0_violation: bool,
    pub policy: Option<PolicyDecision>,
    pub score: Option<RobustScore>,
}
#[derive(Debug, Error)]
pub enum PlanError {
    #[error("no feasible plan")]
    NoFeasible,
    #[error(transparent)]
    Dro(#[from] DroError),
    #[error("loss: {0}")]
    Loss(String),
}
pub fn candidates(
    services: &BTreeMap<ServiceId, Service>,
    regions: &BTreeMap<RegionId, Region>,
    safe: &RegionId,
) -> Vec<FailoverPlan> {
    let no_move = FailoverPlan {
        id: "hold".into(),
        moves: BTreeMap::new(),
        tier0_violation: false,
        policy: None,
        score: None,
    };
    let mut shift = no_move.clone();
    shift.id = "shift-noncritical".into();
    for s in services.values().filter(|s| s.tier > 0) {
        shift.moves.insert(s.id.clone(), safe.clone());
    }
    let mut unsafe_plan = no_move.clone();
    unsafe_plan.id = "move-tier0".into();
    unsafe_plan.tier0_violation = true;
    for s in services.values().filter(|s| s.tier == 0) {
        unsafe_plan.moves.insert(s.id.clone(), safe.clone());
    }
    let _ = regions;
    vec![no_move, shift, unsafe_plan]
}
pub fn score_plan(
    mut plan: FailoverPlan,
    services: &BTreeMap<ServiceId, Service>,
    scenarios: &ca_scenario::ScenarioSet,
    ranker: &RobustRanker,
) -> Result<FailoverPlan, PlanError> {
    let losses = LossModel::evaluate(
        services,
        scenarios,
        &PlanShape {
            moved: plan.moves.clone(),
            tier0_violation: plan.tier0_violation,
        },
    )
    .map_err(|e| PlanError::Loss(e.to_string()))?;
    plan.score = Some(ranker.score(&losses)?);
    Ok(plan)
}
pub fn select(plans: Vec<FailoverPlan>) -> Result<FailoverPlan, PlanError> {
    plans
        .into_iter()
        .filter(|p| p.policy.as_ref().is_some_and(|d| d.allowed))
        .min_by(|a, b| {
            a.score
                .as_ref()
                .unwrap()
                .robust_upper
                .total_cmp(&b.score.as_ref().unwrap().robust_upper)
        })
        .ok_or(PlanError::NoFeasible)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn denies_absent_policy() {
        assert!(select(vec![]).is_err());
    }
}
