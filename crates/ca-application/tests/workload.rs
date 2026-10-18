use ca_application::{ExecutionOptions, IncidentInput, execute, execute_with_options};
use ca_config::Config;

#[test]
fn combined_workload_executes_all_integrations() {
    let directory = tempfile::tempdir().unwrap();
    let outcome = execute(
        &IncidentInput {
            failed_region: "us-east".into(),
        },
        directory.path(),
        Config::default(),
    )
    .unwrap();
    assert!(outcome.ruvector_executed);
    assert!(outcome.rvf_executed);
    assert!(outcome.regorus_executed);
    assert!(outcome.analogue_count > 0);
    assert!(outcome.unsafe_candidate_rejected);
    assert!(outcome.evidence_verifiable);
    assert!(outcome.recovery_verified);
}

#[test]
fn ingredient_ablations_change_observed_capabilities() {
    let directory = tempfile::tempdir().unwrap();
    let outcome = execute_with_options(
        &IncidentInput {
            failed_region: "us-east".into(),
        },
        directory.path(),
        Config::default(),
        ExecutionOptions {
            use_ruvector: false,
            use_rvf: false,
            use_regorus: false,
            use_dro: false,
        },
    )
    .unwrap();
    assert_eq!(outcome.analogue_count, 0);
    assert!(!outcome.unsafe_candidate_rejected);
    assert!(!outcome.evidence_verifiable);
    assert_eq!(outcome.dro_radius, 0.0);
    assert!(outcome.recovery_verified);
}
