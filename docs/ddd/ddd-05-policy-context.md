# DDD-05: Policy context

## Context
This document defines policy context for the regional failover workload and prevents terminology or authority from leaking across bounded contexts.

## Model
Primary concepts: PolicyInput, PolicyDecision, PolicyDenied. Entities carry validated identities; value objects are immutable; aggregates reject transitions that would weaken tier-0 continuity or evidence ordering.

## Language
`PolicyInput` is the anti-corruption representation sent to Rego, `PolicyDecision` is Regorus' typed allow/deny result, `tier0_violation` marks a move that weakens the critical replica invariant, and `destination_utilization` is predicted post-move load. Advisory mode still fails closed before sealing a denied plan.

## Invariants
Identifiers are non-empty and path-safe; event sequences are monotonic; tier-0 retains its minimum healthy placement; only policy-allowed feasible plans may be sealed; replay must reproduce the accepted outcome.

## Ownership
The owning crates named in `architecture.md` create and mutate these models. Adapters translate external representations and never own domain truth. The application orchestrates but does not redefine invariants.

## Events
Relevant events are emitted from `SnapshotFrozen`, `ScenariosGenerated`, `AnalogueRecalled`, `PlanRanked`, `PolicyEvaluated`, `EvidenceSealed`, and `RecoveryVerified`, with context-specific detail.

## Validation
Focused unit tests prove aggregate invariants; the integration workload crosses the context map; failure tests reject invalid transitions; evaluation ablations demonstrate the context's measurable contribution.
