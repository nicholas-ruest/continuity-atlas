# DDD-11: Domain events

## Context
This document defines domain events for the regional failover workload and prevents terminology or authority from leaking across bounded contexts.

## Model
Primary concepts: SnapshotFrozen through RecoveryVerified. Entities carry validated identities; value objects are immutable; aggregates reject transitions that would weaken tier-0 continuity or evidence ordering.

## Language
`SnapshotFrozen` begins the evidence sequence, `ScenariosGenerated` binds the uncertainty set, `AnalogueRecalled` records memory contribution, `PlanRanked` binds the robust objective, `PolicyEvaluated` records authority, `EvidenceSealed` records RVF success, and `RecoveryVerified` closes the replay. Sequence numbers are part of the invariant.

## Invariants
Identifiers are non-empty and path-safe; event sequences are monotonic; tier-0 retains its minimum healthy placement; only policy-allowed feasible plans may be sealed; replay must reproduce the accepted outcome.

## Ownership
The owning crates named in `architecture.md` create and mutate these models. Adapters translate external representations and never own domain truth. The application orchestrates but does not redefine invariants.

## Events
Relevant events are emitted from `SnapshotFrozen`, `ScenariosGenerated`, `AnalogueRecalled`, `PlanRanked`, `PolicyEvaluated`, `EvidenceSealed`, and `RecoveryVerified`, with context-specific detail.

## Validation
Focused unit tests prove aggregate invariants; the integration workload crosses the context map; failure tests reject invalid transitions; evaluation ablations demonstrate the context's measurable contribution.
