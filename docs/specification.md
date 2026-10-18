# Continuity Atlas specification

## Original problem
During a regional cloud incident, SREs must decide which services and traffic to move before partial telemetry, correlated capacity loss, and policy constraints become a full outage. Static runbooks neither recall analogous incidents nor quantify tail risk, and a syntactically valid policy check does not prove that the resulting plan preserves tier-0 service.

## Intended users
Platform SREs, incident commanders, resilience engineers, and regulated-service owners operating multi-region systems.

## System capability
Continuity Atlas reconstructs regional service dependencies, retrieves analogous incidents, ranks feasible failover plans with Wasserstein distributionally robust CVaR, authorizes the winner through Microsoft Regorus/Rego, seals the decision with RVF, and replays recovery while proving tier-0 continuity.

## Real workload
A primary region loses 55% capacity while latency triples. Checkout depends on payments and identity; payments is tier-0 and must retain two healthy placements. The system ingests a frozen fleet snapshot, generates correlated scenarios, recalls a prior brownout with RuVector, compares traffic-shift plans, rejects a policy-breaking tier-0 move through Regorus, selects the lowest robust tail-loss plan, writes a hash-linked journal, seals it with RVF, and verifies replay.

## Success criteria
- zero tier-0 continuity violations in the combined run;
- Regorus denies the deliberately unsafe plan;
- RuVector returns at least one persisted analogue after reopen;
- RVF verifies the genuine chain and rejects a tampered chain;
- combined robust tail loss is lower than empirical-only and enterprise-only baselines under the frozen stress set;
- removing each major ingredient changes its owned capability metric;
- deterministic seed 20261017, bounded paths, explicit errors, no secrets.

## Constraints and exclusions
This is a reproducible control-plane evaluation, not a production traffic director. It never changes live cloud resources. Policy decisions are advisory and authority remains external. The fixture is synthetic and the Wasserstein radius is an explicit engineering parameter, not a universal statistical guarantee.
