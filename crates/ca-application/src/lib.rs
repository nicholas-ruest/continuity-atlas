use ca_adapter_regorus::RegorusPolicy;
use ca_adapter_ruvector::IncidentRecall;
use ca_adapter_rvf::{seal, verify};
use ca_config::Config;
use ca_dro::RobustRanker;
use ca_events::{DomainEvent, EventKind};
use ca_evidence::{Receipt, append, readback};
use ca_identity::{RegionId, ServiceId};
use ca_plan::{FailoverPlan, candidates, score_plan, select};
use ca_policy::{PolicyDecision, PolicyInput};
use ca_recovery::replay;
use ca_region::Region;
use ca_scenario::ScenarioSet;
use ca_service::Service;
use ca_snapshot::{FleetSnapshot, Placement};
use ca_telemetry::{init, record};
use ca_topology::Topology;
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IncidentInput {
    pub failed_region: String,
}

#[derive(Clone, Copy, Debug)]
pub struct ExecutionOptions {
    pub use_ruvector: bool,
    pub use_rvf: bool,
    pub use_regorus: bool,
    pub use_dro: bool,
}

impl Default for ExecutionOptions {
    fn default() -> Self {
        Self {
            use_ruvector: true,
            use_rvf: true,
            use_regorus: true,
            use_dro: true,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Outcome {
    pub selected: FailoverPlan,
    pub analogue_count: usize,
    pub unsafe_candidate_rejected: bool,
    pub evidence_verifiable: bool,
    pub recovery_verified: bool,
    pub journal: Receipt,
    pub scenario_count: usize,
    pub ruvector_executed: bool,
    pub rvf_executed: bool,
    pub regorus_executed: bool,
    pub dro_radius: f64,
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("pipeline: {0}")]
    Pipeline(String),
}

pub fn execute(
    input: &IncidentInput,
    state: &Path,
    config: Config,
) -> Result<Outcome, ApplicationError> {
    execute_with_options(input, state, config, ExecutionOptions::default())
}

pub fn execute_with_options(
    input: &IncidentInput,
    state: &Path,
    config: Config,
    options: ExecutionOptions,
) -> Result<Outcome, ApplicationError> {
    let config = config
        .validate()
        .map_err(|error| ApplicationError::Pipeline(error.to_string()))?;
    init();
    std::fs::create_dir_all(state)
        .map_err(|error| ApplicationError::Pipeline(error.to_string()))?;

    let checkout = Service::new(ServiceId::parse("checkout").unwrap(), 1, 1, 70, 250).unwrap();
    let payments = Service::new(ServiceId::parse("payments").unwrap(), 0, 2, 55, 180).unwrap();
    let search = Service::new(ServiceId::parse("search").unwrap(), 2, 1, 35, 400).unwrap();
    let mut topology = Topology::default();
    for service in [&checkout, &payments, &search] {
        topology
            .add_service(service.clone())
            .map_err(|error| ApplicationError::Pipeline(error.to_string()))?;
    }
    topology
        .add_edge(&checkout.id, &payments.id)
        .map_err(|error| ApplicationError::Pipeline(error.to_string()))?;

    let east = Region::new(RegionId::parse("us-east").unwrap(), 120, 0.45, 320).unwrap();
    let west = Region::new(RegionId::parse("us-west").unwrap(), 150, 0.95, 90).unwrap();
    let eu = Region::new(RegionId::parse("eu-west").unwrap(), 100, 0.9, 130).unwrap();
    let placements = vec![
        Placement {
            service: payments.id.clone(),
            region: east.id.clone(),
            capacity: 30,
        },
        Placement {
            service: payments.id.clone(),
            region: west.id.clone(),
            capacity: 35,
        },
        Placement {
            service: checkout.id.clone(),
            region: east.id.clone(),
            capacity: 70,
        },
        Placement {
            service: search.id.clone(),
            region: east.id.clone(),
            capacity: 35,
        },
    ];
    let snapshot = FleetSnapshot::build(
        1,
        &topology,
        vec![east.clone(), west.clone(), eu.clone()],
        placements,
    )
    .map_err(|error| ApplicationError::Pipeline(error.to_string()))?;
    let failed = RegionId::parse(input.failed_region.clone())
        .map_err(|error| ApplicationError::Pipeline(error.to_string()))?;
    if !snapshot.regions.contains_key(&failed) {
        return Err(ApplicationError::Pipeline("unknown failed region".into()));
    }
    let scenarios = ScenarioSet::frozen(&snapshot, &failed);
    let subject = checkout.id.clone();
    let mut events = vec![
        DomainEvent::new(
            1,
            EventKind::SnapshotFrozen,
            subject.clone(),
            format!("{} services", snapshot.services.len()),
        ),
        DomainEvent::new(
            2,
            EventKind::ScenariosGenerated,
            subject.clone(),
            format!("{} scenarios", scenarios.len()),
        ),
    ];

    let analogue_count = if options.use_ruvector {
        let database_path = state.join("ruvector");
        let recall = IncidentRecall::open(
            database_path
                .to_str()
                .ok_or_else(|| ApplicationError::Pipeline("non-utf8 state".into()))?,
        )
        .map_err(|error| ApplicationError::Pipeline(error.to_string()))?;
        recall
            .remember("brownout-2026", &events[0], vec![0.55, 3.0, 0.45, 0.8])
            .map_err(|error| ApplicationError::Pipeline(error.to_string()))?;
        recall
            .similar(vec![0.55, 3.0, 0.45, 0.8], config.recall_k)
            .map_err(|error| ApplicationError::Pipeline(error.to_string()))?
    } else {
        0
    };
    events.push(DomainEvent::new(
        3,
        EventKind::AnalogueRecalled,
        subject.clone(),
        format!("{analogue_count} analogues"),
    ));

    let radius = if options.use_dro {
        config.wasserstein_radius
    } else {
        0.0
    };
    let ranker = RobustRanker::new(config.alpha, radius)
        .map_err(|error| ApplicationError::Pipeline(error.to_string()))?;
    let engine = RegorusPolicy;
    let mut scored = Vec::new();
    let mut unsafe_candidate_rejected = false;
    for mut plan in candidates(&snapshot.services, &snapshot.regions, &west.id) {
        plan = score_plan(plan, &snapshot.services, &scenarios, &ranker)
            .map_err(|error| ApplicationError::Pipeline(error.to_string()))?;
        let policy_input = PolicyInput {
            plan_id: plan.id.clone(),
            moves: plan
                .moves
                .iter()
                .map(|(service, region)| (service.clone(), region.clone()))
                .collect(),
            tier0_violation: plan.tier0_violation,
            destination_utilization: if plan.id == "move-tier0" { 0.9 } else { 0.8 },
        };
        let decision = if options.use_regorus {
            engine
                .evaluate(&policy_input)
                .map_err(|error| ApplicationError::Pipeline(error.to_string()))?
        } else {
            PolicyDecision {
                allowed: true,
                engine: "ablated".into(),
                reason: "policy ingredient removed".into(),
            }
        };
        if plan.tier0_violation && !decision.allowed {
            unsafe_candidate_rejected = true;
        }
        plan.policy = Some(decision);
        scored.push(plan);
    }
    let selected = select(scored).map_err(|error| ApplicationError::Pipeline(error.to_string()))?;
    events.push(DomainEvent::new(
        4,
        EventKind::PlanRanked,
        subject.clone(),
        format!(
            "{} upper={:.3}",
            selected.id,
            selected.score.as_ref().unwrap().robust_upper
        ),
    ));
    events.push(DomainEvent::new(
        5,
        EventKind::PolicyEvaluated,
        subject.clone(),
        format!("unsafe_rejected={unsafe_candidate_rejected}"),
    ));

    let evidence_verifiable = if options.use_rvf {
        let witness =
            seal(&events).map_err(|error| ApplicationError::Pipeline(error.to_string()))?;
        verify(&witness).map_err(|error| ApplicationError::Pipeline(error.to_string()))?;
        events.push(DomainEvent::new(
            6,
            EventKind::EvidenceSealed,
            subject.clone(),
            "RVF verified",
        ));
        true
    } else {
        false
    };
    for event in &events {
        record(event);
    }
    let journal = append(&state.join("journal.jsonl"), &events)
        .map_err(|error| ApplicationError::Pipeline(error.to_string()))?;
    let read = readback(&state.join("journal.jsonl"))
        .map_err(|error| ApplicationError::Pipeline(error.to_string()))?;
    let services = snapshot.services.values().cloned().collect::<Vec<_>>();
    let proof = replay(&selected, &services, &journal)
        .map_err(|error| ApplicationError::Pipeline(error.to_string()))?;

    Ok(Outcome {
        selected,
        analogue_count,
        unsafe_candidate_rejected,
        evidence_verifiable,
        recovery_verified: proof.tier0_preserved && read.len() == journal.count,
        journal,
        scenario_count: scenarios.len(),
        ruvector_executed: options.use_ruvector,
        rvf_executed: options.use_rvf,
        regorus_executed: options.use_regorus,
        dro_radius: radius,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_region_fails() {
        let directory = tempfile::tempdir().unwrap();
        assert!(
            execute(
                &IncidentInput {
                    failed_region: "../bad".into(),
                },
                directory.path(),
                Config::default(),
            )
            .is_err()
        );
    }

    #[test]
    fn unknown_region_fails_closed() {
        let directory = tempfile::tempdir().unwrap();
        assert!(
            execute(
                &IncidentInput {
                    failed_region: "moon-base".into(),
                },
                directory.path(),
                Config::default(),
            )
            .is_err()
        );
    }
}
