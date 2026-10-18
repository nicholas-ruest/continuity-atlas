# Architecture

    ## Bounded contexts
    1. **Fleet Knowledge** owns service identity, region capacity, topology, and observed placements.
    2. **Uncertainty Lab** owns scenario generation, impact loss, and Wasserstein-CVaR scoring.
    3. **Continuity Control** owns candidate plans, invariants, and Rego authorization.
    4. **Incident Memory** owns RuVector analogue recall and reusable incident features.
    5. **Evidence & Recovery** owns append-only receipts, RVF witnesses, and deterministic replay.
    6. **Runtime & Evaluation** owns configuration, telemetry, CLI execution, baselines, and ablations.

    No single ingredient provides this capability: Regorus decides policy but has no graph, tail-risk optimizer, memory, or recovery evidence; RuVector recalls analogues but cannot authorize or plan; RVF seals bytes but cannot decide; the DRO mechanism ranks plans but does not persist memory or enforce policy.

    ## End-to-end flow
    Snapshot -> topology validation -> scenario stress -> RuVector recall -> candidate generation -> Wasserstein-CVaR ranking -> Regorus authorization -> append-only journal -> RVF seal -> recovery replay -> evaluation evidence.

    ## Workspace map
    All 22 crates are required by the workload. Library crates have reverse callers; `ca-cli` and `ca-evaluation` are useful executable roots.

    | Crate | Bounded context | Typed public API | Real callers | Acceptance | Traceability |
    |---|---|---|---|---|---|
    | `ca-identity` | Shared kernel | `ServiceId, RegionId` | service, region, events | reject blank/unsafe IDs | ADR-001..025; DDD-01..12 |
| `ca-service` | Service inventory | `Service::new; Service::required_capacity` | topology, snapshot, plan | tier and capacity invariants | ADR-001..025; DDD-01..12 |
| `ca-region` | Regional capacity | `Region::new; Region::available` | snapshot, scenario, plan | health/capacity bounds | ADR-001..025; DDD-01..12 |
| `ca-topology` | Dependency graph | `Topology::add_service/add_edge/closure` | snapshot, application | acyclic typed service graph | ADR-001..025; DDD-01..12 |
| `ca-snapshot` | Observed state | `FleetSnapshot::place/signal` | scenario, application | placement totals and freshness | ADR-001..025; DDD-01..12 |
| `ca-scenario` | Outage scenarios | `ScenarioSet::from_snapshot/stress` | loss, dro | deterministic weighted scenarios | ADR-001..025; DDD-01..12 |
| `ca-loss` | Impact model | `LossModel::evaluate` | dro, evaluation | tier-weighted capacity/latency loss | ADR-001..025; DDD-01..12 |
| `ca-dro` | Robust optimization | `RobustRanker::score/rank` | plan, evaluation | Wasserstein-CVaR bound | ADR-001..025; DDD-01..12 |
| `ca-plan` | Failover planning | `Planner::candidates/select` | application, recovery | feasible least-risk traffic shifts | ADR-001..025; DDD-01..12 |
| `ca-policy` | Policy contracts | `PolicyInput, PolicyDecision` | Regorus adapter, application | fail-closed authorization | ADR-001..025; DDD-01..12 |
| `ca-events` | Domain events | `DomainEvent::new` | adapters, evidence, telemetry | monotonic evidence sequence | ADR-001..025; DDD-01..12 |
| `ca-ports` | Hexagonal ports | `PolicyEngine, IncidentMemory, EvidenceSealer` | application, adapters | typed upstream ownership | ADR-001..025; DDD-01..12 |
| `ca-adapter-regorus` | Enterprise policy | `RegorusPolicy::evaluate` | application | real Rego allow/deny | ADR-001..025; DDD-01..12 |
| `ca-adapter-ruvector` | Incident memory | `IncidentRecall::open/remember/similar` | application | persistent insert/search/reopen | ADR-001..025; DDD-01..12 |
| `ca-adapter-rvf` | Witness sealing | `seal/verify` | application | tamper rejection | ADR-001..025; DDD-01..12 |
| `ca-evidence` | Append-only journal | `Journal::append/readback` | application, recovery | hash-linked durable receipts | ADR-001..025; DDD-01..12 |
| `ca-recovery` | Recovery replay | `RecoveryVerifier::replay` | application, evaluation | tier-0 preserved after plan | ADR-001..025; DDD-01..12 |
| `ca-application` | Workflow orchestration | `execute` | CLI, evaluation | real end-to-end workload | ADR-001..025; DDD-01..12 |
| `ca-config` | Runtime configuration | `Config::validate` | application, CLI | bounded alpha/radius/paths | ADR-001..025; DDD-01..12 |
| `ca-telemetry` | Structured observability | `init/record` | application, CLI | domain-event tracing | ADR-001..025; DDD-01..12 |
| `ca-cli` | Operator interface | `safe_state_path; continuity-atlas binary` | executable root | safe paths and JSON I/O | ADR-001..025; DDD-01..12 |
| `ca-evaluation` | Evidence evaluation | `run_suite; Metrics` | executable root | combined/baselines/ablations | ADR-001..025; DDD-01..12 |

    ## Dependency rules
    Domain crates never depend on adapters. Ports depend only on typed domain contracts. The application composes adapters; CLI and evaluation are roots. Only the integration owner changes workspace manifests and lockfiles.

    ## Failure handling
    Invalid identifiers, stale snapshots, impossible capacity, malformed Rego, policy denial, database errors, witness tampering, unsafe paths, and incomplete recovery all return typed errors. No failure mutates a previously accepted journal state.
