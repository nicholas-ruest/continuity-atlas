# Core-memory guidance map

Live state: `LIVE_MEMORY_EMPTY_UNSCREENED`; no live preference was inferred. Repository guidance and Nick's explicit override remain authoritative.

| Source / blob | Guidance | Concrete decision | Evidence | Status |
|---|---|---|---|---|
| `AGENTS.md@0b2613ac` | Ruflo coordinates; executor implements | Ruflo ledger separated from code evidence | `evidence/ruflo-execution-ledger.md` | APPLIED |
| build standard `a63fd146` | design before implementation, 20+ real crates | design-only commit precedes Rust commit; 22 linked crates | git history; architecture | APPLIED |
| `CLAUDE.md@01781672` | one writer, typed boundaries, exact receipts | single governed worktree; ports/adapters; source-bound evidence | executor ledger; tests | APPLIED |
| explicit daily override | one distinct new system per date | Continuity Atlas is new; no JitterMap maintenance | daily ledger | APPLIED |
| explicit security guidance | no secrets; validate inputs/paths | environment-free fixtures; safe state path | CLI tests; secret scan | APPLIED |
| evidence guidance | checker is structural only | independent semantic linkage and integration review | `evidence/semantic-review.md` | APPLIED |
