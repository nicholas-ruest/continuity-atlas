# DDD-07: Evidence context

## Context
This document defines evidence context for the regional failover workload and prevents terminology or authority from leaking across bounded contexts.

## Model
Primary concepts: DomainEvent, JournalReceipt, WitnessSeal. Entities carry validated identities; value objects are immutable; aggregates reject transitions that would weaken tier-0 continuity or evidence ordering.

## Language
`DomainEvent` is a sequenced fact emitted by the control loop, `JournalReceipt` binds the event count to a SHA-256 root, `WitnessSeal` is the RVF-encoded witness chain, and `verifiable` means the pinned RVF verifier successfully reconstructs the entries. Append-only evidence is distinct from mutable runtime telemetry.

## Invariants
Identifiers are non-empty and path-safe; event sequences are monotonic; tier-0 retains its minimum healthy placement; only policy-allowed feasible plans may be sealed; replay must reproduce the accepted outcome.

## Ownership
The owning crates named in `architecture.md` create and mutate these models. Adapters translate external representations and never own domain truth. The application orchestrates but does not redefine invariants.

## Events
Relevant events are emitted from `SnapshotFrozen`, `ScenariosGenerated`, `AnalogueRecalled`, `PlanRanked`, `PolicyEvaluated`, `EvidenceSealed`, and `RecoveryVerified`, with context-specific detail.

## Validation
Focused unit tests prove aggregate invariants; the integration workload crosses the context map; failure tests reject invalid transitions; evaluation ablations demonstrate the context's measurable contribution.
