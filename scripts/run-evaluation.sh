#!/usr/bin/env bash
set -euo pipefail
mkdir -p evidence
RUST_LOG=error cargo run --locked -q -p ca-evaluation --bin continuity-atlas-evaluation \
  | tee evidence/evaluation.raw.log \
  | sed -n '/^{/p' \
  | tee evidence/evaluation.jsonl
test "$(wc -l < evidence/evaluation.jsonl)" -eq 7
