# Semantic evidence review

## Workspace linkage

Cargo metadata reports 22 workspace packages and 62 internal dependency edges. The dependency-direction check reports no zero-reverse-caller libraries; the only permitted roots are the `ca-cli` and `ca-evaluation` executables. Each package owns a typed domain responsibility and is referenced from the architecture map, ADR index, DDD index, and implementation contract.

## Source-bound upstream execution

- `ca-adapter-regorus` calls `Engine::add_policy`, `add_data_json`, `set_input_json`, and `eval_rule` from Regorus 0.13.0. The integration workload observes rejection of the deliberately unsafe tier-0 candidate.
- `ca-adapter-ruvector` creates a four-dimensional persistent `VectorDB`, inserts a typed event as metadata, and performs HNSW-backed search. The persistent-roundtrip test reopens the database.
- `ca-adapter-rvf` calls `create_witness_chain` and `verify_witness_chain` from RVF crypto 0.2.0. The tamper test flips an encoded witness byte and observes rejection.
- `ca-dro` computes empirical mean, upper-tail CVaR, Wasserstein ambiguity penalty, and robust upper bound. Removing DRO sets the radius to zero in a fresh workload execution and changes the objective from 8.75 to 8.0.

## Meaningfulness review

The crates are grouped into fleet knowledge, uncertainty modelling, continuity control, policy, incident memory, evidence/recovery, and runtime/evaluation bounded contexts. The application orchestrator reaches every group on the normal workload. Recovery and failure tests cover invalid identifiers, unknown regions, unsafe policy candidates, witness tampering, empty journals, graph cycles, and missing feasible plans.

## Evidence interpretation

The combined path retains analogue recall, tier-0 policy rejection, cryptographic evidence verification, robust control, and recovery verification. Each executed ablation loses only the capability owned by the removed ingredient. Enterprise-only and Ruvnet-only baselines demonstrate that neither ingredient family supplies the composed capability by itself.

## Review conclusion

The workspace is not a ceremonial split: source inspection, Cargo metadata, integration tests, and ablation output show live caller paths across all bounded contexts. Production deployment remains outside scope, as documented in the README and validation record.
