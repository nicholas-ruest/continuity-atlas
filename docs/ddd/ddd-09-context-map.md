# DDD-09: Context map

## Context
This document defines context map for the regional failover workload and prevents terminology or authority from leaking across bounded contexts.

## Model
Primary concepts: upstream/downstream translations and ownership. Entities carry validated identities; value objects are immutable; aggregates reject transitions that would weaken tier-0 continuity or evidence ordering.

## Language
`Upstream` owns source facts, `downstream` consumes a translated contract, `conformist` accepts an upstream shape, and `anti-corruption layer` protects domain language from an external API. Fleet Knowledge feeds Uncertainty Lab; Continuity Control consumes scores; Policy authorizes; Evidence and Recovery observe the accepted decision.

## Invariants
Identifiers are non-empty and path-safe; event sequences are monotonic; tier-0 retains its minimum healthy placement; only policy-allowed feasible plans may be sealed; replay must reproduce the accepted outcome.

## Ownership
The owning crates named in `architecture.md` create and mutate these models. Adapters translate external representations and never own domain truth. The application orchestrates but does not redefine invariants.

## Events
Relevant events are emitted from `SnapshotFrozen`, `ScenariosGenerated`, `AnalogueRecalled`, `PlanRanked`, `PolicyEvaluated`, `EvidenceSealed`, and `RecoveryVerified`, with context-specific detail.

## Validation
Focused unit tests prove aggregate invariants; the integration workload crosses the context map; failure tests reject invalid transitions; evaluation ablations demonstrate the context's measurable contribution.
