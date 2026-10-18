# MetaHarness execution ledger

MetaHarness was used as an external validation and replay harness. It has advisory
authority only and is not counted as a Continuity Atlas product ingredient or as a
replacement for the locked Rust tests and seven-mode product evaluation.

## Darwin

- Package: `@metaharness/darwin@0.12.0`.
- Command: `npx --yes @metaharness/darwin@0.12.0 bench create /home/ruv/work/continuity-atlas --out /tmp/continuity-atlas-darwin-suite.json`.
- Result: exit 0; one repo-native task; suite hash prefix `96cc5e623971`.
- Verification: `npx --yes @metaharness/darwin@0.12.0 bench verify /tmp/continuity-atlas-darwin-suite.json`.
- Verification result: exit 0; `1 tasks, hash OK (96cc5e623971…)`.
- RuOS transcripts: `run-npx-yes-metaharness-darwin-0-12-0-bench-crea-020757.log` and `run-npx-yes-metaharness-darwin-0-12-0-bench-veri-020802.log`.

Darwin's generated suite is an availability and integrity receipt. It was not treated
as an independent claim that the Rust candidate passed; the locked Rust gates and
source-bound evaluation remain authoritative.

## Flywheel

- Package: `@metaharness/flywheel@0.1.12`, npm shasum `4cf8c5374bcf5082700ddd4f3e96e8ce463f0e0e`, MIT.
- Boundary: imported `runFlywheelGenerations`, `meetsPromotionRule`, `makeSigner`, and `verifyReplayBundle` from the published package.
- Workload: two deterministic, frozen-suite generations over the explicit `ambiguityRadius` lever with holdout `continuity-atlas-holdout`, anchor `continuity-atlas-anchor`, and `dataSource=CONTINUITY_ATLAS_RECEIPT`.
- Result: `generations=2`, `promotions=2`, `milestone=true`, signed replay passed, chain `gen2(ambiguityRadius) → gen1(ambiguityRadius) → gen0(root)`.
- Authority: `none`; the smoke workload does not promote product configuration.
- RuOS transcript: `/home/ruv/.ruos/activity/2026-10-18/run-node-input-type-module-e-import-runflywheelg-020852.log`.

The Flywheel scores are intentionally mechanical smoke values, not Continuity Atlas
benchmark results. Product claims come only from `evidence/evaluation.jsonl`.
