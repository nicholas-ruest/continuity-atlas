use ca_application::{ExecutionOptions, IncidentInput, execute_with_options};
use ca_config::Config;
use ca_evaluation::score;
use std::time::Instant;

fn main() {
    let modes = [
        ("combined", ExecutionOptions::default()),
        (
            "without-ruvector",
            ExecutionOptions {
                use_ruvector: false,
                ..ExecutionOptions::default()
            },
        ),
        (
            "without-rvf",
            ExecutionOptions {
                use_rvf: false,
                ..ExecutionOptions::default()
            },
        ),
        (
            "without-regorus",
            ExecutionOptions {
                use_regorus: false,
                ..ExecutionOptions::default()
            },
        ),
        (
            "without-dro",
            ExecutionOptions {
                use_dro: false,
                ..ExecutionOptions::default()
            },
        ),
        (
            "enterprise-only",
            ExecutionOptions {
                use_ruvector: false,
                use_rvf: false,
                use_regorus: true,
                use_dro: false,
            },
        ),
        (
            "ruvnet-only",
            ExecutionOptions {
                use_ruvector: true,
                use_rvf: true,
                use_regorus: false,
                use_dro: false,
            },
        ),
    ];

    for (mode, options) in modes {
        let directory = tempfile::tempdir().expect("temporary evaluation state");
        let started = Instant::now();
        let outcome = execute_with_options(
            &IncidentInput {
                failed_region: "us-east".into(),
            },
            directory.path(),
            Config::default(),
            options,
        )
        .expect("evaluation workload");
        let mut value = serde_json::to_value(score(mode, &outcome)).unwrap();
        value["elapsed_us"] = serde_json::json!(started.elapsed().as_micros());
        println!("{}", serde_json::to_string(&value).unwrap());
    }
}
