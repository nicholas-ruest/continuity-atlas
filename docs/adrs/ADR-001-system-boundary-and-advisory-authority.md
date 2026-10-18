# ADR-001: System boundary and advisory authority

## Context
Continuity Atlas must make regional failover decisions under partial evidence without confusing a plausible plan with authorized execution. This decision governs `system boundary and advisory authority` in the frozen end-to-end workload.

## Viable alternatives
1. **Direct cloud mutations** — simpler initially, but it cannot provide the owned safety/evidence property.
2. **External opaque service** — operationally mature, but would make the evaluation network-dependent and hide exact decision semantics.
3. **Project-local explicit mechanism** — more implementation work, but deterministic, testable, and source-bound.

## Decision
Use advisory plan plus external authority. The owning crate exposes a typed API; callers cannot bypass the invariant through raw strings or unverified booleans.

## Tradeoffs
This adds a boundary and focused tests. It intentionally prefers explainability and reproducibility over live-cloud breadth or automatic authority.

## Consequences
The combined workflow can attribute behavior to this decision. Failure is explicit, evidence remains append-only, and the associated ablation can remove only this capability.

## Validation
Unit-test the invariant, execute it through `ca-application`, cover its failure path, and verify the corresponding combined-versus-ablation metric in `ca-evaluation`.
