---
started: 2026-06-19
updated: 2026-06-19
status: completed
task: Cargo Resolver "1" → "2" Migration
adr: n.a.
agent: claude-code
---

## Goal
Migrate the workspace from Cargo resolver "1" (legacy) to resolver "2" by updating workspace manifests and verifying the
build remains clean.

## Context
- `src/Cargo.toml` — main workspace manifest
- `src/Cargo.docker.toml` — Docker build workspace manifest
- `src/Cargo.lock` — lock file to regenerate after resolver change

## Outcome
Resolver "2" migration completed. Feature unification analysis confirmed no problematic unification exists — all shared
dependencies have identical feature requests across consumers. Two workspace manifests updated (`resolver = "1"` →
`resolver = "2"`). Build and tests verified clean.

**Recommendations for future work:**
- Pin `clap` to consistent version across `work-pulse-service` (4.5.50) and `work-pulse-cli` (4.5.40)
- Align `tokio` version constraints (`work-pulse-core`: 1.48.0, `work-pulse-service`: 1.45.1)
- Investigate `sqlx` pulling MySQL + SQLite backends despite only PostgreSQL being used

## Log
### 2026-06-19
- Analyzed feature unification across all three member crates (`work-pulse-core`, `work-pulse-service`,
  `work-pulse-cli`)
- Confirmed no `default-features = false`, no optional dependencies, no `[features]` sections — no feature divergence
  risk
- Noted `sqlx` pulls unused MySQL/SQLite backends (sqlx-internal behavior, not resolver-related)
- Updated `src/Cargo.toml`: `resolver = "1"` → `resolver = "2"`
- Updated `src/Cargo.docker.toml`: `resolver = "1"` → `resolver = "2"`
- Regenerated `Cargo.lock` via `cargo build --workspace`
- Verified build and tests pass with `cargo build --workspace` and `cargo test --workspace`
- Identified `clap` version divergence (4.5.50 vs 4.5.40) and `tokio` version divergence (1.48.0 vs 1.45.1) as
  maintenance hygiene items
