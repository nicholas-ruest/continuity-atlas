# Validation plan

Run `cargo fmt --check`, strict all-target/all-feature Clippy with `-D warnings`, `cargo test --locked --workspace --all-features`, `cargo deny check`, real adapter tests, failure-path integration tests, dependency-direction review, frozen benchmark, combined/Ruvnet-only/enterprise-only baselines, four ingredient ablations, the pinned structural checker, semantic source readback, exact PR CI, and final repository/Gist readback.

Seeds, fixture, toolchain, source SHAs, commands, exit codes, and raw JSONL evidence are immutable inputs to the release decision. Structural pass means only `STRUCTURAL_PASS_REQUIRES_EVIDENCE_REVIEW`.
