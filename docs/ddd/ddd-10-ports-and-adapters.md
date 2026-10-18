# DDD-10: Ports and adapters

## Context
This document defines ports and adapters for the regional failover workload and prevents terminology or authority from leaking across bounded contexts.

## Model
Primary concepts: PolicyEngine, IncidentMemory, EvidenceSealer. Entities carry validated identities; value objects are immutable; aggregates reject transitions that would weaken tier-0 continuity or evidence ordering.

## Language
`Port` is a domain-owned capability contract, `adapter` translates one upstream implementation, `PolicyEngine` evaluates a typed candidate, `IncidentMemory` persists and recalls incident features, and `EvidenceSealer` binds ordered events. Adapters may fail explicitly but may not manufacture a domain success.

## Invariants
Identifiers are non-empty and path-safe; event sequences are monotonic; tier-0 retains its minimum healthy placement; only policy-allowed feasible plans may be sealed; replay must reproduce the accepted outcome.

## Ownership
The owning crates named in `architecture.md` create and mutate these models. Adapters translate external representations and never own domain truth. The application orchestrates but does not redefine invariants.

## Events
Relevant events are emitted from `SnapshotFrozen`, `ScenariosGenerated`, `AnalogueRecalled`, `PlanRanked`, `PolicyEvaluated`, `EvidenceSealed`, and `RecoveryVerified`, with context-specific detail.

## Validation
Focused unit tests prove aggregate invariants; the integration workload crosses the context map; failure tests reject invalid transitions; evaluation ablations demonstrate the context's measurable contribution.
