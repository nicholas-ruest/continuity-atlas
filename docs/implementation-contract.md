# Implementation contract

    ## Vertical slices
    1. Model and validate fleet identities, regions, services, topology, and snapshot.
    2. Generate correlated scenarios and compute tier-weighted impact loss.
    3. Rank failover plans with empirical CVaR and Wasserstein ambiguity penalty.
    4. Execute Regorus authorization and RuVector recall through typed adapters.
    5. Append events, seal with RVF, replay recovery, and prove tier-0 continuity.
    6. Expose safe CLI and frozen evaluation with combined/baseline/ablation modes.

    ## Per-crate contract
    | Crate | Implementation task | Acceptance test |
    |---|---|---|
    | `ca-identity` | Implement shared kernel through `ServiceId, RegionId` with typed errors | reject blank/unsafe IDs; invoked by service, region, events |
| `ca-service` | Implement service inventory through `Service::new; Service::required_capacity` with typed errors | tier and capacity invariants; invoked by topology, snapshot, plan |
| `ca-region` | Implement regional capacity through `Region::new; Region::available` with typed errors | health/capacity bounds; invoked by snapshot, scenario, plan |
| `ca-topology` | Implement dependency graph through `Topology::add_service/add_edge/closure` with typed errors | acyclic typed service graph; invoked by snapshot, application |
| `ca-snapshot` | Implement observed state through `FleetSnapshot::place/signal` with typed errors | placement totals and freshness; invoked by scenario, application |
| `ca-scenario` | Implement outage scenarios through `ScenarioSet::from_snapshot/stress` with typed errors | deterministic weighted scenarios; invoked by loss, dro |
| `ca-loss` | Implement impact model through `LossModel::evaluate` with typed errors | tier-weighted capacity/latency loss; invoked by dro, evaluation |
| `ca-dro` | Implement robust optimization through `RobustRanker::score/rank` with typed errors | Wasserstein-CVaR bound; invoked by plan, evaluation |
| `ca-plan` | Implement failover planning through `Planner::candidates/select` with typed errors | feasible least-risk traffic shifts; invoked by application, recovery |
| `ca-policy` | Implement policy contracts through `PolicyInput, PolicyDecision` with typed errors | fail-closed authorization; invoked by Regorus adapter, application |
| `ca-events` | Implement domain events through `DomainEvent::new` with typed errors | monotonic evidence sequence; invoked by adapters, evidence, telemetry |
| `ca-ports` | Implement hexagonal ports through `PolicyEngine, IncidentMemory, EvidenceSealer` with typed errors | typed upstream ownership; invoked by application, adapters |
| `ca-adapter-regorus` | Implement enterprise policy through `RegorusPolicy::evaluate` with typed errors | real Rego allow/deny; invoked by application |
| `ca-adapter-ruvector` | Implement incident memory through `IncidentRecall::open/remember/similar` with typed errors | persistent insert/search/reopen; invoked by application |
| `ca-adapter-rvf` | Implement witness sealing through `seal/verify` with typed errors | tamper rejection; invoked by application |
| `ca-evidence` | Implement append-only journal through `Journal::append/readback` with typed errors | hash-linked durable receipts; invoked by application, recovery |
| `ca-recovery` | Implement recovery replay through `RecoveryVerifier::replay` with typed errors | tier-0 preserved after plan; invoked by application, evaluation |
| `ca-application` | Implement workflow orchestration through `execute` with typed errors | real end-to-end workload; invoked by CLI, evaluation |
| `ca-config` | Implement runtime configuration through `Config::validate` with typed errors | bounded alpha/radius/paths; invoked by application, CLI |
| `ca-telemetry` | Implement structured observability through `init/record` with typed errors | domain-event tracing; invoked by application, CLI |
| `ca-cli` | Implement operator interface through `safe_state_path; continuity-atlas binary` with typed errors | safe paths and JSON I/O; invoked by executable root |
| `ca-evaluation` | Implement evidence evaluation through `run_suite; Metrics` with typed errors | combined/baselines/ablations; invoked by executable root |

    ## Completion gates
    Twenty-two linked crates; exact upstream pins; real integration and failure paths; strict formatting/Clippy/locked tests; dependency/security review; reproducible benchmark and all ablations; 25 ADRs; 12 DDD documents; two animated SVGs; semantic linkage review; PR/CI/merge; public repository and two owner-read-back Gists.
