# Validation record

Continuity Atlas is accepted only when the exact indexed candidate passes the locked Rust workspace gates, executes every upstream boundary in the end-to-end workload, and produces reproducible combined/baseline/ablation evidence. Structural acceptance is necessary but explicitly insufficient.

## Required commands

1. `cargo fmt --all --check`
2. `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
3. `cargo test --locked --workspace --all-features`
4. `cargo deny check`
5. `python3 scripts/check-dependency-direction.py`
6. `bash scripts/run-evaluation.sh`
7. `node /home/ruv/.local/share/dream-machine/factory-checker-80f006f/check-project-contract.test.mjs`
8. `node /home/ruv/.local/share/dream-machine/factory-checker-80f006f/check-project-contract.mjs .`

## Executed boundaries

The `combined_workload_executes_all_integrations` integration test performs a persistent RuVector insert/search, evaluates the failover plans with Microsoft Regorus, creates and verifies an RVF witness chain, computes the Wasserstein DRO-CVaR objective, appends the decision journal, and replays recovery. `ingredient_ablations_change_observed_capabilities` removes all four ingredients and asserts the lost capabilities rather than editing a completed result.

## Evidence policy

`evidence/evaluation.jsonl` is generated from seven separate workload executions. Each line records the selected plan, policy rejection, analogue recall, RVF verification, robust-control state, recovery result, objective bound, scenario count, and measured wall time. Console output is retained separately in `evidence/evaluation.raw.log`; only valid JSON objects enter the JSONL file. Exact command receipts and exit codes are captured in `evidence/validation.txt`.

## Limitations

The frozen fixture is synthetic, the benchmark is a local single-process micro-workload, and the control plane remains advisory. The evidence establishes executable composition and deterministic capability changes; it does not establish production SLOs, cloud-provider correctness, or universal optimality of the ambiguity radius.

## External harness receipts

Ruflo 3.56.10 recorded design coordination. MetaHarness Darwin 0.12.0 created and
verified a one-task repo-native suite with hash prefix `96cc5e623971`. MetaHarness
Flywheel 0.1.12 executed a two-generation signed replay smoke test with
`authority=none`. Exact commands, outputs, package pins, and limitations are retained
in `evidence/ruflo-execution-ledger.md` and
`evidence/metaharness-execution-ledger.md`. These receipts do not replace or inflate
the product test and benchmark results above.
