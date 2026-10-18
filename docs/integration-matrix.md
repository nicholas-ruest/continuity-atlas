# Integration matrix

| Ingredient | Pin / license | Capability reused | Executed boundary | Crate | Integration/failure test | Ablation |
|---|---|---|---|---|---|---|
| Microsoft Regorus | 0.13.0; `1833df6...`; MIT AND Apache-2.0 AND BSD-3-Clause | Rego evaluation | `Engine::add_policy/add_data/set_input/eval_rule` | `ca-adapter-regorus` | safe plan allows; tier-0 violation denies; malformed policy errors | without Regorus admits unsafe action |
| RuVector | 2.4.0; `e3d82b1...`; Apache-2.0 | persistent vector memory | `VectorDB::new/insert/search`, reopen | `ca-adapter-ruvector` | insert/search/reopen and invalid dimensions | without RuVector analogue recall is false |
| RVF crypto | 0.2.0; `9fb3d2c...`; Apache-2.0 | witness-chain integrity | `shake256_256/create_witness_chain/verify_witness_chain` | `ca-adapter-rvf` | genuine chain passes; mutation fails | without RVF evidence is unverifiable |
| Wasserstein DRO-CVaR | Esfahani/Kuhn 2018 DOI above | ambiguity-aware tail-risk ranking | empirical tail mean plus radius sensitivity | `ca-dro` | deterministic ranking and radius monotonicity | empirical-only removes robustness |
| Ruflo 3.56.10 | npm package; coordination only | routing/swarm provenance | CLI route/swarm receipts | external ledger | receipt readback | not counted as product ingredient |
