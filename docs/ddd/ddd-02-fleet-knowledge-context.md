# DDD-02: Fleet Knowledge context

## Context
This document defines fleet knowledge context for the regional failover workload and prevents terminology or authority from leaking across bounded contexts.

## Model
Primary concepts: Service, Region, Topology, FleetSnapshot. Entities carry validated identities; value objects are immutable; aggregates reject transitions that would weaken tier-0 continuity or evidence ordering.

## Language
`Service` is a recoverable workload, `Region` is a failure domain with capacity and health, `Placement` allocates service capacity to a region, and `FleetSnapshot` is the immutable topology version used by one decision. A snapshot is frozen before uncertainty is generated so later telemetry cannot silently change the decision basis.

## Invariants
Identifiers are non-empty and path-safe; event sequences are monotonic; tier-0 retains its minimum healthy placement; only policy-allowed feasible plans may be sealed; replay must reproduce the accepted outcome.

## Ownership
The owning crates named in `architecture.md` create and mutate these models. Adapters translate external representations and never own domain truth. The application orchestrates but does not redefine invariants.

## Events
Relevant events are emitted from `SnapshotFrozen`, `ScenariosGenerated`, `AnalogueRecalled`, `PlanRanked`, `PolicyEvaluated`, `EvidenceSealed`, and `RecoveryVerified`, with context-specific detail.

## Validation
Focused unit tests prove aggregate invariants; the integration workload crosses the context map; failure tests reject invalid transitions; evaluation ablations demonstrate the context's measurable contribution.
