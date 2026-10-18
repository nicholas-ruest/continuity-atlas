use ca_application::Outcome;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metrics {
    pub mode: String,
    pub selected_plan: String,
    pub unsafe_candidate_rejected: bool,
    pub analogue_recall: bool,
    pub evidence_verifiable: bool,
    pub robust_control: bool,
    pub recovery_verified: bool,
    pub objective_upper_bound: f64,
    pub scenario_count: usize,
}

pub fn score(mode: &str, outcome: &Outcome) -> Metrics {
    Metrics {
        mode: mode.into(),
        selected_plan: outcome.selected.id.clone(),
        unsafe_candidate_rejected: outcome.unsafe_candidate_rejected,
        analogue_recall: outcome.ruvector_executed && outcome.analogue_count > 0,
        evidence_verifiable: outcome.rvf_executed && outcome.evidence_verifiable,
        robust_control: outcome.dro_radius > 0.0,
        recovery_verified: outcome.recovery_verified,
        objective_upper_bound: outcome.selected.score.as_ref().unwrap().robust_upper,
        scenario_count: outcome.scenario_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metrics_type_is_serializable() {
        let metrics = Metrics {
            mode: "x".into(),
            selected_plan: "hold".into(),
            unsafe_candidate_rejected: true,
            analogue_recall: true,
            evidence_verifiable: true,
            robust_control: true,
            recovery_verified: true,
            objective_upper_bound: 1.0,
            scenario_count: 3,
        };
        assert!(serde_json::to_string(&metrics).is_ok());
    }
}
