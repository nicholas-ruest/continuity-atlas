# Research and candidate selection

## Candidate comparison
- **Continuity Atlas — selected:** system-scale regional failover control joining topology, memory, robust optimization, policy, evidence, and recovery.
- **Rego plan linter — rejected:** narrow validator; no persistence, orchestration, recovery, or evaluation.
- **Incident similarity CLI — rejected:** RuVector wrapper without policy or end-to-end control.
- **Retry-budget service — rejected:** near-duplicate of JitterMap.

## Enterprise OSS
Microsoft Regorus 0.13.0 is a Rust Rego interpreter. Source: `microsoft/regorus@1833df6c12d24142bad0e8ae51754b72bfeae1b9`; license expression from crates.io: `MIT AND Apache-2.0 AND BSD-3-Clause`. The implementation executes `Engine::add_policy`, `add_data`, `set_input`, and `eval_rule` on the real policy path.

## Ruvnet composition
RuVector `ruvector-core` 2.4.0 reuses durable vector insertion and HNSW search (`ruvnet/ruvector@e3d82b19683a08c6392a2534bafc90927ca690c7`). RVF `rvf-crypto` 0.2.0 reuses SHAKE-256 witness-chain creation and verification (`ruvnet/ruvector@9fb3d2c63b19ac6950b931f4adb56834bf96c45d`). Their responsibilities are distinct and both are in the combined workload.

## Frontier mechanism
Wasserstein distributionally robust optimization follows Esfahani and Kuhn, *Data-driven distributionally robust optimization using the Wasserstein metric*, Mathematical Programming 171 (2018), DOI `10.1007/s10107-017-1172-1`. Continuity Atlas implements a bounded empirical CVaR tail objective plus an explicit ambiguity-radius sensitivity penalty. It materially changes plan selection and is ablated independently.

## Hypothesis
Under the frozen correlated-region stress set, a policy-authorized plan ranked by robust CVaR will reduce worst-case tail loss versus empirical-only and enterprise-only baselines, while RuVector increases analogue recall and RVF makes evidence tamper-evident.
